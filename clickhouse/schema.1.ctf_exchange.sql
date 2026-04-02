-- CTFExchange OrderFilled --
CREATE TABLE IF NOT EXISTS ctfexchange_order_filled AS TEMPLATE_LOG
COMMENT 'CTFExchange OrderFilled events (swap events)';
ALTER TABLE ctfexchange_order_filled
    -- event information --
    ADD COLUMN IF NOT EXISTS order_hash           String COMMENT 'Order hash identifier',
    ADD COLUMN IF NOT EXISTS maker                String COMMENT 'Maker address',
    ADD COLUMN IF NOT EXISTS taker                String COMMENT 'Taker address',
    ADD COLUMN IF NOT EXISTS maker_asset_id       UInt256 COMMENT 'Maker asset token ID',
    ADD COLUMN IF NOT EXISTS taker_asset_id       UInt256 COMMENT 'Taker asset token ID',
    ADD COLUMN IF NOT EXISTS maker_amount_filled  UInt256 COMMENT 'Maker amount filled',
    ADD COLUMN IF NOT EXISTS taker_amount_filled  UInt256 COMMENT 'Taker amount filled',
    ADD COLUMN IF NOT EXISTS fee                  UInt256 COMMENT 'Fee amount';
ALTER TABLE ctfexchange_order_filled
    ADD INDEX IF NOT EXISTS idx_taker_asset_id (taker_asset_id) TYPE bloom_filter GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_maker_asset_id (maker_asset_id) TYPE bloom_filter GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_taker (taker) TYPE bloom_filter GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_maker (maker) TYPE bloom_filter GRANULARITY 1;

-- CTFExchange FeeCharged --
CREATE TABLE IF NOT EXISTS ctfexchange_fee_charged AS TEMPLATE_LOG
COMMENT 'CTFExchange FeeCharged events';
ALTER TABLE ctfexchange_fee_charged
    -- event information --
    ADD COLUMN IF NOT EXISTS receiver             String COMMENT 'Fee receiver address',
    ADD COLUMN IF NOT EXISTS token_id             UInt256 COMMENT 'Token ID',
    ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Fee amount';

-- CTFExchange NewAdmin --
CREATE TABLE IF NOT EXISTS ctfexchange_new_admin AS TEMPLATE_LOG
COMMENT 'CTFExchange NewAdmin events';
ALTER TABLE ctfexchange_new_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS new_admin_address    String COMMENT 'New admin address',
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin who added the new admin';

-- CTFExchange NewOperator --
CREATE TABLE IF NOT EXISTS ctfexchange_new_operator AS TEMPLATE_LOG
COMMENT 'CTFExchange NewOperator events';
ALTER TABLE ctfexchange_new_operator
    -- event information --
    ADD COLUMN IF NOT EXISTS new_operator_address String COMMENT 'New operator address',
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin who added the new operator';

-- CTFExchange OrderCancelled --
CREATE TABLE IF NOT EXISTS ctfexchange_order_cancelled AS TEMPLATE_LOG
COMMENT 'CTFExchange OrderCancelled events';
ALTER TABLE ctfexchange_order_cancelled
    -- event information --
    ADD COLUMN IF NOT EXISTS order_hash           String COMMENT 'Order hash identifier';

-- CTFExchange OrdersMatched --
CREATE TABLE IF NOT EXISTS ctfexchange_orders_matched AS TEMPLATE_LOG
COMMENT 'CTFExchange OrdersMatched events';
ALTER TABLE ctfexchange_orders_matched
    -- event information --
    ADD COLUMN IF NOT EXISTS taker_order_hash     String COMMENT 'Taker order hash',
    ADD COLUMN IF NOT EXISTS taker_order_maker    String COMMENT 'Taker order maker address',
    ADD COLUMN IF NOT EXISTS maker_asset_id       UInt256 COMMENT 'Maker asset token ID',
    ADD COLUMN IF NOT EXISTS taker_asset_id       UInt256 COMMENT 'Taker asset token ID',
    ADD COLUMN IF NOT EXISTS maker_amount_filled  UInt256 COMMENT 'Maker amount filled',
    ADD COLUMN IF NOT EXISTS taker_amount_filled  UInt256 COMMENT 'Taker amount filled';

-- CTFExchange ProxyFactoryUpdated --
CREATE TABLE IF NOT EXISTS ctfexchange_proxy_factory_updated AS TEMPLATE_LOG
COMMENT 'CTFExchange ProxyFactoryUpdated events';
ALTER TABLE ctfexchange_proxy_factory_updated
    -- event information --
    ADD COLUMN IF NOT EXISTS old_proxy_factory    String COMMENT 'Old proxy factory address',
    ADD COLUMN IF NOT EXISTS new_proxy_factory    String COMMENT 'New proxy factory address';

-- CTFExchange RemovedAdmin --
CREATE TABLE IF NOT EXISTS ctfexchange_removed_admin AS TEMPLATE_LOG
COMMENT 'CTFExchange RemovedAdmin events';
ALTER TABLE ctfexchange_removed_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS removed_admin        String COMMENT 'Removed admin address',
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin who removed the admin';

-- CTFExchange RemovedOperator --
CREATE TABLE IF NOT EXISTS ctfexchange_removed_operator AS TEMPLATE_LOG
COMMENT 'CTFExchange RemovedOperator events';
ALTER TABLE ctfexchange_removed_operator
    -- event information --
    ADD COLUMN IF NOT EXISTS removed_operator     String COMMENT 'Removed operator address',
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin who removed the operator';

-- CTFExchange SafeFactoryUpdated --
CREATE TABLE IF NOT EXISTS ctfexchange_safe_factory_updated AS TEMPLATE_LOG
COMMENT 'CTFExchange SafeFactoryUpdated events';
ALTER TABLE ctfexchange_safe_factory_updated
    -- event information --
    ADD COLUMN IF NOT EXISTS old_safe_factory     String COMMENT 'Old safe factory address',
    ADD COLUMN IF NOT EXISTS new_safe_factory     String COMMENT 'New safe factory address';

-- CTFExchange TokenRegistered --
CREATE TABLE IF NOT EXISTS ctfexchange_token_registered AS TEMPLATE_LOG
COMMENT 'CTFExchange TokenRegistered events';
ALTER TABLE ctfexchange_token_registered
    -- event information --
    ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS token0               UInt256 COMMENT 'Token0 ID',
    ADD COLUMN IF NOT EXISTS token1               UInt256 COMMENT 'Token1 ID';

-- CTFExchange TradingPaused --
CREATE TABLE IF NOT EXISTS ctfexchange_trading_paused AS TEMPLATE_LOG
COMMENT 'CTFExchange TradingPaused events';
ALTER TABLE ctfexchange_trading_paused
    -- event information --
    ADD COLUMN IF NOT EXISTS pauser               String COMMENT 'Address that paused trading';

-- CTFExchange TradingUnpaused --
CREATE TABLE IF NOT EXISTS ctfexchange_trading_unpaused AS TEMPLATE_LOG
COMMENT 'CTFExchange TradingUnpaused events';
ALTER TABLE ctfexchange_trading_unpaused
    -- event information --
    ADD COLUMN IF NOT EXISTS pauser               String COMMENT 'Address that unpaused trading';