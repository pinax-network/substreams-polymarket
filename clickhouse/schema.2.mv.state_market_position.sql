-- Market Position --
-- Same data as state_user_position but with token_id-first ordering
-- Optimized for per-market queries: "all users on a given token"
-- state_user_position ORDER BY: (interval_min, user, token_id, timestamp)
-- state_market_position ORDER BY: (interval_min, token_id, user, timestamp)

-- State Market Position Table (by Token ID, then User) --
CREATE TABLE IF NOT EXISTS state_market_position (
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
    token_id                UInt256 COMMENT 'Token ID (position ID)',

    -- Position changes in window --
    buy_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount bought in window',
    sell_amount             SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount sold in window',
    net_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Net amount change (buy - sell)',

    -- Cost basis tracking --
    buy_cost                SimpleAggregateFunction(sum, Int256) COMMENT 'Total cost of buys in USDC (amount * price)',
    sell_revenue            SimpleAggregateFunction(sum, Int256) COMMENT 'Total revenue from sells in USDC (amount * price)',

    -- Transaction counts --
    buy_count               SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of buy transactions',
    sell_count              SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of sell transactions',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of transactions',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_user                  (user)                  TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_token_id              (token_id)              TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_net_amount            (net_amount)            TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    token_id, user,
    timestamp
)
COMMENT 'Market Positions — same data as state_user_position with token_id-first ordering for per-market queries.';

-- Materialized View for Market Positions from OrderFilled BUY events --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_market_position_buy
TO state_market_position
AS
WITH
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,
    maker AS user,
    taker_asset_id AS token_id,
    sum(toInt256(taker_amount_filled)) AS buy_amount,
    toInt256(0) AS sell_amount,
    sum(toInt256(taker_amount_filled)) AS net_amount,
    sum(toInt256(maker_amount_filled)) AS buy_cost,
    toInt256(0) AS sell_revenue,
    count() AS buy_count,
    toUInt64(0) AS sell_count,
    count() AS transactions
FROM ctfexchange_order_filled
WHERE maker_asset_id = 0
GROUP BY
    interval_min,
    user, token_id,
    timestamp;

-- Materialized View for Market Positions from OrderFilled SELL events --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_market_position_sell
TO state_market_position
AS
WITH
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,
    maker AS user,
    maker_asset_id AS token_id,
    toInt256(0) AS buy_amount,
    sum(toInt256(maker_amount_filled)) AS sell_amount,
    -sum(toInt256(maker_amount_filled)) AS net_amount,
    toInt256(0) AS buy_cost,
    sum(toInt256(taker_amount_filled)) AS sell_revenue,
    toUInt64(0) AS buy_count,
    count() AS sell_count,
    count() AS transactions
FROM ctfexchange_order_filled
WHERE maker_asset_id != 0
GROUP BY
    interval_min,
    user, token_id,
    timestamp;
