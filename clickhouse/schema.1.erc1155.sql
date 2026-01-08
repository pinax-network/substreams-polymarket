-- ERC1155 TransferSingle --
CREATE TABLE IF NOT EXISTS erc1155_transfer_single AS TEMPLATE_LOG
COMMENT 'ERC1155 TransferSingle events';
ALTER TABLE erc1155_transfer_single
	-- event information --
	ADD COLUMN IF NOT EXISTS operator             String COMMENT 'Operator address',
	ADD COLUMN IF NOT EXISTS from_address         String COMMENT 'Sender address',
	ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Recipient address',
	ADD COLUMN IF NOT EXISTS token_id             UInt256 COMMENT 'Token ID',
	ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Transfer amount';

-- ERC1155 TransferBatch --
CREATE TABLE IF NOT EXISTS erc1155_transfer_batch AS TEMPLATE_LOG
COMMENT 'ERC1155 TransferBatch events';
ALTER TABLE erc1155_transfer_batch
	-- event information --
	ADD COLUMN IF NOT EXISTS operator             String COMMENT 'Operator address',
	ADD COLUMN IF NOT EXISTS from_address         String COMMENT 'Sender address',
	ADD COLUMN IF NOT EXISTS to_address           String COMMENT 'Recipient address',
	ADD COLUMN IF NOT EXISTS token_ids            String COMMENT 'Comma-separated token IDs',
	ADD COLUMN IF NOT EXISTS amounts              String COMMENT 'Comma-separated transfer amounts';

-- ERC1155 ApprovalForAll --
CREATE TABLE IF NOT EXISTS erc1155_approval_for_all AS TEMPLATE_LOG
COMMENT 'ERC1155 ApprovalForAll events';
ALTER TABLE erc1155_approval_for_all
	-- event information --
	ADD COLUMN IF NOT EXISTS account              String COMMENT 'Token holder address',
	ADD COLUMN IF NOT EXISTS operator             String COMMENT 'Operator address',
	ADD COLUMN IF NOT EXISTS approved             Bool COMMENT 'Approval status';

-- ERC1155 URI --
CREATE TABLE IF NOT EXISTS erc1155_uri AS TEMPLATE_LOG
COMMENT 'ERC1155 URI update events';
ALTER TABLE erc1155_uri
	-- event information --
	ADD COLUMN IF NOT EXISTS uri_value            String COMMENT 'New token URI',
	ADD COLUMN IF NOT EXISTS token_id             UInt256 COMMENT 'Token ID';
