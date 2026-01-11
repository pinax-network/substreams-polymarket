-- User Position / PNL --
-- Aggregated state for User Token Positions with PNL tracking
-- Reference: https://github.com/Polymarket/polymarket-subgraph/tree/main/pnl-subgraph
--
-- This implementation tracks position changes from:
-- 1. OrderFilled events (exchange trades with exact token_id)
--
-- Events tracked by the Polymarket PNL subgraph:
-- - Merges, Splits, Redemptions, Conversions, OrdersMatched
--
-- Note: Splits, Merges, Redemptions, and Conversions events only have condition_id,
-- not the derived token_id. For those events, use the state_user_condition_position table below.

-- State User Position Table (by Token ID) --
-- Tracks user positions by token_id from exchange trades
-- Use this for PNL tracking where you have the exact token_id
CREATE TABLE IF NOT EXISTS state_user_position (
    -- bar interval --
    timestamp               DateTime('UTC', 0) COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC', 0)) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC', 0)) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- User identity --
    user                    String COMMENT 'User address (hex with 0x prefix)',
    token_id                UInt256 COMMENT 'Token ID (position ID)',

    -- Position changes in window --
    buy_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount bought in window',
    sell_amount             SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount sold in window',
    net_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Net amount change (buy - sell)',

    -- Cost basis tracking --
    buy_cost                SimpleAggregateFunction(sum, Int256) COMMENT 'Total cost of buys in USDC (amount * price)',
    sell_revenue            SimpleAggregateFunction(sum, Int256) COMMENT 'Total revenue from sells in USDC (amount * price)',

    -- Transaction counts --
    buy_count               SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of buy transactions',
    sell_count              SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of sell transactions',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of transactions',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_user                  (user)                  TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_token_id              (token_id)              TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_net_amount            (net_amount)            TYPE minmax         GRANULARITY 1,
    INDEX idx_buy_cost              (buy_cost)              TYPE minmax         GRANULARITY 1,
    INDEX idx_sell_revenue          (sell_revenue)          TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    user, token_id,
    timestamp
)
COMMENT 'User Token Positions from exchange trades, aggregated by interval. Query cumulative values by summing over time.';

