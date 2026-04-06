-- Fee View --
-- Per-token fee metrics
CREATE VIEW IF NOT EXISTS fee AS
SELECT
    timestamp,
    interval_min,
    asset_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Fee aggregates --
    sum(total_fee) AS total_fee,
    sum(fee_count) AS fee_count,
    sum(total_volume) AS total_volume,
    sum(trade_count) AS trade_count,
    -- Scaled amounts (USDC has 6 decimals) --
    toFloat64(sum(state_fee.total_fee)) / 1000000.0 AS scaled_total_fee,
    toFloat64(sum(state_fee.total_volume)) / 1000000.0 AS scaled_total_volume,
    -- Effective fee rate (fee / volume) --
    if(sum(state_fee.total_volume) > 0,
        toFloat64(sum(state_fee.total_fee)) / toFloat64(sum(state_fee.total_volume)),
        0) AS effective_fee_rate,
    -- Unique participants --
    uniqMerge(uniq_fee_payers) AS unique_fee_payers
FROM state_fee
GROUP BY
    interval_min,
    asset_id,
    timestamp
ORDER BY
    interval_min,
    asset_id,
    timestamp;

