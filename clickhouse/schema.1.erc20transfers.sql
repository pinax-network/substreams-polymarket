-- ERC20 Transfer --
CREATE TABLE IF NOT EXISTS erc20_transfer AS TEMPLATE_LOG
COMMENT 'ERC-20 Transfer events';
ALTER TABLE erc20_transfer
	-- event information --
	ADD COLUMN IF NOT EXISTS from_address         String COMMENT 'Sender address',
	ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Recipient address',
	ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Transfer amount';

-- ERC20 Approval --
CREATE TABLE IF NOT EXISTS erc20_approval AS TEMPLATE_LOG
COMMENT 'ERC-20 Approval events';
ALTER TABLE erc20_approval
	-- event information --
	ADD COLUMN IF NOT EXISTS owner                String COMMENT 'Token owner address',
	ADD COLUMN IF NOT EXISTS spender              String COMMENT 'Spender address',
	ADD COLUMN IF NOT EXISTS value                UInt256 COMMENT 'Approval amount';

-- WETH Deposit --
CREATE TABLE IF NOT EXISTS weth_deposit AS TEMPLATE_LOG
COMMENT 'WETH Deposit events';
ALTER TABLE weth_deposit
	-- event information --
	ADD COLUMN IF NOT EXISTS dst                  String COMMENT 'Destination address',
	ADD COLUMN IF NOT EXISTS wad                  UInt256 COMMENT 'Deposit amount';

-- WETH Withdrawal --
CREATE TABLE IF NOT EXISTS weth_withdrawal AS TEMPLATE_LOG
COMMENT 'WETH Withdrawal events';
ALTER TABLE weth_withdrawal
	-- event information --
	ADD COLUMN IF NOT EXISTS src                  String COMMENT 'Source address',
	ADD COLUMN IF NOT EXISTS wad                  UInt256 COMMENT 'Withdrawal amount';
