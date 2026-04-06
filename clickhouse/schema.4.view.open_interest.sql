-- Open Interest View --
-- Per-condition metrics with condition_id as a dimension
-- Calculated from conditionaltokens_position_split (+) and conditionaltokens_positions_merge (-)
-- Note: USDC has 6 decimals, so divide by 10^6 (1000000.0) to get scaled amounts
CREATE VIEW IF NOT EXISTS open_interest AS
SELECT
    timestamp,
    interval_min,
    parent_collection_id,
    condition_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Aggregate Open Interest --
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(net_open_interest) AS net_open_interest,
    -- Scaled amounts (USDC has 6 decimals, so divide by 10^6) --
    toFloat64(sum(state_open_interest.split_amount)) / 1000000.0 AS scaled_split_amount,
    toFloat64(sum(state_open_interest.merge_amount)) / 1000000.0 AS scaled_merge_amount,
    toFloat64(sum(state_open_interest.net_open_interest)) / 1000000.0 AS scaled_net_open_interest,
    -- Transaction counts --
    sum(split_count) AS split_count,
    sum(merge_count) AS merge_count,
    sum(transactions) AS transactions,
    -- Unique stakeholders --
    uniqMerge(uniq_stakeholders) AS unique_stakeholders
FROM state_open_interest
GROUP BY
    interval_min,
    parent_collection_id,
    condition_id,
    timestamp
ORDER BY
    interval_min,
    parent_collection_id,
    condition_id,
    timestamp;

