-- User Leaderboard --
-- Per (interval_min, user) snapshot rolling up state_user_position across
-- token_ids and adding unrealized PNL via state_latest_price. Refreshed
-- hourly via an APPEND-mode refresh MV.
--
-- Engine: ReplacingMergeTree(refresh_time) — the substreams sink rewrites to
-- ReplicatedReplacingMergeTree on a cluster. TTL bounds storage to ~3 hourly
-- snapshots pre-merge. Consumers must read with FINAL.

CREATE TABLE IF NOT EXISTS state_user (
    refresh_time             DateTime('UTC'),
    interval_min             UInt32 COMMENT '0=all-time, 60=1h, 1440=1d, 10080=1w, 43200=30d',
    user                     String,
    buy_cost                 Float64,
    sell_revenue             Float64,
    buy_count                UInt64,
    sell_count               UInt64,
    transactions             UInt64,
    realized_pnl             Float64,
    unrealized_pnl           Float64,
    total_pnl                Float64,
    first_trade              DateTime('UTC'),
    last_trade               DateTime('UTC')
) ENGINE = ReplacingMergeTree(refresh_time)
ORDER BY (interval_min, user)
TTL refresh_time + INTERVAL 3 HOUR;

-- state_user_position already encodes the interval snapshots (0/60/1440/10080/
-- 43200), so the leaderboard rolls up token_ids → users for the same intervals.
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_refresh_state_user
REFRESH EVERY 1 HOUR APPEND
TO state_user
AS
WITH per_user_token AS (
    SELECT
        interval_min,
        user,
        token_id,
        buy_amount,
        sell_amount,
        net_amount,
        buy_cost,
        sell_revenue,
        buy_count,
        sell_count,
        transactions,
        first_trade,
        last_trade
    FROM state_user_position FINAL
    WHERE interval_min IN (0, 60, 1440, 10080, 43200)
)
SELECT
    now()                                                                                AS refresh_time,
    p.interval_min                                                                       AS interval_min,
    p.user                                                                               AS user,
    toFloat64(sum(p.buy_cost)) / 1e6                                                     AS buy_cost,
    toFloat64(sum(p.sell_revenue)) / 1e6                                                 AS sell_revenue,
    sum(p.buy_count)                                                                     AS buy_count,
    sum(p.sell_count)                                                                    AS sell_count,
    sum(p.transactions)                                                                  AS transactions,
    toFloat64(sum(p.sell_revenue) - sum(p.buy_cost)) / 1e6                               AS realized_pnl,
    sumIf(toFloat64(p.net_amount) / 1e6 * coalesce(lp.close, 0), p.net_amount > 0)       AS unrealized_pnl,
    toFloat64(sum(p.sell_revenue) - sum(p.buy_cost)) / 1e6
        + sumIf(toFloat64(p.net_amount) / 1e6 * coalesce(lp.close, 0), p.net_amount > 0) AS total_pnl,
    min(p.first_trade)                                                                   AS first_trade,
    max(p.last_trade)                                                                    AS last_trade
FROM per_user_token p
LEFT JOIN state_latest_price lp FINAL ON lp.asset_id = p.token_id
GROUP BY p.interval_min, p.user
SETTINGS max_execution_time = 600;
