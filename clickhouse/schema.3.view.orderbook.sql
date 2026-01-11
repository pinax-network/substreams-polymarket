-- OrderBook View --
-- Per-asset metrics with asset_id as a dimension
-- Note: USDC has 6 decimals, so divide by 10^6 (1000000.0) to get scaled volumes
CREATE VIEW IF NOT EXISTS orderbook AS
SELECT
    timestamp,
    interval_min,
    asset_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Trading quantities --
    sum(trades_quantity) AS trades_quantity,
    sum(buys_quantity) AS buys_quantity,
    sum(sells_quantity) AS sells_quantity,
    -- Volume in USDC base units --
    sum(collateral_volume) AS collateral_volume,
    sum(collateral_buy_volume) AS collateral_buy_volume,
    sum(collateral_sell_volume) AS collateral_sell_volume,
    -- Scaled volumes (USDC has 6 decimals, so divide by 10^6) --
    toFloat64(sum(state_orderbook.collateral_volume)) / 1000000.0 AS scaled_collateral_volume,
    toFloat64(sum(state_orderbook.collateral_buy_volume)) / 1000000.0 AS scaled_collateral_buy_volume,
    toFloat64(sum(state_orderbook.collateral_sell_volume)) / 1000000.0 AS scaled_collateral_sell_volume,
    -- Unique participants (merge the aggregate states) --
    uniqMerge(uniq_makers) AS unique_makers,
    uniqMerge(uniq_takers) AS unique_takers
FROM state_orderbook
GROUP BY
    interval_min,
    asset_id,
    timestamp
ORDER BY
    interval_min,
    asset_id,
    timestamp;

-- OrdersMatchedGlobal View --
-- Global metrics aggregated across all order books
-- This is a convenience view that aggregates all asset_ids
-- Can be used to get global statistics without GROUP BY asset_id
-- Note: USDC has 6 decimals, so divide by 10^6 (1000000.0) to get scaled volumes
CREATE VIEW IF NOT EXISTS orderbook_global AS
SELECT
    timestamp,
    interval_min,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Global trading quantities --
    sum(trades_quantity) AS trades_quantity,
    sum(buys_quantity) AS buys_quantity,
    sum(sells_quantity) AS sells_quantity,
    -- Global volume in USDC base units --
    sum(collateral_volume) AS collateral_volume,
    sum(collateral_buy_volume) AS collateral_buy_volume,
    sum(collateral_sell_volume) AS collateral_sell_volume,
    -- Scaled volumes (USDC has 6 decimals, so divide by 10^6) --
    toFloat64(sum(state_orderbook.collateral_volume)) / 1000000.0 AS scaled_collateral_volume,
    toFloat64(sum(state_orderbook.collateral_buy_volume)) / 1000000.0 AS scaled_collateral_buy_volume,
    toFloat64(sum(state_orderbook.collateral_sell_volume)) / 1000000.0 AS scaled_collateral_sell_volume
FROM state_orderbook
GROUP BY
    interval_min,
    timestamp
ORDER BY
    interval_min,
    timestamp;
