-- pUSD collateral flow view --
CREATE VIEW IF NOT EXISTS collateral_flow AS
SELECT
    interval_min,
    asset,
    user,
    sum(wrapped_amount) AS wrapped_amount,
    sum(unwrapped_amount) AS unwrapped_amount,
    sum(net_amount) AS net_amount,
    max(timestamp) AS latest_timestamp,
    max(block_num) AS latest_block_num
FROM state_collateral_flow
GROUP BY interval_min, asset, user
ORDER BY interval_min DESC, asset, user;
