-- Fee Aggregation --
-- Per-asset fee data from CTFExchange FeeCharged events, plus matching
-- per-asset refunds from FeeModule FeeRefunded events. Trade-volume context
-- lives in state_orderbook (joined when needed); not duplicated here.
--
-- Why two sources:
--   * FeeCharged is the gross fee transferred from the maker/taker proceeds
--     to the operator on every match. Both maker-side and taker-side fees
--     are emitted (when non-zero), so summing every event gives the true
--     gross fee. The previous mv_state_fee read OrderFilled.fee, which
--     captured maker-side rows only after substreams' self-referential
--     filter dropped taker-side OrderFilled events — silently undercounting
--     V1 by ~50% and producing ~$0 on V2 (V2 OrderFilled.fee is left at 0
--     by design, fees come exclusively via FeeCharged).
--   * FeeRefunded carries the refund portion of the Maker Rebates Program
--     on V1. ~83% of gross fees on V1 are refunded back to makers, so
--     net fees retained by Polymarket = total_fee - total_refund.
--     V2 has no FeeRefunded events; total_refund stays 0 there.

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
    asset_id                UInt256 COMMENT 'V1: asset the fee was paid in. BUY-side fees use the outcome token id; SELL-side fees use 0 (USDC sentinel). V2: always 0 (FeeCharged carries no token_id, fees are pUSD-only)',

    -- Fee aggregates --
    total_fee               SimpleAggregateFunction(sum, Int256) COMMENT 'Gross fees collected in window (USDC base units, before maker rebates)',
    total_refund            SimpleAggregateFunction(sum, Int256) COMMENT 'Maker rebates refunded in window (V1 only; 0 on V2). Net fee = total_fee - total_refund',
    fee_count               SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of FeeCharged events in window',

    -- V1/V2 era discriminator --
    -- 1 if this bar contains any V2 FeeCharged (log_address = V2 CTFExchange or
    -- V2 NegRiskCTFExchange); 0 if V1-only. V2 FeeCharged always lands on the
    -- asset_id=0 sentinel row, so v2_present on the asset_id=0 bar tells the
    -- Token API to nullify fee fields for the matching window.
    v2_present              SimpleAggregateFunction(max, UInt8) COMMENT 'Max over the window: 1 if any V2 FeeCharged fell in this bar, 0 otherwise',

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
COMMENT 'Per-asset fee aggregation, sourced from CTFExchange FeeCharged + FeeModule FeeRefunded events.';

-- Drop legacy MVs before recreating, so existing databases pick up the new
-- SELECTs. Idempotent: noop on a fresh database.
DROP TABLE IF EXISTS mv_state_fee;
DROP TABLE IF EXISTS mv_state_fee_refund;

-- Materialized View for gross fees from FeeCharged events --
-- Each match emits one FeeCharged per side that pays a non-zero fee; summing
-- all of them gives the true gross fee. Works identically for V1 and V2.
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_fee
TO state_fee
AS
WITH
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals,
    -- V1/V2 discriminator: V2 CTFExchange and NegRiskCTFExchange addresses.
    if(lower(log_address) IN (
        '0xe111180000d2663c0091e4f400237545b87b996b',
        '0xe2222d279d744050d28e00520010520000310f59'
    ), 1, 0) AS is_v2
SELECT
    arrayJoin(intervals) AS interval_min,
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,
    token_id AS asset_id,
    sum(toInt256(amount)) AS total_fee,
    toInt256(0) AS total_refund,
    count() AS fee_count,
    max(is_v2) AS v2_present
FROM ctfexchange_fee_charged
GROUP BY
    interval_min,
    asset_id,
    timestamp;

-- Materialized View for maker rebates from FeeRefunded events --
-- V1-only. FeeRefunded.token_id matches the corresponding FeeCharged.token_id,
-- so refunds aggregate cleanly on the same (interval_min, asset_id, timestamp)
-- key. V2 has no FeeRefunded source; this MV is dormant on V2 windows.
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_fee_refund
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
    token_id AS asset_id,
    toInt256(0) AS total_fee,
    sum(toInt256(refund)) AS total_refund,
    toUInt64(0) AS fee_count,
    -- FeeModule is V1-only (V2 has no FeeRefunded source); refunds never set v2_present.
    toUInt8(0) AS v2_present
FROM feemodule_fee_refunded
GROUP BY
    interval_min,
    asset_id,
    timestamp;