-- State User Condition Position Table (by Condition ID) --
-- Tracks user positions by condition_id from splits, merges, redemptions
-- Use this for tracking position changes where token_id cannot be derived in SQL
CREATE TABLE IF NOT EXISTS state_user_condition_position (
    -- bar interval --
    timestamp               DateTime('UTC', 0) COMMENT 'beginning of the bar',
    interval_min            UInt16 DEFAULT 1 COMMENT 'bar interval in minutes (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w)',

    -- timestamp & block number --
    min_timestamp           SimpleAggregateFunction(min, DateTime('UTC', 0)) COMMENT 'first timestamp seen',
    max_timestamp           SimpleAggregateFunction(max, DateTime('UTC', 0)) COMMENT 'last timestamp seen',
    min_block_num           SimpleAggregateFunction(min, UInt32) COMMENT 'first block number seen',
    max_block_num           SimpleAggregateFunction(max, UInt32) COMMENT 'last block number seen',

    -- User identity --
    user                    String COMMENT 'User address (hex with 0x prefix)',
    condition_id            String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',

    -- Position changes in window (valued at 50 cents per token for splits/merges) --
    split_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from splits (buy at 50 cents)',
    merge_amount            SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from merges (sell at 50 cents)',
    redeem_payout           SimpleAggregateFunction(sum, Int256) COMMENT 'Total payout from redemptions',
    convert_amount          SimpleAggregateFunction(sum, Int256) COMMENT 'Total amount from conversions',

    -- Net position change --
    net_amount              SimpleAggregateFunction(sum, Int256) COMMENT 'Net amount change (split - merge)',

    -- Transaction counts by event type --
    split_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of split transactions',
    merge_count             SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of merge transactions',
    redeem_count            SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of redemption transactions',
    convert_count           SimpleAggregateFunction(sum, UInt64) COMMENT 'Number of conversion transactions',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'Total number of transactions',

    -- indexes --
    INDEX idx_timestamp             (timestamp)             TYPE minmax         GRANULARITY 1,
    INDEX idx_user                  (user)                  TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_condition_id          (condition_id)          TYPE bloom_filter   GRANULARITY 1,
    INDEX idx_net_amount            (net_amount)            TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (
    interval_min,
    user, condition_id,
    timestamp
)
COMMENT 'User Condition Positions from splits/merges/redemptions, aggregated by interval.';

-- Materialized View for User Positions from OrderFilled events --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_position
TO state_user_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    user,
    token_id,

    -- Position changes --
    sum(buy_amount) AS buy_amount,
    sum(sell_amount) AS sell_amount,
    sum(net_amount) AS net_amount,

    -- Cost basis tracking --
    sum(buy_cost) AS buy_cost,
    sum(sell_revenue) AS sell_revenue,

    -- Transaction counts --
    sum(is_buy) AS buy_count,
    sum(is_sell) AS sell_count,
    count() AS transactions
FROM (
    -- OrderFilled BUY: maker buys tokens (makerAssetId = 0 means maker pays USDC for tokens)
    -- When makerAssetId = 0: maker is buying, taker_asset_id is the token, taker_amount_filled is token amount
    SELECT
        timestamp,
        block_num,
        maker AS user,
        taker_asset_id AS token_id,
        toInt256(taker_amount_filled) AS buy_amount,
        toInt256(0) AS sell_amount,
        toInt256(taker_amount_filled) AS net_amount,
        -- price = makerAmountFilled / takerAmountFilled (USDC per token)
        -- buy_cost = makerAmountFilled (total USDC spent)
        toInt256(maker_amount_filled) AS buy_cost,
        toInt256(0) AS sell_revenue,
        toUInt64(1) AS is_buy,
        toUInt64(0) AS is_sell
    FROM ctfexchange_order_filled
    WHERE maker_asset_id = 0

    UNION ALL

    -- OrderFilled SELL: maker sells tokens (makerAssetId != 0 means maker is selling tokens for USDC)
    SELECT
        timestamp,
        block_num,
        maker AS user,
        maker_asset_id AS token_id,
        toInt256(0) AS buy_amount,
        toInt256(maker_amount_filled) AS sell_amount,
        -toInt256(maker_amount_filled) AS net_amount,
        toInt256(0) AS buy_cost,
        -- sell_revenue = takerAmountFilled (total USDC received)
        toInt256(taker_amount_filled) AS sell_revenue,
        toUInt64(0) AS is_buy,
        toUInt64(1) AS is_sell
    FROM ctfexchange_order_filled
    WHERE maker_asset_id != 0
) AS combined
GROUP BY
    interval_min,
    user, token_id,
    timestamp;

-- Materialized View for User Condition Positions from Splits, Merges, Redemptions --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_user_condition_position
TO state_user_condition_position
AS
WITH
    -- predefined intervals --
    -- in minutes: 1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w
    [1, 5, 10, 30, 60, 240, 1440, 10080] AS intervals
SELECT
    arrayJoin(intervals) AS interval_min,
    -- floor to the interval in seconds
    toDateTime(intDiv(toUInt32(timestamp), interval_min * 60) * interval_min * 60, 'UTC') AS timestamp,

    -- timestamp & block number --
    min(timestamp) AS min_timestamp,
    max(timestamp) AS max_timestamp,
    min(block_num) AS min_block_num,
    max(block_num) AS max_block_num,

    -- User identity --
    user,
    condition_id,

    -- Position changes --
    sum(split_amount) AS split_amount,
    sum(merge_amount) AS merge_amount,
    sum(redeem_payout) AS redeem_payout,
    sum(convert_amount) AS convert_amount,
    sum(net_amount) AS net_amount,

    -- Transaction counts --
    sum(is_split) AS split_count,
    sum(is_merge) AS merge_count,
    sum(is_redeem) AS redeem_count,
    sum(is_convert) AS convert_count,
    count() AS transactions
FROM (
    -- ConditionalTokens Position Split: user receives tokens at 50 cents each
    -- Splits increase user position
    SELECT
        timestamp,
        block_num,
        stakeholder AS user,
        condition_id,
        toInt256(amount) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(0) AS redeem_payout,
        toInt256(0) AS convert_amount,
        toInt256(amount) AS net_amount,
        toUInt64(1) AS is_split,
        toUInt64(0) AS is_merge,
        toUInt64(0) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM conditionaltokens_position_split

    UNION ALL

    -- ConditionalTokens Positions Merge: user sells tokens at 50 cents each
    -- Merges decrease user position
    SELECT
        timestamp,
        block_num,
        stakeholder AS user,
        condition_id,
        toInt256(0) AS split_amount,
        toInt256(amount) AS merge_amount,
        toInt256(0) AS redeem_payout,
        toInt256(0) AS convert_amount,
        -toInt256(amount) AS net_amount,
        toUInt64(0) AS is_split,
        toUInt64(1) AS is_merge,
        toUInt64(0) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM conditionaltokens_positions_merge

    UNION ALL

    -- ConditionalTokens Payout Redemption: user redeems tokens at payout price
    -- Redemptions decrease user position and realize gains/losses
    SELECT
        timestamp,
        block_num,
        redeemer AS user,
        condition_id,
        toInt256(0) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(payout) AS redeem_payout,
        toInt256(0) AS convert_amount,
        toInt256(0) AS net_amount,  -- payout is in USDC, not tokens
        toUInt64(0) AS is_split,
        toUInt64(0) AS is_merge,
        toUInt64(1) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM conditionaltokens_payout_redemption

    UNION ALL

    -- NegRiskAdapter Position Split
    SELECT
        timestamp,
        block_num,
        stakeholder AS user,
        condition_id,
        toInt256(amount) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(0) AS redeem_payout,
        toInt256(0) AS convert_amount,
        toInt256(amount) AS net_amount,
        toUInt64(1) AS is_split,
        toUInt64(0) AS is_merge,
        toUInt64(0) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM negriskadapter_position_split

    UNION ALL

    -- NegRiskAdapter Positions Merge
    SELECT
        timestamp,
        block_num,
        stakeholder AS user,
        condition_id,
        toInt256(0) AS split_amount,
        toInt256(amount) AS merge_amount,
        toInt256(0) AS redeem_payout,
        toInt256(0) AS convert_amount,
        -toInt256(amount) AS net_amount,
        toUInt64(0) AS is_split,
        toUInt64(1) AS is_merge,
        toUInt64(0) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM negriskadapter_positions_merge

    UNION ALL

    -- NegRiskAdapter Positions Converted
    -- Note: Conversions involve complex token swaps between YES/NO positions
    -- market_id is used instead of condition_id for conversions
    SELECT
        timestamp,
        block_num,
        stakeholder AS user,
        market_id AS condition_id,  -- using market_id as the grouping key
        toInt256(0) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(0) AS redeem_payout,
        toInt256(amount) AS convert_amount,
        toInt256(0) AS net_amount,  -- net is 0 as it's a conversion
        toUInt64(0) AS is_split,
        toUInt64(0) AS is_merge,
        toUInt64(0) AS is_redeem,
        toUInt64(1) AS is_convert
    FROM negriskadapter_positions_converted

    UNION ALL

    -- NegRiskAdapter Payout Redemption
    SELECT
        timestamp,
        block_num,
        redeemer AS user,
        condition_id,
        toInt256(0) AS split_amount,
        toInt256(0) AS merge_amount,
        toInt256(payout) AS redeem_payout,
        toInt256(0) AS convert_amount,
        toInt256(0) AS net_amount,
        toUInt64(0) AS is_split,
        toUInt64(0) AS is_merge,
        toUInt64(1) AS is_redeem,
        toUInt64(0) AS is_convert
    FROM negriskadapter_payout_redemption
) AS combined
GROUP BY
    interval_min,
    user, condition_id,
    timestamp;
