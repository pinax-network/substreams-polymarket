-- User Condition Position View --
-- Per-user, per-condition metrics
-- Tracks position changes from splits, merges, redemptions, and conversions
-- Note: USDC has 6 decimals, so divide by 10^6 (1000000.0) to get scaled amounts
CREATE VIEW IF NOT EXISTS user_condition_position AS
SELECT
    timestamp,
    interval_min,
    user,
    condition_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Position changes --
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(redeem_payout) AS redeem_payout,
    sum(convert_amount) AS convert_amount,
    sum(net_amount) AS net_amount,
    -- Scaled amounts (USDC has 6 decimals, so divide by 10^6) --
    toFloat64(sum(split_amount)) / 1000000.0 AS scaled_split_amount,
    toFloat64(sum(merge_amount)) / 1000000.0 AS scaled_merge_amount,
    toFloat64(sum(redeem_payout)) / 1000000.0 AS scaled_redeem_payout,
    toFloat64(sum(convert_amount)) / 1000000.0 AS scaled_convert_amount,
    toFloat64(sum(net_amount)) / 1000000.0 AS scaled_net_amount,
    -- Transaction counts --
    sum(split_count) AS split_count,
    sum(merge_count) AS merge_count,
    sum(redeem_count) AS redeem_count,
    sum(convert_count) AS convert_count,
    sum(transactions) AS transactions
FROM state_user_condition_position
GROUP BY
    interval_min,
    user,
    condition_id,
    timestamp
ORDER BY
    interval_min,
    user,
    condition_id,
    timestamp;

-- User Condition Position by User View --
-- Aggregated metrics per user across all conditions
CREATE VIEW IF NOT EXISTS user_condition_position_by_user AS
SELECT
    timestamp,
    interval_min,
    user,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Position changes --
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(redeem_payout) AS redeem_payout,
    sum(convert_amount) AS convert_amount,
    sum(net_amount) AS net_amount,
    -- Scaled amounts --
    toFloat64(sum(split_amount)) / 1000000.0 AS scaled_split_amount,
    toFloat64(sum(merge_amount)) / 1000000.0 AS scaled_merge_amount,
    toFloat64(sum(redeem_payout)) / 1000000.0 AS scaled_redeem_payout,
    toFloat64(sum(convert_amount)) / 1000000.0 AS scaled_convert_amount,
    toFloat64(sum(net_amount)) / 1000000.0 AS scaled_net_amount,
    -- Transaction counts --
    sum(split_count) AS split_count,
    sum(merge_count) AS merge_count,
    sum(redeem_count) AS redeem_count,
    sum(convert_count) AS convert_count,
    sum(transactions) AS transactions
FROM state_user_condition_position
GROUP BY
    interval_min,
    user,
    timestamp
ORDER BY
    interval_min,
    user,
    timestamp;

-- User Condition Position by Condition View --
-- Aggregated metrics per condition across all users
CREATE VIEW IF NOT EXISTS user_condition_position_by_condition AS
SELECT
    timestamp,
    interval_min,
    condition_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Position changes --
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(redeem_payout) AS redeem_payout,
    sum(convert_amount) AS convert_amount,
    sum(net_amount) AS net_amount,
    -- Scaled amounts --
    toFloat64(sum(split_amount)) / 1000000.0 AS scaled_split_amount,
    toFloat64(sum(merge_amount)) / 1000000.0 AS scaled_merge_amount,
    toFloat64(sum(redeem_payout)) / 1000000.0 AS scaled_redeem_payout,
    toFloat64(sum(convert_amount)) / 1000000.0 AS scaled_convert_amount,
    toFloat64(sum(net_amount)) / 1000000.0 AS scaled_net_amount,
    -- Transaction counts --
    sum(split_count) AS split_count,
    sum(merge_count) AS merge_count,
    sum(redeem_count) AS redeem_count,
    sum(convert_count) AS convert_count,
    sum(transactions) AS transactions
FROM state_user_condition_position
GROUP BY
    interval_min,
    condition_id,
    timestamp
ORDER BY
    interval_min,
    condition_id,
    timestamp;
