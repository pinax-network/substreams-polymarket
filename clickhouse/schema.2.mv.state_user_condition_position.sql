-- State User Condition Position Table (by Condition ID) --
-- Tracks user positions by condition_id from splits, merges, redemptions
-- Use this for tracking position changes where token_id cannot be derived in SQL
CREATE TABLE IF NOT EXISTS state_user_condition_position (
    -- bar interval --
    timestamp               DateTime('UTC') COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC')) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC')) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- User identity --
    user                    String COMMENT 'User address (hex with 0x prefix)',
    condition_id            String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',

    -- Position changes in window (valued at 50 cents per token for splits/merges) --
    split_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from splits (buy at 50 cents)',
    merge_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from merges (sell at 50 cents)',
    redeem_payout           SimpleAggregateFunction(sum, Int256) COMMENT 'Total payout from redemptions',
    convert_amount          SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from conversions',

    -- Net position change --
    net_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Net amount change (split - merge)',

    -- Transaction counts by event type --
    split_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of split transactions',
    merge_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of merge transactions',
    redeem_count            SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of redemption transactions',
    convert_count           SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of conversion transactions',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of transactions',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_user                  (user)                  TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_condition_id          (condition_id)          TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_net_amount            (net_amount)            TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    user, condition_id,
    timestamp
)
COMMENT 'User Condition Positions from splits/merges/redemptions, aggregated by interval.';

-- Materialized View for User Condition Positions from ConditionalTokens Position Split --
-- Splits increase user position --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_ct_split
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    stakeholder AS user,
    condition_id,

    -- Position changes --
    sum(toInt256(amount)) AS split_amount,
    toInt256(0) AS merge_amount,
    toInt256(0) AS redeem_payout,
    toInt256(0) AS convert_amount,
    sum(toInt256(amount)) AS net_amount,

    -- Transaction counts --
    count() AS split_count,
    toUInt64(0) AS merge_count,
    toUInt64(0) AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM conditionaltokens_position_split
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from ConditionalTokens Positions Merge --
-- Merges decrease user position --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_ct_merge
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    stakeholder AS user,
    condition_id,

    -- Position changes --
    toInt256(0) AS split_amount,
    sum(toInt256(amount)) AS merge_amount,
    toInt256(0) AS redeem_payout,
    toInt256(0) AS convert_amount,
    -sum(toInt256(amount)) AS net_amount,

    -- Transaction counts --
    toUInt64(0) AS split_count,
    count() AS merge_count,
    toUInt64(0) AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM conditionaltokens_positions_merge
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from ConditionalTokens Payout Redemption --
-- Redemptions decrease user position and realize gains/losses --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_ct_redeem
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    redeemer AS user,
    condition_id,

    -- Position changes --
    toInt256(0) AS split_amount,
    toInt256(0) AS merge_amount,
    sum(toInt256(payout)) AS redeem_payout,
    toInt256(0) AS convert_amount,
    toInt256(0) AS net_amount,  -- payout is in USDC, not tokens

    -- Transaction counts --
    toUInt64(0) AS split_count,
    toUInt64(0) AS merge_count,
    count() AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM conditionaltokens_payout_redemption
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from NegRiskAdapter Position Split --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_nr_split
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    stakeholder AS user,
    condition_id,

    -- Position changes --
    sum(toInt256(amount)) AS split_amount,
    toInt256(0) AS merge_amount,
    toInt256(0) AS redeem_payout,
    toInt256(0) AS convert_amount,
    sum(toInt256(amount)) AS net_amount,

    -- Transaction counts --
    count() AS split_count,
    toUInt64(0) AS merge_count,
    toUInt64(0) AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM negriskadapter_position_split
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from NegRiskAdapter Positions Merge --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_nr_merge
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    stakeholder AS user,
    condition_id,

    -- Position changes --
    toInt256(0) AS split_amount,
    sum(toInt256(amount)) AS merge_amount,
    toInt256(0) AS redeem_payout,
    toInt256(0) AS convert_amount,
    -sum(toInt256(amount)) AS net_amount,

    -- Transaction counts --
    toUInt64(0) AS split_count,
    count() AS merge_count,
    toUInt64(0) AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM negriskadapter_positions_merge
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from NegRiskAdapter Positions Converted --
-- Note: Conversions involve complex token swaps between YES/NO positions
-- market_id is used instead of condition_id for conversions since this event
-- operates at the market level (multi-question markets). The market_id identifies
-- the neg-risk market containing multiple conditions/questions.
-- WARNING: Do not join with other sources on condition_id when convert_count > 0
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_nr_convert
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    stakeholder AS user,
    market_id AS condition_id,  -- market_id identifies the neg-risk market (different from condition_id)

    -- Position changes --
    toInt256(0) AS split_amount,
    toInt256(0) AS merge_amount,
    toInt256(0) AS redeem_payout,
    sum(toInt256(amount)) AS convert_amount,
    toInt256(0) AS net_amount,  -- net is 0 as it's a conversion between positions

    -- Transaction counts --
    toUInt64(0) AS split_count,
    toUInt64(0) AS merge_count,
    toUInt64(0) AS redeem_count,
    count() AS convert_count,
    count() AS transactions
FROM negriskadapter_positions_converted
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;

-- Materialized View for User Condition Positions from NegRiskAdapter Payout Redemption --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position_nr_redeem
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    redeemer AS user,
    condition_id,

    -- Position changes --
    toInt256(0) AS split_amount,
    toInt256(0) AS merge_amount,
    sum(toInt256(payout)) AS redeem_payout,
    toInt256(0) AS convert_amount,
    toInt256(0) AS net_amount,  -- payout is in USDC, not tokens

    -- Transaction counts --
    toUInt64(0) AS split_count,
    toUInt64(0) AS merge_count,
    count() AS redeem_count,
    toUInt64(0) AS convert_count,
    count() AS transactions
FROM negriskadapter_payout_redemption
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;
