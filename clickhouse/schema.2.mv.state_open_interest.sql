-- Open Interest --
-- Aggregated state for Market and Global Open Interest
-- Calculated from conditionaltokens_position_split (+) and conditionaltokens_positions_merge (-)
-- Reference: https://github.com/Polymarket/polymarket-subgraph/tree/main/oi-subgraph

-- State Open Interest Table --
CREATE TABLE IF NOT EXISTS state_open_interest (
    -- bar interval --
    timestamp               DateTime('UTC', 0) COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC', 0)) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC', 0)) COMMENT 'last timestamp seen',
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

-- Materialized View for Open Interest from Position Splits and Merges --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_open_interest
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
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(net_amount) AS net_open_interest,

    -- Transaction counts --
    sum(is_split) AS split_count,
    sum(is_merge) AS merge_count,
    count() AS transactions,

    -- Unique stakeholders --
    uniqState(stakeholder) AS uniq_stakeholders
-- Splits: increase OI --
FROM (
    SELECT
        timestamp,
        block_num,
        parent_collection_id,
        condition_id,
        stakeholder,
        toInt256(amount) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(amount) AS net_amount,
        toUInt64(1) AS is_split,
        toUInt64(0) AS is_merge
    FROM conditionaltokens_position_split

    UNION ALL

    -- Merges: decrease OI --
    SELECT
        timestamp,
        block_num,
        parent_collection_id,
        condition_id,
        stakeholder,
        toInt256(0) AS split_amount,
        toInt256(amount) AS merge_amount,
        -toInt256(amount) AS net_amount,
        toUInt64(0) AS is_split,
        toUInt64(1) AS is_merge
    FROM conditionaltokens_positions_merge
) AS combined
GROUP BY
    interval_min,
    parent_collection_id, condition_id,
    timestamp;
