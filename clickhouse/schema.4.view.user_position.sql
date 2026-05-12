-- User Position View --
-- Per (interval_min, user, token_id) view over the state_user_position
-- refresh-MV snapshot table. USDC has 6 decimals, so divide raw amounts by
-- 10^6 to get scaled values. Realized PNL is approximated as sell_revenue -
-- buy_cost (net cash flow), not FIFO/LIFO cost-basis PNL.
CREATE VIEW IF NOT EXISTS user_position AS
SELECT
    interval_min,
    user,
    token_id,
    first_trade,
    last_trade,
    buy_amount,
    sell_amount,
    net_amount,
    buy_cost,
    sell_revenue,
    sell_revenue - buy_cost                                                    AS realized_pnl,
    toFloat64(buy_amount)                / 1000000.0                           AS scaled_buy_amount,
    toFloat64(sell_amount)               / 1000000.0                           AS scaled_sell_amount,
    toFloat64(net_amount)                / 1000000.0                           AS scaled_net_amount,
    toFloat64(buy_cost)                  / 1000000.0                           AS scaled_buy_cost,
    toFloat64(sell_revenue)              / 1000000.0                           AS scaled_sell_revenue,
    toFloat64(sell_revenue - buy_cost)   / 1000000.0                           AS scaled_realized_pnl,
    buy_count,
    sell_count,
    transactions
FROM state_user_position FINAL;

-- User Position by User View --
-- Aggregated metrics per (interval_min, user) across all tokens.
CREATE VIEW IF NOT EXISTS user_position_by_user AS
SELECT
    interval_min,
    user,
    min(state_user_position.first_trade)                                                                       AS first_trade,
    max(state_user_position.last_trade)                                                                        AS last_trade,
    sum(state_user_position.buy_amount)                                                                        AS buy_amount,
    sum(state_user_position.sell_amount)                                                                       AS sell_amount,
    sum(state_user_position.net_amount)                                                                        AS net_amount,
    sum(state_user_position.buy_cost)                                                                          AS buy_cost,
    sum(state_user_position.sell_revenue)                                                                      AS sell_revenue,
    sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)                                  AS realized_pnl,
    toFloat64(sum(state_user_position.buy_amount))                / 1000000.0                                  AS scaled_buy_amount,
    toFloat64(sum(state_user_position.sell_amount))               / 1000000.0                                  AS scaled_sell_amount,
    toFloat64(sum(state_user_position.net_amount))                / 1000000.0                                  AS scaled_net_amount,
    toFloat64(sum(state_user_position.buy_cost))                  / 1000000.0                                  AS scaled_buy_cost,
    toFloat64(sum(state_user_position.sell_revenue))              / 1000000.0                                  AS scaled_sell_revenue,
    toFloat64(sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)) / 1000000.0           AS scaled_realized_pnl,
    sum(state_user_position.buy_count)                                                                         AS buy_count,
    sum(state_user_position.sell_count)                                                                        AS sell_count,
    sum(state_user_position.transactions)                                                                      AS transactions
FROM state_user_position FINAL
GROUP BY interval_min, user;

-- User Position by Token View --
-- Aggregated metrics per (interval_min, token_id) across all users.
CREATE VIEW IF NOT EXISTS user_position_by_token AS
SELECT
    interval_min,
    token_id,
    min(state_user_position.first_trade)                                                                       AS first_trade,
    max(state_user_position.last_trade)                                                                        AS last_trade,
    sum(state_user_position.buy_amount)                                                                        AS buy_amount,
    sum(state_user_position.sell_amount)                                                                       AS sell_amount,
    sum(state_user_position.net_amount)                                                                        AS net_amount,
    sum(state_user_position.buy_cost)                                                                          AS buy_cost,
    sum(state_user_position.sell_revenue)                                                                      AS sell_revenue,
    sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)                                  AS realized_pnl,
    toFloat64(sum(state_user_position.buy_amount))                / 1000000.0                                  AS scaled_buy_amount,
    toFloat64(sum(state_user_position.sell_amount))               / 1000000.0                                  AS scaled_sell_amount,
    toFloat64(sum(state_user_position.net_amount))                / 1000000.0                                  AS scaled_net_amount,
    toFloat64(sum(state_user_position.buy_cost))                  / 1000000.0                                  AS scaled_buy_cost,
    toFloat64(sum(state_user_position.sell_revenue))              / 1000000.0                                  AS scaled_sell_revenue,
    toFloat64(sum(state_user_position.sell_revenue) - sum(state_user_position.buy_cost)) / 1000000.0           AS scaled_realized_pnl,
    sum(state_user_position.buy_count)                                                                         AS buy_count,
    sum(state_user_position.sell_count)                                                                        AS sell_count,
    sum(state_user_position.transactions)                                                                      AS transactions
FROM state_user_position FINAL
GROUP BY interval_min, token_id;
