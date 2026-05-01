-- Platform-level aggregation table --
-- Pre-aggregates orderbook, open interest, and fee data across all assets/conditions
-- Replaces the three *_global views with a single materialized table for fast reads
CREATE TABLE IF NOT EXISTS state_platform (
    `timestamp` DateTime('UTC'),
    `interval_min` UInt16,
    -- Orderbook --
    `trades_quantity` SimpleAggregateFunction(sum, UInt64),
    `buys_quantity` SimpleAggregateFunction(sum, UInt64),
    `sells_quantity` SimpleAggregateFunction(sum, UInt64),
    `collateral_volume` SimpleAggregateFunction(sum, Int256),
    `collateral_buy_volume` SimpleAggregateFunction(sum, Int256),
    `collateral_sell_volume` SimpleAggregateFunction(sum, Int256),
    -- Open Interest --
    `split_amount` SimpleAggregateFunction(sum, Int256),
    `merge_amount` SimpleAggregateFunction(sum, Int256),
    `net_open_interest` SimpleAggregateFunction(sum, Int256),
    `split_count` SimpleAggregateFunction(sum, UInt64),
    `merge_count` SimpleAggregateFunction(sum, UInt64),
    `oi_transactions` SimpleAggregateFunction(sum, UInt64),
    -- Fees --
    `total_fee` SimpleAggregateFunction(sum, Int256),
    `fee_count` SimpleAggregateFunction(sum, UInt64)
    -- effective_fee_rate is computed at query time as total_fee / collateral_volume
    -- (the orderbook column above already holds the trade-volume denominator)
) ENGINE = AggregatingMergeTree
ORDER BY (interval_min, timestamp)
SETTINGS index_granularity = 8192;

-- MV: orderbook → platform --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_platform_orderbook
TO state_platform AS
SELECT
    timestamp,
    interval_min,
    sum(trades_quantity) AS trades_quantity,
    sum(buys_quantity) AS buys_quantity,
    sum(sells_quantity) AS sells_quantity,
    sum(collateral_volume) AS collateral_volume,
    sum(collateral_buy_volume) AS collateral_buy_volume,
    sum(collateral_sell_volume) AS collateral_sell_volume,
    toInt256(0) AS split_amount,
    toInt256(0) AS merge_amount,
    toInt256(0) AS net_open_interest,
    toUInt64(0) AS split_count,
    toUInt64(0) AS merge_count,
    toUInt64(0) AS oi_transactions,
    toInt256(0) AS total_fee,
    toUInt64(0) AS fee_count
FROM state_orderbook
GROUP BY interval_min, timestamp;

-- MV: open interest → platform --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_platform_oi
TO state_platform AS
SELECT
    timestamp,
    interval_min,
    toUInt64(0) AS trades_quantity,
    toUInt64(0) AS buys_quantity,
    toUInt64(0) AS sells_quantity,
    toInt256(0) AS collateral_volume,
    toInt256(0) AS collateral_buy_volume,
    toInt256(0) AS collateral_sell_volume,
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(net_open_interest) AS net_open_interest,
    sum(split_count) AS split_count,
    sum(merge_count) AS merge_count,
    sum(transactions) AS oi_transactions,
    toInt256(0) AS total_fee,
    toUInt64(0) AS fee_count
FROM state_open_interest
GROUP BY interval_min, timestamp;

-- MV: fee → platform --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_platform_fee
TO state_platform AS
SELECT
    timestamp,
    interval_min,
    toUInt64(0) AS trades_quantity,
    toUInt64(0) AS buys_quantity,
    toUInt64(0) AS sells_quantity,
    toInt256(0) AS collateral_volume,
    toInt256(0) AS collateral_buy_volume,
    toInt256(0) AS collateral_sell_volume,
    toInt256(0) AS split_amount,
    toInt256(0) AS merge_amount,
    toInt256(0) AS net_open_interest,
    toUInt64(0) AS split_count,
    toUInt64(0) AS merge_count,
    toUInt64(0) AS oi_transactions,
    sum(total_fee) AS total_fee,
    sum(fee_count) AS fee_count
FROM state_fee
GROUP BY interval_min, timestamp;
