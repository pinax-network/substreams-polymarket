-- pUSD collateral flow from CollateralToken Wrapped/Unwrapped events --
CREATE TABLE IF NOT EXISTS state_collateral_flow
(
    interval_min            UInt64 COMMENT 'Minute interval timestamp',
    asset                   String COMMENT 'Underlying collateral asset address',
    user                    String COMMENT 'Recipient/user address',
    wrapped_amount          Int256 COMMENT 'pUSD minted from wrapped collateral',
    unwrapped_amount        Int256 COMMENT 'pUSD burned back to collateral',
    net_amount              Int256 COMMENT 'Net pUSD mint/burn flow',
    timestamp               UInt64 COMMENT 'Event timestamp',
    block_num               UInt64 COMMENT 'Block number'
)
ENGINE = SummingMergeTree((wrapped_amount, unwrapped_amount, net_amount))
ORDER BY (interval_min, asset, user, timestamp)
COMMENT 'Minute-level pUSD wrapped/unwrapped collateral flows';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_collateral_flow_wrapped
TO state_collateral_flow AS
SELECT
    minute AS interval_min,
    asset,
    to_address AS user,
    toInt256(amount) AS wrapped_amount,
    toInt256(0) AS unwrapped_amount,
    toInt256(amount) AS net_amount,
    timestamp,
    block_num
FROM collateral_token_wrapped;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_state_collateral_flow_unwrapped
TO state_collateral_flow AS
SELECT
    minute AS interval_min,
    asset,
    to_address AS user,
    toInt256(0) AS wrapped_amount,
    toInt256(amount) AS unwrapped_amount,
    -toInt256(amount) AS net_amount,
    timestamp,
    block_num
FROM collateral_token_unwrapped;
