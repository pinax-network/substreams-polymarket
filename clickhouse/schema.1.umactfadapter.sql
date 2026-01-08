-- UmaCtfAdapter AncillaryDataUpdated --
CREATE TABLE IF NOT EXISTS umactfadapter_ancillary_data_updated AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter AncillaryDataUpdated events';
ALTER TABLE umactfadapter_ancillary_data_updated
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS owner               String COMMENT 'Owner address',
    ADD COLUMN IF NOT EXISTS update_data         String COMMENT 'Update data (bytes as hex with 0x prefix)';

-- UmaCtfAdapter NewAdmin --
CREATE TABLE IF NOT EXISTS umactfadapter_new_admin AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter NewAdmin events';
ALTER TABLE umactfadapter_new_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin               String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS new_admin_address   String COMMENT 'New admin address';

-- UmaCtfAdapter QuestionEmergencyResolved --
CREATE TABLE IF NOT EXISTS umactfadapter_question_emergency_resolved AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionEmergencyResolved events';
ALTER TABLE umactfadapter_question_emergency_resolved
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS payouts             String COMMENT 'Comma-separated payouts (uint256[])';

-- UmaCtfAdapter QuestionFlagged --
CREATE TABLE IF NOT EXISTS umactfadapter_question_flagged AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionFlagged events';
ALTER TABLE umactfadapter_question_flagged
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)';

-- UmaCtfAdapter QuestionInitialized --
CREATE TABLE IF NOT EXISTS umactfadapter_question_initialized AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionInitialized events';
ALTER TABLE umactfadapter_question_initialized
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS request_timestamp   UInt256 COMMENT 'Request timestamp',
    ADD COLUMN IF NOT EXISTS creator             String COMMENT 'Creator address',
    ADD COLUMN IF NOT EXISTS ancillary_data      String COMMENT 'Ancillary data (bytes as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS reward_token        String COMMENT 'Reward token address',
    ADD COLUMN IF NOT EXISTS reward              UInt256 COMMENT 'Reward amount',
    ADD COLUMN IF NOT EXISTS proposal_bond       UInt256 COMMENT 'Proposal bond amount';

-- UmaCtfAdapter QuestionPaused --
CREATE TABLE IF NOT EXISTS umactfadapter_question_paused AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionPaused events';
ALTER TABLE umactfadapter_question_paused
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)';

-- UmaCtfAdapter QuestionReset --
CREATE TABLE IF NOT EXISTS umactfadapter_question_reset AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionReset events';
ALTER TABLE umactfadapter_question_reset
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)';

-- UmaCtfAdapter QuestionResolved --
CREATE TABLE IF NOT EXISTS umactfadapter_question_resolved AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionResolved events';
ALTER TABLE umactfadapter_question_resolved
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)',
    ADD COLUMN IF NOT EXISTS settled_price       Int256 COMMENT 'Settled price (int256)',
    ADD COLUMN IF NOT EXISTS payouts             String COMMENT 'Comma-separated payouts (uint256[])';

-- UmaCtfAdapter QuestionUnpaused --
CREATE TABLE IF NOT EXISTS umactfadapter_question_unpaused AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionUnpaused events';
ALTER TABLE umactfadapter_question_unpaused
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)';

-- UmaCtfAdapter RemovedAdmin --
CREATE TABLE IF NOT EXISTS umactfadapter_removed_admin AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter RemovedAdmin events';
ALTER TABLE umactfadapter_removed_admin
    -- event information --
    ADD COLUMN IF NOT EXISTS admin               String COMMENT 'Admin address',
    ADD COLUMN IF NOT EXISTS removed_admin       String COMMENT 'Removed admin address';

-- UmaCtfAdapter QuestionUnflagged (V3 only) --
CREATE TABLE IF NOT EXISTS umactfadapter_question_unflagged AS TEMPLATE_LOG
COMMENT 'UmaCtfAdapter QuestionUnflagged events (V3 only)';
ALTER TABLE umactfadapter_question_unflagged
    -- event information --
    ADD COLUMN IF NOT EXISTS question_id         String COMMENT 'Question ID (bytes32 as hex with 0x prefix)';
