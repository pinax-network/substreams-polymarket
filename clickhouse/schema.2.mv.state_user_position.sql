-- User Position / PNL --
-- Aggregated state for User Token Positions with PNL tracking
-- Reference: https://github.com/Polymarket/polymarket-subgraph/tree/main/pnl-subgraph
--
-- This implementation tracks position changes from:
-- 1. OrderFilled events (exchange trades with exact token_id)
--
-- Events tracked by the Polymarket PNL subgraph:
-- - Merges, Splits, Redemptions, Conversions, OrdersMatched
--
-- Note: Splits, Merges, Redemptions, and Conversions events only have condition_id,
-- not the derived token_id. For those events, use the state_user_condition_position table below.

-- State User Position Table (by Token ID) --
-- Tracks user positions by token_id from exchange trades
-- Use this for PNL tracking where you have the exact token_id
CREATE TABLE IF NOT EXISTS state_user_position (
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
    INDEX idx_net_amount            (net_amount)            TYPE minmax         GRANULARITY 1,
    INDEX idx_buy_cost              (buy_cost)              TYPE minmax         GRANULARITY 1,
    INDEX idx_sell_revenue          (sell_revenue)          TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    user, token_id,
    timestamp
)
COMMENT 'User Token Positions from exchange trades, aggregated by interval. Query cumulative values by summing over time.';

-- Materialized View for User Positions from OrderFilled BUY events --
-- OrderFilled BUY: maker buys tokens (makerAssetId = 0 means maker pays USDC for tokens) --
-- When makerAssetId = 0: maker is buying, taker_asset_id is the token, taker_amount_filled is token amount --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_position_buy
TO state_user_position
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
    maker AS user,
    taker_asset_id AS token_id,

    -- Position changes --
    sum(toInt256(taker_amount_filled)) AS buy_amount,
    toInt256(0) AS sell_amount,
    sum(toInt256(taker_amount_filled)) AS net_amount,

    -- Cost basis tracking --
    -- price = makerAmountFilled / takerAmountFilled (USDC per token)
    -- buy_cost = makerAmountFilled (total USDC spent)
    sum(toInt256(maker_amount_filled)) AS buy_cost,
    toInt256(0) AS sell_revenue,

    -- Transaction counts --
    count() AS buy_count,
    toUInt64(0) AS sell_count,
    count() AS transactions
FROM ctfexchange_order_filled
WHERE maker_asset_id = 0
GROUP BY
    interval_min,
    user, token_id,
    timestamp;

-- Materialized View for User Positions from OrderFilled SELL events --
-- OrderFilled SELL: maker sells tokens (makerAssetId != 0 means maker is selling tokens for USDC)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_position_sell
TO state_user_position
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
    maker AS user,
    maker_asset_id AS token_id,

    -- Position changes --
    toInt256(0) AS buy_amount,
    sum(toInt256(maker_amount_filled)) AS sell_amount,
    -sum(toInt256(maker_amount_filled)) AS net_amount,

    -- Cost basis tracking --
    toInt256(0) AS buy_cost,
    -- sell_revenue = takerAmountFilled (total USDC received)
    sum(toInt256(taker_amount_filled)) AS sell_revenue,

    -- Transaction counts --
    toUInt64(0) AS buy_count,
    count() AS sell_count,
    count() AS transactions
FROM ctfexchange_order_filled
WHERE maker_asset_id != 0
GROUP BY
    interval_min,
    user, token_id,
    timestamp;
