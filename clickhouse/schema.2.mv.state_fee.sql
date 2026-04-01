-- Fee Aggregation --
-- Aggregated fee data per market token from OrderFilled events
-- Tracks total fees collected, trade counts, and volume for context

-- State Fee Table --
CREATE TABLE IF NOT EXISTS state_fee (
    -- bar interval --
    timestamp               DateTime('UTC') COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC')) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC')) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- Fee identity --
    asset_id                String COMMENT 'Asset ID (Token ID as string) — the token being traded',

    -- Fee aggregates --
    total_fee               SimpleAggregateFunction(sum, Int256) COMMENT 'Total fees collected in window (USDC base units)',
    fee_count               SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of trades with non-zero fee',
    total_volume            SimpleAggregateFunction(sum, Int256) COMMENT 'Total collateral volume for context (USDC base units)',
    trade_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of trades',

    -- Unique participants --
    uniq_fee_payers         AggregateFunction(uniq, String) COMMENT 'Unique taker addresses who paid fees',

    -- indexes --
    INDEX idx_timestamp         (timestamp)         TYPE minmax         GRANULARITY 1,
    INDEX idx_asset_id          (asset_id)          TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_total_fee         (total_fee)         TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    asset_id,
    timestamp
)
COMMENT 'Fee aggregation per market token, from OrderFilled events.';

-- Materialized View for Fee Aggregation from OrderFilled events --
-- Fee is charged on the taker side of every trade
-- asset_id is the non-USDC token being traded
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_fee
TO state_fee
AS
WITH
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals,
    toString(if(taker_asset_id = 0, maker_asset_id, taker_asset_id)) AS asset_id,
    toInt256(if(taker_asset_id = 0, taker_amount_filled, maker_amount_filled)) AS collateral_amount
SELECT
    arrayJoin(intervals) AS interval_min,
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,
    asset_id,
    sum(toInt256(fee)) AS total_fee,
    countIf(fee > 0) AS fee_count,
    sum(collateral_amount) AS total_volume,
    count() AS trade_count,
    uniqStateIf(taker, fee > 0) AS uniq_fee_payers
FROM ctfexchange_order_filled
WHERE taker_asset_id = 0 OR maker_asset_id = 0
GROUP BY
    interval_min,
    asset_id,
    timestamp;
