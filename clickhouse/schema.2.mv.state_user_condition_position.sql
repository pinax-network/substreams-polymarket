-- User Condition Position --
-- Per (interval_min, user, condition_id) snapshot of split/merge/redeem/convert
-- activity from ConditionalTokens and NegRiskAdapter. Refreshed hourly via an
-- APPEND-mode refresh MV, replacing the previous 7 continuous
-- AggregatingMergeTree MVs (CT split/merge/redeem + NR split/merge/redeem/
-- convert). Same snapshot pattern as state_user_position.
--
-- The non-canonical-decimals filter (amount < 10^16, see
-- pinax-network/token-api#489) is preserved per source so WMATIC-style 18-decimal
-- collateral doesn't inflate aggregates by 10^12.
--
-- NegRisk conversions use market_id as the position key because this event
-- operates at the multi-question market level (different from condition_id).
-- This is the only source where condition_id semantically holds market_id; see
-- the inline comment on the NR convert leg.

CREATE TABLE IF NOT EXISTS state_user_condition_position (
    refresh_time            DateTime('UTC'),
    interval_min            UInt32 COMMENT '0=all-time, 60=1h, 1440=1d, 10080=1w, 43200=30d',
    user                    String COMMENT 'User address (hex with 0x prefix)',
    condition_id            String COMMENT 'Condition ID (bytes32 as hex with 0x prefix), for NR convert this carries market_id',
    split_amount            Int256 COMMENT 'Total amount from splits',
    merge_amount            Int256 COMMENT 'Total amount from merges',
    redeem_payout           Int256 COMMENT 'Total USDC payout from redemptions',
    convert_amount          Int256 COMMENT 'Total amount from NegRisk conversions',
    net_amount              Int256 COMMENT 'split - merge (redemptions and conversions are net-zero on amount)',
    split_count             UInt64 COMMENT 'Number of split events',
    merge_count             UInt64 COMMENT 'Number of merge events',
    redeem_count            UInt64 COMMENT 'Number of redemption events',
    convert_count           UInt64 COMMENT 'Number of NegRisk conversion events',
    transactions            UInt64 COMMENT 'Total events in the window',
    first_trade             DateTime('UTC') COMMENT 'Earliest event timestamp in the window',
    last_trade              DateTime('UTC') COMMENT 'Latest event timestamp in the window'
) ENGINE = ReplacingMergeTree(refresh_time)
ORDER BY (interval_min, user, condition_id)
TTL refresh_time + INTERVAL 3 HOUR
COMMENT 'User condition positions snapshot per refresh window. Read with FINAL.';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_refresh_state_user_condition_position
REFRESH EVERY 1 HOUR APPEND
TO state_user_condition_position
AS
WITH
    time_periods AS (
        SELECT 0 AS interval_min, toDateTime('1970-01-01', 'UTC') AS since
        UNION ALL SELECT 43200, now() - INTERVAL 30 DAY
        UNION ALL SELECT 10080, now() - INTERVAL 7 DAY
        UNION ALL SELECT 1440,  now() - INTERVAL 1 DAY
        UNION ALL SELECT 60,    now() - INTERVAL 1 HOUR
    ),
    events AS (
        -- ConditionalTokens position split (CT split) --
        SELECT
            timestamp,
            stakeholder                  AS user,
            condition_id                 AS condition_id,
            toInt256(amount)             AS split_amount,
            toInt256(0)                  AS merge_amount,
            toInt256(0)                  AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            toInt256(amount)             AS net_amount,
            toUInt64(1)                  AS split_count,
            toUInt64(0)                  AS merge_count,
            toUInt64(0)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM conditionaltokens_position_split
        WHERE amount < toUInt256('10000000000000000')
        UNION ALL
        -- ConditionalTokens positions merge (CT merge) --
        SELECT
            timestamp,
            stakeholder                  AS user,
            condition_id                 AS condition_id,
            toInt256(0)                  AS split_amount,
            toInt256(amount)             AS merge_amount,
            toInt256(0)                  AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            -toInt256(amount)            AS net_amount,
            toUInt64(0)                  AS split_count,
            toUInt64(1)                  AS merge_count,
            toUInt64(0)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM conditionaltokens_positions_merge
        WHERE amount < toUInt256('10000000000000000')
        UNION ALL
        -- ConditionalTokens payout redemption (CT redeem) --
        SELECT
            timestamp,
            redeemer                     AS user,
            condition_id                 AS condition_id,
            toInt256(0)                  AS split_amount,
            toInt256(0)                  AS merge_amount,
            toInt256(payout)             AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            toInt256(0)                  AS net_amount,
            toUInt64(0)                  AS split_count,
            toUInt64(0)                  AS merge_count,
            toUInt64(1)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM conditionaltokens_payout_redemption
        WHERE payout < toUInt256('10000000000000000')
        UNION ALL
        -- NegRiskAdapter position split (NR split) --
        SELECT
            timestamp,
            stakeholder                  AS user,
            condition_id                 AS condition_id,
            toInt256(amount)             AS split_amount,
            toInt256(0)                  AS merge_amount,
            toInt256(0)                  AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            toInt256(amount)             AS net_amount,
            toUInt64(1)                  AS split_count,
            toUInt64(0)                  AS merge_count,
            toUInt64(0)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM negriskadapter_position_split
        UNION ALL
        -- NegRiskAdapter positions merge (NR merge) --
        SELECT
            timestamp,
            stakeholder                  AS user,
            condition_id                 AS condition_id,
            toInt256(0)                  AS split_amount,
            toInt256(amount)             AS merge_amount,
            toInt256(0)                  AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            -toInt256(amount)            AS net_amount,
            toUInt64(0)                  AS split_count,
            toUInt64(1)                  AS merge_count,
            toUInt64(0)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM negriskadapter_positions_merge
        UNION ALL
        -- NegRiskAdapter payout redemption (NR redeem) --
        SELECT
            timestamp,
            redeemer                     AS user,
            condition_id                 AS condition_id,
            toInt256(0)                  AS split_amount,
            toInt256(0)                  AS merge_amount,
            toInt256(payout)             AS redeem_payout,
            toInt256(0)                  AS convert_amount,
            toInt256(0)                  AS net_amount,
            toUInt64(0)                  AS split_count,
            toUInt64(0)                  AS merge_count,
            toUInt64(1)                  AS redeem_count,
            toUInt64(0)                  AS convert_count
        FROM negriskadapter_payout_redemption
        UNION ALL
        -- NegRiskAdapter positions converted (NR convert) --
        -- market_id (not condition_id) is the position key for conversions;
        -- this event operates at the multi-question market level. Consumers
        -- joining condition_id from other sources must filter out rows where
        -- convert_count > 0.
        SELECT
            timestamp,
            stakeholder                  AS user,
            market_id                    AS condition_id,
            toInt256(0)                  AS split_amount,
            toInt256(0)                  AS merge_amount,
            toInt256(0)                  AS redeem_payout,
            toInt256(amount)             AS convert_amount,
            toInt256(0)                  AS net_amount,
            toUInt64(0)                  AS split_count,
            toUInt64(0)                  AS merge_count,
            toUInt64(0)                  AS redeem_count,
            toUInt64(1)                  AS convert_count
        FROM negriskadapter_positions_converted
    )
SELECT
    now()                                       AS refresh_time,
    tp.interval_min                             AS interval_min,
    e.user                                      AS user,
    e.condition_id                              AS condition_id,
    sum(e.split_amount)                         AS split_amount,
    sum(e.merge_amount)                         AS merge_amount,
    sum(e.redeem_payout)                        AS redeem_payout,
    sum(e.convert_amount)                       AS convert_amount,
    sum(e.net_amount)                           AS net_amount,
    sum(e.split_count)                          AS split_count,
    sum(e.merge_count)                          AS merge_count,
    sum(e.redeem_count)                         AS redeem_count,
    sum(e.convert_count)                        AS convert_count,
    sum(e.split_count) + sum(e.merge_count)
        + sum(e.redeem_count) + sum(e.convert_count) AS transactions,
    min(e.timestamp)                            AS first_trade,
    max(e.timestamp)                            AS last_trade
FROM events e
CROSS JOIN time_periods tp
WHERE e.timestamp >= tp.since
GROUP BY tp.interval_min, e.user, e.condition_id
SETTINGS max_execution_time = 600;
