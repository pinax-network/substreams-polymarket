-- Fee View --
-- Per-asset fee metrics. Trade-volume context (denominator for effective fee
-- rate) is sourced from state_orderbook on the same (interval_min, asset_id,
-- timestamp) key — no need to duplicate it in state_fee.
--
-- Both state_fee and state_orderbook are AggregatingMergeTree with multiple
-- rows per key between merges; we pre-aggregate each in a CTE before joining
-- so the join doesn't multiply row counts.
CREATE VIEW IF NOT EXISTS fee AS
WITH
    fees AS (
        SELECT
            interval_min,
            asset_id,
            timestamp,
            min(min_timestamp)              AS min_timestamp,
            max(max_timestamp)              AS max_timestamp,
            min(min_block_num)              AS min_block_num,
            max(max_block_num)              AS max_block_num,
            sum(total_fee)                  AS total_fee,
            sum(fee_count)                  AS fee_count,
            uniqMerge(uniq_fee_payers)      AS unique_fee_payers
        FROM state_fee
        GROUP BY interval_min, asset_id, timestamp
    ),
    volumes AS (
        SELECT
            interval_min,
            asset_id,
            timestamp,
            sum(collateral_volume)          AS collateral_volume,
            sum(trades_quantity)            AS trade_count
        FROM state_orderbook
        GROUP BY interval_min, asset_id, timestamp
    )
SELECT
    f.timestamp                                                                  AS timestamp,
    f.interval_min                                                               AS interval_min,
    f.asset_id                                                                   AS asset_id,
    f.min_timestamp                                                              AS min_timestamp,
    f.max_timestamp                                                              AS max_timestamp,
    f.min_block_num                                                              AS min_block_num,
    f.max_block_num                                                              AS max_block_num,
    f.total_fee                                                                  AS total_fee,
    f.fee_count                                                                  AS fee_count,
    v.collateral_volume                                                          AS collateral_volume,
    v.trade_count                                                                AS trade_count,
    toFloat64(f.total_fee) / 1000000.0                                           AS scaled_total_fee,
    toFloat64(v.collateral_volume) / 1000000.0                                   AS scaled_collateral_volume,
    if(v.collateral_volume > 0,
        toFloat64(f.total_fee) / toFloat64(v.collateral_volume),
        0)                                                                       AS effective_fee_rate,
    f.unique_fee_payers                                                          AS unique_fee_payers
FROM fees AS f
LEFT JOIN volumes AS v
    ON v.interval_min = f.interval_min
   AND v.asset_id     = f.asset_id
   AND v.timestamp    = f.timestamp
ORDER BY
    f.interval_min,
    f.asset_id,
    f.timestamp;
