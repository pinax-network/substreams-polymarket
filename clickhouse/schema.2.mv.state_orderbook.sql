-- OrderBook --
-- Aggregated state for OrderBook and Global OrdersMatched
-- Calculated from ctfexchange_orders_matched events
-- Reference: https://github.com/Polymarket/polymarket-subgraph/tree/main/orderbook-subgraph

-- State OrderBook Table --
-- Aggregates trading data per asset_id (Token ID) across time intervals
-- Global metrics can be computed by omitting asset_id from GROUP BY
CREATE TABLE IF NOT EXISTS state_orderbook (
    -- bar interval --
    timestamp               DateTime(0, 'UTC') COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime(0, 'UTC')) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime(0, 'UTC')) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- OrderBook identity --
    -- Uses asset_id (Token) as the smallest aggregating market
    asset_id                String COMMENT 'Asset ID (Token ID as string)',

    -- Trading quantities --
    trades_quantity         SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of trades of any kind against this order book',
    buys_quantity           SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of purchases of shares from this order book',
    sells_quantity          SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of sales of shares to this order book',

    -- Volume in USDC base units (raw amounts) --
    collateral_volume       SimpleAggregateFunction(sum, Int256) COMMENT 'Market volume in terms of the underlying collateral value (USDC base units)',
    collateral_buy_volume   SimpleAggregateFunction(sum, Int256) COMMENT 'Volume of share purchases in USDC base units',
    collateral_sell_volume  SimpleAggregateFunction(sum, Int256) COMMENT 'Volume of share sales in USDC base units',

    -- Unique participants --
    uniq_makers             AggregateFunction(uniq, String) COMMENT 'Unique maker addresses in the window',
    uniq_takers             AggregateFunction(uniq, String) COMMENT 'Unique taker addresses in the window',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_asset_id              (asset_id)              TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_collateral_volume     (collateral_volume)     TYPE minmax         GRANULARITY 1,
    INDEX idx_trades_quantity       (trades_quantity)       TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    asset_id,
    timestamp
)
COMMENT 'OrderBook for Polymarket assets, aggregated by interval. Global metrics can be computed by omitting asset_id from GROUP BY';

-- Materialized View for OrderBook from OrdersMatched events --
-- OrdersMatched events contain:
-- - taker_order_hash: Taker order hash
-- - taker_order_maker: Taker order maker address
-- - maker_asset_id: Maker asset token ID (UInt256)
-- - taker_asset_id: Taker asset token ID (UInt256)
-- - maker_amount_filled: Maker amount filled (UInt256)
-- - taker_amount_filled: Taker amount filled (UInt256)
--
-- Trade classification:
-- - If taker_asset_id is 0 (USDC), this is a BUY (taker pays USDC to buy shares)
-- - If maker_asset_id is 0 (USDC), this is a SELL (taker sells shares for USDC)
-- The collateral token (USDC) has token ID 0
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_orderbook
TO state_orderbook
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w --
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

    -- OrderBook identity (asset_id) --
    asset_id,

    -- Trading quantities --
    sum(trades_count) AS trades_quantity,
    sum(is_buy) AS buys_quantity,
    sum(is_sell) AS sells_quantity,

    -- Volume in USDC base units --
    sum(collateral_amount) AS collateral_volume,
    sum(buy_collateral) AS collateral_buy_volume,
    sum(sell_collateral) AS collateral_sell_volume,

    -- Unique participants --
    uniqState(maker) AS uniq_makers,
    uniqState(taker) AS uniq_takers

-- Each OrdersMatched event represents a trade --
-- We determine if it's a buy or sell based on which asset is USDC (token ID 0) --
-- and we extract the non-USDC asset as the asset_id --
FROM (
    SELECT
        timestamp,
        block_num,
        -- The asset being traded is the non-USDC token --
        -- If taker_asset_id is 0 (USDC), the traded asset is maker_asset_id (BUY) --
        -- If maker_asset_id is 0 (USDC), the traded asset is taker_asset_id (SELL) --
        toString(
            if(taker_asset_id = 0, maker_asset_id, taker_asset_id)
        ) AS asset_id,
        -- Trade classification --
        toUInt64(1) AS trades_count,
        -- BUY: taker pays USDC (taker_asset_id = 0) to receive shares
        toUInt64(if(taker_asset_id = 0, 1, 0)) AS is_buy,
        -- SELL: taker receives USDC (maker_asset_id = 0) by selling shares
        toUInt64(if(maker_asset_id = 0, 1, 0)) AS is_sell,
        -- Collateral amount is the USDC amount in the trade
        -- If taker pays USDC (buy), collateral = taker_amount_filled
        -- If taker receives USDC (sell), collateral = maker_amount_filled
        toInt256(
            if(taker_asset_id = 0, taker_amount_filled, maker_amount_filled)
        ) AS collateral_amount,
        -- Buy collateral
        toInt256(
            if(taker_asset_id = 0, taker_amount_filled, 0)
        ) AS buy_collateral,
        -- Sell collateral
        toInt256(
            if(maker_asset_id = 0, maker_amount_filled, 0)
        ) AS sell_collateral,
        -- Participants
        taker_order_maker AS maker,
        tx_from AS taker
    FROM ctfexchange_orders_matched
    -- Filter out trades where neither asset is USDC (edge cases)
    WHERE taker_asset_id = 0 OR maker_asset_id = 0
) AS trades
GROUP BY
    interval_min,
    asset_id,
    timestamp;
