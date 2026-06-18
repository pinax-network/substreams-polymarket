-- Market Position --
-- Same data as state_user_position with token_id-first ordering, optimized
-- for "all users on a given market" queries. Derived from state_user_position
-- FINAL so both tables share one source of truth.

CREATE TABLE IF NOT EXISTS state_market_position (
    refresh_time            DateTime('UTC'),
    interval_min            UInt32 COMMENT '0=all-time, 60=1h, 1440=1d, 10080=1w, 43200=30d',
    user                    String COMMENT 'User address (hex with 0x prefix)',
    token_id                UInt256 COMMENT 'Token ID (position ID)',
    buy_amount              Int256 COMMENT 'Total token amount bought in window',
    sell_amount             Int256 COMMENT 'Total token amount sold in window',
    net_amount              Int256 COMMENT 'Net amount change (buy - sell)',
    buy_cost                Int256 COMMENT 'Total USDC spent on buys (raw, 6 decimals)',
    sell_revenue            Int256 COMMENT 'Total USDC received on sells (raw, 6 decimals)',
    buy_count               UInt64 COMMENT 'Number of buy fills',
    sell_count              UInt64 COMMENT 'Number of sell fills',
    transactions            UInt64 COMMENT 'buy_count + sell_count',
    first_trade             DateTime('UTC') COMMENT 'Earliest fill timestamp in the window',
    last_trade              DateTime('UTC') COMMENT 'Latest fill timestamp in the window'
) ENGINE = ReplacingMergeTree(refresh_time)
ORDER BY (interval_min, token_id, user)
TTL refresh_time + INTERVAL 3 HOUR
COMMENT 'Market positions — same data as state_user_position with token_id-first ordering. Read with FINAL.';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_refresh_state_market_position
REFRESH EVERY 1 HOUR OFFSET 5 MINUTE APPEND
TO state_market_position
AS
SELECT
    now()           AS refresh_time,
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
SETTINGS max_threads = 4, max_insert_threads = 4, max_execution_time = 1200;
