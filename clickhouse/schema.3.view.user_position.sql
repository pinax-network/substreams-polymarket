-- User Position View --
-- Per-user, per-token metrics for PNL tracking
-- Tracks position changes from exchange trades (OrderFilled events)
-- Note: USDC has 6 decimals, so divide by 10^6 (1000000.0) to get scaled amounts
CREATE VIEW IF NOT EXISTS user_position AS
SELECT
    timestamp,
    interval_min,
    user,
    token_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Position changes --
    sum(buy_amount) AS buy_amount,
    sum(sell_amount) AS sell_amount,
    sum(net_amount) AS net_amount,
    -- Cost basis tracking --
    sum(buy_cost) AS buy_cost,
    sum(sell_revenue) AS sell_revenue,
    -- Realized PNL (sell_revenue - proportional buy_cost) approximation --
    sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost) AS realized_pnl,
    -- Scaled amounts (USDC has 6 decimals, so divide by 10^6) --
    toFloat64(sum(state_user_position.buy_amount)) / 1000000.0 AS scaled_buy_amount,
    toFloat64(sum(state_user_position.sell_amount)) / 1000000.0 AS scaled_sell_amount,
    toFloat64(sum(state_user_position.net_amount)) / 1000000.0 AS scaled_net_amount,
    toFloat64(sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_buy_cost,
    toFloat64(sum(state_user_position.sell_revenue)) / 1000000.0 AS scaled_sell_revenue,
    toFloat64(sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_realized_pnl,
    -- Transaction counts --
    sum(buy_count) AS buy_count,
    sum(sell_count) AS sell_count,
    sum(transactions) AS transactions
FROM state_user_position
GROUP BY
    interval_min,
    user,
    token_id,
    timestamp
ORDER BY
    interval_min,
    user,
    token_id,
    timestamp;

-- User Position by User View --
-- Aggregated metrics per user across all tokens
CREATE VIEW IF NOT EXISTS user_position_by_user AS
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
    sum(buy_amount) AS buy_amount,
    sum(sell_amount) AS sell_amount,
    sum(net_amount) AS net_amount,
    -- Cost basis tracking --
    sum(buy_cost) AS buy_cost,
    sum(sell_revenue) AS sell_revenue,
    sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost) AS realized_pnl,
    -- Scaled amounts --
    toFloat64(sum(state_user_position.buy_amount)) / 1000000.0 AS scaled_buy_amount,
    toFloat64(sum(state_user_position.sell_amount)) / 1000000.0 AS scaled_sell_amount,
    toFloat64(sum(state_user_position.net_amount)) / 1000000.0 AS scaled_net_amount,
    toFloat64(sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_buy_cost,
    toFloat64(sum(state_user_position.sell_revenue)) / 1000000.0 AS scaled_sell_revenue,
    toFloat64(sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_realized_pnl,
    -- Transaction counts --
    sum(buy_count) AS buy_count,
    sum(sell_count) AS sell_count,
    sum(transactions) AS transactions
FROM state_user_position
GROUP BY
    interval_min,
    user,
    timestamp
ORDER BY
    interval_min,
    user,
    timestamp;

-- User Position by Token View --
-- Aggregated metrics per token across all users
CREATE VIEW IF NOT EXISTS user_position_by_token AS
SELECT
    timestamp,
    interval_min,
    token_id,
    -- timestamp & block number --
    min(min_timestamp) AS min_timestamp,
    max(max_timestamp) AS max_timestamp,
    min(min_block_num) AS min_block_num,
    max(max_block_num) AS max_block_num,
    -- Position changes --
    sum(buy_amount) AS buy_amount,
    sum(sell_amount) AS sell_amount,
    sum(net_amount) AS net_amount,
    -- Cost basis tracking --
    sum(state_user_position.buy_cost) AS buy_cost,
    sum(state_user_position.sell_revenue) AS sell_revenue,
    sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost) AS realized_pnl,
    -- Scaled amounts --
    toFloat64(sum(state_user_position.buy_amount)) / 1000000.0 AS scaled_buy_amount,
    toFloat64(sum(state_user_position.sell_amount)) / 1000000.0 AS scaled_sell_amount,
    toFloat64(sum(state_user_position.net_amount)) / 1000000.0 AS scaled_net_amount,
    toFloat64(sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_buy_cost,
    toFloat64(sum(state_user_position.sell_revenue)) / 1000000.0 AS scaled_sell_revenue,
    toFloat64(sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)) / 1000000.0 AS scaled_realized_pnl,
    -- Transaction counts --
    sum(buy_count) AS buy_count,
    sum(sell_count) AS sell_count,
    sum(transactions) AS transactions
FROM state_user_position
GROUP BY
    interval_min,
    token_id,
    timestamp
ORDER BY
    interval_min,
    token_id,
    timestamp;
