-- User Condition Position View --
-- Per (interval_min, user, condition_id) view over the snapshot table.
-- USDC has 6 decimals, so divide raw amounts by 10^6 to get scaled values.
CREATE VIEW IF NOT EXISTS user_condition_position AS
SELECT
    interval_min,
    user,
    condition_id,
    first_trade,
    last_trade,
    split_amount,
    merge_amount,
    redeem_payout,
    convert_amount,
    net_amount,
    toFloat64(split_amount)   / 1000000.0                                      AS scaled_split_amount,
    toFloat64(merge_amount)   / 1000000.0                                      AS scaled_merge_amount,
    toFloat64(redeem_payout)  / 1000000.0                                      AS scaled_redeem_payout,
    toFloat64(convert_amount) / 1000000.0                                      AS scaled_convert_amount,
    toFloat64(net_amount)     / 1000000.0                                      AS scaled_net_amount,
    split_count,
    merge_count,
    redeem_count,
    convert_count,
    transactions
FROM state_user_condition_position FINAL;

-- User Condition Position by User View --
-- Aggregated metrics per (interval_min, user) across all conditions.
CREATE VIEW IF NOT EXISTS user_condition_position_by_user AS
SELECT
    interval_min,
    user,
    min(state_user_condition_position.first_trade)                                  AS first_trade,
    max(state_user_condition_position.last_trade)                                   AS last_trade,
    sum(state_user_condition_position.split_amount)                                 AS split_amount,
    sum(state_user_condition_position.merge_amount)                                 AS merge_amount,
    sum(state_user_condition_position.redeem_payout)                                AS redeem_payout,
    sum(state_user_condition_position.convert_amount)                               AS convert_amount,
    sum(state_user_condition_position.net_amount)                                   AS net_amount,
    toFloat64(sum(state_user_condition_position.split_amount))   / 1000000.0        AS scaled_split_amount,
    toFloat64(sum(state_user_condition_position.merge_amount))   / 1000000.0        AS scaled_merge_amount,
    toFloat64(sum(state_user_condition_position.redeem_payout))  / 1000000.0        AS scaled_redeem_payout,
    toFloat64(sum(state_user_condition_position.convert_amount)) / 1000000.0        AS scaled_convert_amount,
    toFloat64(sum(state_user_condition_position.net_amount))     / 1000000.0        AS scaled_net_amount,
    sum(state_user_condition_position.split_count)                                  AS split_count,
    sum(state_user_condition_position.merge_count)                                  AS merge_count,
    sum(state_user_condition_position.redeem_count)                                 AS redeem_count,
    sum(state_user_condition_position.convert_count)                                AS convert_count,
    sum(state_user_condition_position.transactions)                                 AS transactions
FROM state_user_condition_position FINAL
GROUP BY interval_min, user;

-- User Condition Position by Condition View --
-- Aggregated metrics per (interval_min, condition_id) across all users.
CREATE VIEW IF NOT EXISTS user_condition_position_by_condition AS
SELECT
    interval_min,
    condition_id,
    min(state_user_condition_position.first_trade)                                  AS first_trade,
    max(state_user_condition_position.last_trade)                                   AS last_trade,
    sum(state_user_condition_position.split_amount)                                 AS split_amount,
    sum(state_user_condition_position.merge_amount)                                 AS merge_amount,
    sum(state_user_condition_position.redeem_payout)                                AS redeem_payout,
    sum(state_user_condition_position.convert_amount)                               AS convert_amount,
    sum(state_user_condition_position.net_amount)                                   AS net_amount,
    toFloat64(sum(state_user_condition_position.split_amount))   / 1000000.0        AS scaled_split_amount,
    toFloat64(sum(state_user_condition_position.merge_amount))   / 1000000.0        AS scaled_merge_amount,
    toFloat64(sum(state_user_condition_position.redeem_payout))  / 1000000.0        AS scaled_redeem_payout,
    toFloat64(sum(state_user_condition_position.convert_amount)) / 1000000.0        AS scaled_convert_amount,
    toFloat64(sum(state_user_condition_position.net_amount))     / 1000000.0        AS scaled_net_amount,
    sum(state_user_condition_position.split_count)                                  AS split_count,
    sum(state_user_condition_position.merge_count)                                  AS merge_count,
    sum(state_user_condition_position.redeem_count)                                 AS redeem_count,
    sum(state_user_condition_position.convert_count)                                AS convert_count,
    sum(state_user_condition_position.transactions)                                 AS transactions
FROM state_user_condition_position FINAL
GROUP BY interval_min, condition_id;
