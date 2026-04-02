-- ConditionalTokens ConditionPreparation --
CREATE TABLE IF NOT EXISTS conditionaltokens_condition_preparation AS TEMPLATE_LOG
COMMENT 'ConditionalTokens ConditionPreparation events';
ALTER TABLE conditionaltokens_condition_preparation
	-- event information --
	ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS oracle               String COMMENT 'Oracle address',
	ADD COLUMN IF NOT EXISTS question_id          String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS outcome_slot_count   UInt256 COMMENT 'Number of outcome slots';

-- ConditionalTokens ConditionResolution --
CREATE TABLE IF NOT EXISTS conditionaltokens_condition_resolution AS TEMPLATE_LOG
COMMENT 'ConditionalTokens ConditionResolution events';
ALTER TABLE conditionaltokens_condition_resolution
	-- event information --
	ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS oracle               String COMMENT 'Oracle address',
	ADD COLUMN IF NOT EXISTS question_id          String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS outcome_slot_count   UInt256 COMMENT 'Number of outcome slots',
	ADD COLUMN IF NOT EXISTS payout_numerators    String COMMENT 'Comma-separated payout numerators (uint256[])';

-- ConditionalTokens PositionSplit --
CREATE TABLE IF NOT EXISTS conditionaltokens_position_split AS TEMPLATE_LOG
COMMENT 'ConditionalTokens PositionSplit events';
ALTER TABLE conditionaltokens_position_split
	-- event information --
	ADD COLUMN IF NOT EXISTS stakeholder          String COMMENT 'Stakeholder address',
	ADD COLUMN IF NOT EXISTS collateral_token     String COMMENT 'Collateral token address',
	ADD COLUMN IF NOT EXISTS parent_collection_id String COMMENT 'Parent collection ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS partition            String COMMENT 'Comma-separated partition (uint256[])',
	ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Split amount';
ALTER TABLE conditionaltokens_position_split
	ADD INDEX IF NOT EXISTS idx_condition_id (condition_id) TYPE bloom_filter GRANULARITY 1,
	ADD INDEX IF NOT EXISTS idx_stakeholder (stakeholder) TYPE bloom_filter GRANULARITY 1;

-- ConditionalTokens PositionsMerge --
CREATE TABLE IF NOT EXISTS conditionaltokens_positions_merge AS TEMPLATE_LOG
COMMENT 'ConditionalTokens PositionsMerge events';
ALTER TABLE conditionaltokens_positions_merge
	-- event information --
	ADD COLUMN IF NOT EXISTS stakeholder          String COMMENT 'Stakeholder address',
	ADD COLUMN IF NOT EXISTS collateral_token     String COMMENT 'Collateral token address',
	ADD COLUMN IF NOT EXISTS parent_collection_id String COMMENT 'Parent collection ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS partition            String COMMENT 'Comma-separated partition (uint256[])',
	ADD COLUMN IF NOT EXISTS amount               UInt256 COMMENT 'Merge amount';
ALTER TABLE conditionaltokens_positions_merge
	ADD INDEX IF NOT EXISTS idx_condition_id (condition_id) TYPE bloom_filter GRANULARITY 1,
	ADD INDEX IF NOT EXISTS idx_stakeholder (stakeholder) TYPE bloom_filter GRANULARITY 1;

-- ConditionalTokens PayoutRedemption --
CREATE TABLE IF NOT EXISTS conditionaltokens_payout_redemption AS TEMPLATE_LOG
COMMENT 'ConditionalTokens PayoutRedemption events';
ALTER TABLE conditionaltokens_payout_redemption
	-- event information --
	ADD COLUMN IF NOT EXISTS redeemer             String COMMENT 'Redeemer address',
	ADD COLUMN IF NOT EXISTS collateral_token     String COMMENT 'Collateral token address',
	ADD COLUMN IF NOT EXISTS parent_collection_id String COMMENT 'Parent collection ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS condition_id         String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
	ADD COLUMN IF NOT EXISTS index_sets           String COMMENT 'Comma-separated index sets (uint256[])',
	ADD COLUMN IF NOT EXISTS payout               UInt256 COMMENT 'Payout amount';
ALTER TABLE conditionaltokens_payout_redemption
	ADD INDEX IF NOT EXISTS idx_condition_id (condition_id) TYPE bloom_filter GRANULARITY 1,
	ADD INDEX IF NOT EXISTS idx_redeemer (redeemer) TYPE bloom_filter GRANULARITY 1;
