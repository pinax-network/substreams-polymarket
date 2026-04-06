-- User Leaderboard --
-- Pre-computed user trading stats with PNL across lookback windows
-- Refreshed hourly via REFRESH materialized view
-- Reads from state_user_position (trade data) and state_latest_price (unrealized PNL)

CREATE TABLE IF NOT EXISTS state_user (
    interval_min             UInt32 COMMENT '0=all-time, 60=1h, 1440=1d, 10080=1w, 43200=30d',
    user                     String,
    buy_cost                 Float64,
    sell_revenue             Float64,
    buy_count                UInt64,
    sell_count               UInt64,
    transactions             UInt64,
    realized_pnl             Float64,
    unrealized_pnl           Float64,
    total_pnl                Float64,
    first_trade              DateTime('UTC'),
    last_trade               DateTime('UTC')
) ENGINE = MergeTree
ORDER BY (interval_min, user);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_refresh_state_user
REFRESH EVERY 1 HOUR
TO state_user
AS
WITH base AS (
    SELECT
        interval_min,
        agg.user AS user,
        toFloat64(sum(agg.buy_cost)) / 1e6 AS buy_cost,
        toFloat64(sum(agg.sell_revenue)) / 1e6 AS sell_revenue,
        sum(agg.buy_count) AS buy_count,
        sum(agg.sell_count) AS sell_count,
        sum(agg.transactions) AS transactions,
        toFloat64(sum(agg.sell_revenue) - sum(agg.buy_cost)) / 1e6 AS realized_pnl,
        sumIf(toFloat64(agg.net_amount) / 1e6 * coalesce(lp.close, 0), agg.net_amount > 0) AS unrealized_pnl,
        toFloat64(sum(agg.sell_revenue) - sum(agg.buy_cost)) / 1e6
            + sumIf(toFloat64(agg.net_amount) / 1e6 * coalesce(lp.close, 0), agg.net_amount > 0) AS total_pnl,
        min(agg.min_timestamp) AS first_trade,
        max(agg.max_timestamp) AS last_trade
    FROM (
        SELECT
            tp.interval_min AS interval_min,
            user, token_id,
            sum(buy_cost) AS buy_cost, sum(sell_revenue) AS sell_revenue,
            sum(buy_count) AS buy_count, sum(sell_count) AS sell_count, sum(transactions) AS transactions,
            sum(net_amount) AS net_amount,
            min(min_timestamp) AS min_timestamp, max(max_timestamp) AS max_timestamp
        FROM state_user_position
        CROSS JOIN (
            SELECT 0 AS interval_min, 10080 AS source_iv, toDateTime('1970-01-01', 'UTC') AS since
            UNION ALL SELECT 43200, 1440, now() - INTERVAL 30 DAY
            UNION ALL SELECT 10080, 1440, now() - INTERVAL 7 DAY
            UNION ALL SELECT 1440, 60, now() - INTERVAL 1 DAY
            UNION ALL SELECT 60, 60, now() - INTERVAL 1 HOUR
        ) tp
        WHERE state_user_position.interval_min = tp.source_iv AND timestamp >= tp.since
        GROUP BY tp.interval_min, user, token_id
    ) agg
    LEFT JOIN state_latest_price lp FINAL ON lp.asset_id = toString(agg.token_id)
    GROUP BY interval_min, agg.user
)
SELECT * FROM base;
