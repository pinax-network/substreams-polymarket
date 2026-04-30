-- CollateralToken Wrapped --
CREATE TABLE IF NOT EXISTS collateral_token_wrapped AS TEMPLATE_LOG
COMMENT 'pUSD CollateralToken Wrapped events';
ALTER TABLE collateral_token_wrapped
    ADD COLUMN IF NOT EXISTS caller               String COMMENT 'Caller address',
    ADD COLUMN IF NOT EXISTS asset                String COMMENT 'Wrapped asset address',
    ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'pUSD recipient address',
    ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Wrapped amount';

-- CollateralToken Unwrapped --
CREATE TABLE IF NOT EXISTS collateral_token_unwrapped AS TEMPLATE_LOG
COMMENT 'pUSD CollateralToken Unwrapped events';
ALTER TABLE collateral_token_unwrapped
    ADD COLUMN IF NOT EXISTS caller               String COMMENT 'Caller address',
    ADD COLUMN IF NOT EXISTS asset                String COMMENT 'Unwrapped asset address',
    ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Asset recipient address',
    ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Unwrapped amount';
