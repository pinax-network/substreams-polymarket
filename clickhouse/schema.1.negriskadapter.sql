-- NegRiskAdapter MarketPrepared --
CREATE TABLE IF NOT EXISTS negriskadapter_market_prepared AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter MarketPrepared events';
ALTER TABLE negriskadapter_market_prepared
    -- event information --
    ADD COLUMN IF NOT EXISTS market_id           String COMMENT 'Market ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS oracle              String COMMENT 'Oracle address',
    ADD COLUMN IF NOT EXISTS fee_bips            UInt256 COMMENT 'Fee in basis points',
    ADD COLUMN IF NOT EXISTS event_data          String COMMENT 'Event data (bytes as hex with 0x prefix)';

-- NegRiskAdapter NewAdmin --
CREATE TABLE IF NOT EXISTS negriskadapter_new_admin AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter NewAdmin events';
ALTER TABLE negriskadapter_new_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin               String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS new_admin_address   String COMMENT 'New admin address';

-- NegRiskAdapter OutcomeReported --
CREATE TABLE IF NOT EXISTS negriskadapter_outcome_reported AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter OutcomeReported events';
ALTER TABLE negriskadapter_outcome_reported
    -- event information --
    ADD COLUMN IF NOT EXISTS market_id           String COMMENT 'Market ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS outcome             Bool COMMENT 'Outcome (true/false)';

-- NegRiskAdapter PayoutRedemption --
CREATE TABLE IF NOT EXISTS negriskadapter_payout_redemption AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter PayoutRedemption events';
ALTER TABLE negriskadapter_payout_redemption
    -- event information --
    ADD COLUMN IF NOT EXISTS redeemer            String COMMENT 'Redeemer address',
    ADD COLUMN IF NOT EXISTS condition_id        String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS amounts             String COMMENT 'Comma-separated amounts (uint256[])',
    ADD COLUMN IF NOT EXISTS payout              UInt256 COMMENT 'Payout amount';

-- NegRiskAdapter PositionSplit --
CREATE TABLE IF NOT EXISTS negriskadapter_position_split AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter PositionSplit events';
ALTER TABLE negriskadapter_position_split
    -- event information --
    ADD COLUMN IF NOT EXISTS stakeholder         String COMMENT 'Stakeholder address',
    ADD COLUMN IF NOT EXISTS condition_id        String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS amount              UInt256 COMMENT 'Split amount';

-- NegRiskAdapter PositionsConverted --
CREATE TABLE IF NOT EXISTS negriskadapter_positions_converted AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter PositionsConverted events';
ALTER TABLE negriskadapter_positions_converted
    -- event information --
    ADD COLUMN IF NOT EXISTS stakeholder         String COMMENT 'Stakeholder address',
    ADD COLUMN IF NOT EXISTS market_id           String COMMENT 'Market ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS index_set           UInt256 COMMENT 'Index set',
    ADD COLUMN IF NOT EXISTS amount              UInt256 COMMENT 'Converted amount';

-- NegRiskAdapter PositionsMerge --
CREATE TABLE IF NOT EXISTS negriskadapter_positions_merge AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter PositionsMerge events';
ALTER TABLE negriskadapter_positions_merge
    -- event information --
    ADD COLUMN IF NOT EXISTS stakeholder         String COMMENT 'Stakeholder address',
    ADD COLUMN IF NOT EXISTS condition_id        String COMMENT 'Condition ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS amount              UInt256 COMMENT 'Merge amount';

-- NegRiskAdapter QuestionPrepared --
CREATE TABLE IF NOT EXISTS negriskadapter_question_prepared AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter QuestionPrepared events';
ALTER TABLE negriskadapter_question_prepared
    -- event information --
    ADD COLUMN IF NOT EXISTS market_id           String COMMENT 'Market ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS question_index      UInt256 COMMENT 'Question index',
    ADD COLUMN IF NOT EXISTS event_data          String COMMENT 'Event data (bytes as hex with 0x prefix)';

-- NegRiskAdapter RemovedAdmin --
CREATE TABLE IF NOT EXISTS negriskadapter_removed_admin AS TEMPLATE_LOG
COMMENT 'NegRiskAdapter RemovedAdmin events';
ALTER TABLE negriskadapter_removed_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin               String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS removed_admin       String COMMENT 'Removed admin address';
