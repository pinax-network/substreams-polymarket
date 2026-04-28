-- Open Interest --
-- Aggregated state for Market and Global Open Interest
-- Calculated from conditionaltokens_position_split (+) and conditionaltokens_positions_merge (-)
-- Reference: https://github.com/Polymarket/polymarket-subgraph/tree/main/oi-subgraph

-- State Open Interest Table --
CREATE TABLE IF NOT EXISTS state_open_interest (
    -- bar interval --
    timestamp               DateTime('UTC') COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC')) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC')) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- Market identity --
    parent_collection_id    String COMMENT 'Parent collection ID (bytes32 as hex with 0x prefix). 0x0...0 represents global',
    condition_id            String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',

    -- Aggregate Open Interest --
    split_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total split amount (increases OI)',
    merge_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total merge amount (decreases OI)',
    net_open_interest       SimpleAggregateFunction(sum, Int256) COMMENT 'Net open interest change (split - merge)',

    -- Transaction counts --
    split_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of split transactions',
    merge_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of merge transactions',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of transactions',

    -- Unique stakeholders --
    uniq_stakeholders       AggregateFunction(uniq, String) COMMENT 'Unique stakeholder addresses in the window',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_parent_collection_id  (parent_collection_id)  TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_condition_id          (condition_id)          TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_net_open_interest     (net_open_interest)     TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    parent_collection_id, condition_id,
    timestamp
)
COMMENT 'Open Interest for Polymarket conditions, aggregated by interval. Global OI is where parent_collection_id=0x0...0';

-- Materialized View for Open Interest from Position Splits --
-- Splits: increase OI --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_open_interest_split
TO state_open_interest
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

    -- Market identity --
    parent_collection_id,
    condition_id,

    -- Aggregate Open Interest --
    sum(toInt256(amount)) AS split_amount,
    toInt256(0) AS merge_amount,
    sum(toInt256(amount)) AS net_open_interest,

    -- Transaction counts --
    count() AS split_count,
    toUInt64(0) AS merge_count,
    count() AS transactions,

    -- Unique stakeholders --
    uniqState(stakeholder) AS uniq_stakeholders
FROM conditionaltokens_position_split
-- Drop non-canonical-decimals collateral splits (e.g. an 18-decimal WMATIC split
-- inflates aggregates by 10^12 because / 1e6 is hard-coded downstream). Cap is
-- well above any realistic 6-decimal Polymarket split (~$10B post-scale). See
-- pinax-network/token-api#489.
WHERE amount < toUInt256('10000000000000000')
GROUP BY
    interval_min,
    parent_collection_id, condition_id,
    timestamp;

-- Materialized View for Open Interest from Position Merges --
-- Merges: decrease OI --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_open_interest_merge
TO state_open_interest
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

    -- Market identity --
    parent_collection_id,
    condition_id,

    -- Aggregate Open Interest --
    toInt256(0) AS split_amount,
    sum(toInt256(amount)) AS merge_amount,
    -sum(toInt256(amount)) AS net_open_interest,

    -- Transaction counts --
    toUInt64(0) AS split_count,
    count() AS merge_count,
    count() AS transactions,

    -- Unique stakeholders --
    uniqState(stakeholder) AS uniq_stakeholders
FROM conditionaltokens_positions_merge
-- See split MV above; same cap on amount to suppress non-USDC-decimals merges.
WHERE amount < toUInt256('10000000000000000')
GROUP BY
    interval_min,
    parent_collection_id, condition_id,
    timestamp;
