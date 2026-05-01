-- Fee Aggregation --
-- Aggregated fee data per asset, sourced from CTFExchange FeeCharged events.
-- Trade volume context lives in state_orderbook (joined when needed); we don't
-- duplicate it here.
--
-- The previous mv_state_fee read OrderFilled.fee, which only carried the maker
-- side of each match (each match emits TWO FeeCharged events but only ONE
-- OrderFilled with the maker's fee). On V1 this silently undercounted by ~50%;
-- on V2 it produced ~$0 because V2 charges fees per-batch via FeeCharged with
-- OrderFilled.fee left at 0 by design.

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
    asset_id                String COMMENT 'V1: outcome token id paying the fee, "0" for unattributed protocol-level fees. V2: always "0" (FeeCharged carries no token_id, fees are pUSD-only)',

    -- Fee aggregates --
    total_fee               SimpleAggregateFunction(sum, Int256) COMMENT 'Total fees collected in window (USDC base units)',
    fee_count               SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of FeeCharged events in window',

    -- Unique participants --
    uniq_fee_payers         AggregateFunction(uniq, String) COMMENT 'Unique tx_from addresses paying fees in window',

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
COMMENT 'Per-asset fee aggregation, sourced from CTFExchange FeeCharged events.';

-- Drop the legacy MV before recreating, so existing databases pick up the
-- new SELECT. Idempotent: noop on a fresh database.
DROP TABLE IF EXISTS mv_state_fee;

-- Materialized View for Fee Aggregation from FeeCharged events --
-- Each match emits one FeeCharged per side (maker and taker); summing all of
-- them gives the true total fee. Works identically for V1 and V2 contracts.
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_fee
TO state_fee
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
    toString(token_id) AS asset_id,
    sum(toInt256(amount)) AS total_fee,
    count() AS fee_count,
    uniqState(tx_from) AS uniq_fee_payers
FROM ctfexchange_fee_charged
GROUP BY
    interval_min,
    asset_id,
    timestamp;
