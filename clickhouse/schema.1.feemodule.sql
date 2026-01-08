-- FeeModule FeeRefunded --
CREATE TABLE IF NOT EXISTS feemodule_fee_refunded AS TEMPLATE_LOG
COMMENT 'FeeModule FeeRefunded events';
ALTER TABLE feemodule_fee_refunded
    -- event information --
    ADD COLUMN IF NOT EXISTS order_hash           String COMMENT 'Order hash (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Recipient address',
    ADD COLUMN IF NOT EXISTS token_id             UInt256 COMMENT 'Token ID',
    ADD COLUMN IF NOT EXISTS refund               UInt256 COMMENT 'Refund amount',
    ADD COLUMN IF NOT EXISTS fee_charged          UInt256 COMMENT 'Fee charged amount';

-- FeeModule FeeWithdrawn --
CREATE TABLE IF NOT EXISTS feemodule_fee_withdrawn AS TEMPLATE_LOG
COMMENT 'FeeModule FeeWithdrawn events';
ALTER TABLE feemodule_fee_withdrawn
    -- event information --
    ADD COLUMN IF NOT EXISTS token                String COMMENT 'Token address',
    ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Recipient address',
    ADD COLUMN IF NOT EXISTS token_id             UInt256 COMMENT 'Token ID',
    ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Withdrawn amount';

-- FeeModule NewAdmin --
CREATE TABLE IF NOT EXISTS feemodule_new_admin AS TEMPLATE_LOG
COMMENT 'FeeModule NewAdmin events';
ALTER TABLE feemodule_new_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS new_admin_address    String COMMENT 'New admin address';

-- FeeModule RemovedAdmin --
CREATE TABLE IF NOT EXISTS feemodule_removed_admin AS TEMPLATE_LOG
COMMENT 'FeeModule RemovedAdmin events';
ALTER TABLE feemodule_removed_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin                String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS removed_admin        String COMMENT 'Removed admin address';
