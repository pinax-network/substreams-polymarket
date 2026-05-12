-- User Position / PNL --
-- Per (interval_min, user, token_id) snapshot of trading position derived
-- from CTFExchange OrderFilled events. Refreshed hourly via an APPEND-mode
-- refresh MV, replacing the previous 4 continuous AggregatingMergeTree MVs
-- (buy / sell / taker_buy / taker_sell), which kept per-bar aggregation rows
-- across 8 intervals × 2 sides for every fill — the dominant storage cost on
-- prod.
--
-- The new shape drops the `timestamp` dimension entirely; each refresh writes
-- one snapshot per interval. Reads are simple GROUP BY (or FINAL) — no
-- AggregateFunction merge cost.
--
-- Engine: ReplacingMergeTree(refresh_time) — substreams sink rewrites to
-- ReplicatedReplacingMergeTree on a cluster. TTL bounds storage to ~3 hourly
-- snapshots pre-merge. Consumers must read with FINAL.

CREATE TABLE IF NOT EXISTS state_user_position (
    refresh_time            DateTime('UTC'),
    interval_min            UInt32 COMMENT '0=all-time, 60=1h, 1440=1d, 10080=1w, 43200=30d',
    user                    String COMMENT 'User address (hex with 0x prefix)',
    token_id                UInt256 COMMENT 'Token ID (position ID)',
    buy_amount              Int256 COMMENT 'Total token amount bought in window',
    sell_amount             Int256 COMMENT 'Total token amount sold in window',
    net_amount              Int256 COMMENT 'Net amount change (buy - sell)',
    buy_cost                Int256 COMMENT 'Total USDC spent on buys (raw, 6 decimals)',
    sell_revenue            Int256 COMMENT 'Total USDC received on sells (raw, 6 decimals)',
    buy_count               UInt64 COMMENT 'Number of buy fills',
    sell_count              UInt64 COMMENT 'Number of sell fills',
    transactions            UInt64 COMMENT 'buy_count + sell_count',
    first_trade             DateTime('UTC') COMMENT 'Earliest fill timestamp in the window',
    last_trade              DateTime('UTC') COMMENT 'Latest fill timestamp in the window'
) ENGINE = ReplacingMergeTree(refresh_time)
ORDER BY (interval_min, user, token_id)
TTL refresh_time + INTERVAL 3 HOUR
COMMENT 'User token positions snapshot per refresh window. Read with FINAL.';

-- Materialized View: hourly refresh from raw ctfexchange_order_filled.
-- For each fill we emit two logical legs (maker, taker); the buy/sell
-- direction of each leg is determined by which side carries USDC
-- (asset_id = 0). The traded token is the non-USDC side. After the substreams
-- mapper translates V2 (side, token_id) → V1 (maker_asset_id, taker_asset_id),
-- this logic applies uniformly to both eras.
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_refresh_state_user_position
REFRESH EVERY 1 HOUR APPEND
TO state_user_position
AS
WITH
    time_periods AS (
        SELECT 0 AS interval_min, toDateTime('1970-01-01', 'UTC') AS since
        UNION ALL SELECT 43200, now() - INTERVAL 30 DAY
        UNION ALL SELECT 10080, now() - INTERVAL 7 DAY
        UNION ALL SELECT 1440,  now() - INTERVAL 1 DAY
        UNION ALL SELECT 60,    now() - INTERVAL 1 HOUR
    ),
    sides AS (
        -- Maker leg --
        SELECT
            timestamp                                                   AS timestamp,
            maker                                                        AS user,
            if(maker_asset_id = 0, taker_asset_id, maker_asset_id)       AS token_id,
            if(maker_asset_id = 0, toInt256(taker_amount_filled), toInt256(0)) AS buy_amount,
            if(maker_asset_id = 0, toInt256(0), toInt256(maker_amount_filled)) AS sell_amount,
            if(maker_asset_id = 0, toInt256(maker_amount_filled), toInt256(0)) AS buy_cost,
            if(maker_asset_id = 0, toInt256(0), toInt256(taker_amount_filled)) AS sell_revenue,
            toUInt64(if(maker_asset_id = 0, 1, 0))                       AS buy_count,
            toUInt64(if(maker_asset_id = 0, 0, 1))                       AS sell_count
        FROM ctfexchange_order_filled
        UNION ALL
        -- Taker leg (mirror; maker buying USDC means taker is selling tokens) --
        SELECT
            timestamp                                                   AS timestamp,
            taker                                                        AS user,
            if(maker_asset_id = 0, taker_asset_id, maker_asset_id)       AS token_id,
            if(maker_asset_id != 0, toInt256(maker_amount_filled), toInt256(0)) AS buy_amount,
            if(maker_asset_id != 0, toInt256(0), toInt256(taker_amount_filled)) AS sell_amount,
            if(maker_asset_id != 0, toInt256(taker_amount_filled), toInt256(0)) AS buy_cost,
            if(maker_asset_id != 0, toInt256(0), toInt256(maker_amount_filled)) AS sell_revenue,
            toUInt64(if(maker_asset_id != 0, 1, 0))                      AS buy_count,
            toUInt64(if(maker_asset_id != 0, 0, 1))                      AS sell_count
        FROM ctfexchange_order_filled
    )
SELECT
    now()                                       AS refresh_time,
    tp.interval_min                             AS interval_min,
    s.user                                      AS user,
    s.token_id                                  AS token_id,
    sum(s.buy_amount)                           AS buy_amount,
    sum(s.sell_amount)                          AS sell_amount,
    sum(s.buy_amount) - sum(s.sell_amount)      AS net_amount,
    sum(s.buy_cost)                             AS buy_cost,
    sum(s.sell_revenue)                         AS sell_revenue,
    sum(s.buy_count)                            AS buy_count,
    sum(s.sell_count)                           AS sell_count,
    sum(s.buy_count) + sum(s.sell_count)        AS transactions,
    min(s.timestamp)                            AS first_trade,
    max(s.timestamp)                            AS last_trade
FROM sides s
CROSS JOIN time_periods tp
WHERE s.timestamp >= tp.since
GROUP BY tp.interval_min, s.user, s.token_id
SETTINGS max_execution_time = 600;
