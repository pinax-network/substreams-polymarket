-- Latest daily close price per asset --
-- Materialized from state_orderbook (interval_min=1440) for fast price lookups
-- Used by positions endpoints to avoid scanning the full daily orderbook on every query
CREATE TABLE IF NOT EXISTS state_latest_price (
    `asset_id` String,
    `timestamp` DateTime('UTC'),
    `close` Float64
) ENGINE = ReplacingMergeTree(timestamp)
ORDER BY asset_id
SETTINGS index_granularity = 8192;

-- MV: daily orderbook → latest price --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_latest_price
TO state_latest_price AS
SELECT
    asset_id,
    timestamp,
    argMaxMerge(close) AS close
FROM state_orderbook
WHERE interval_min = 1440
GROUP BY asset_id, timestamp;
