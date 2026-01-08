use common::{CreateLog, CreateTransaction};
use proto::pb::conditionaltokens::v1 as pb;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

// Event signatures for ConditionalTokens
// ConditionPreparation(bytes32 indexed conditionId, address indexed oracle, bytes32 indexed questionId, uint256 outcomeSlotCount)
const CONDITION_PREPARATION_TOPIC: [u8; 32] = hex_literal::hex!(
    "ab3760c3bd2bb38b5bcf54dc79802ed67338b4b83ebe4e6fabbef47d2ad2da4b"
);

// ConditionResolution(bytes32 indexed conditionId, address indexed oracle, bytes32 indexed questionId, uint256 outcomeSlotCount, uint256[] payoutNumerators)
const CONDITION_RESOLUTION_TOPIC: [u8; 32] = hex_literal::hex!(
    "b44d84d3289691f71497564b85d47f0c58ee5f57c29ac06fc5a60d6b30a1e5c5"
);

// PositionSplit(address indexed stakeholder, IERC20 collateralToken, bytes32 indexed parentCollectionId, bytes32 indexed conditionId, uint256[] partition, uint256 amount)
const POSITION_SPLIT_TOPIC: [u8; 32] = hex_literal::hex!(
    "2e6bb91f8cbcda0c93623c54d0403a43514f2b60a07d70d9e41bcc1ea3fa0e1f"
);

// PositionsMerge(address indexed stakeholder, IERC20 collateralToken, bytes32 indexed parentCollectionId, bytes32 indexed conditionId, uint256[] partition, uint256 amount)
const POSITIONS_MERGE_TOPIC: [u8; 32] = hex_literal::hex!(
    "6f13ca62ed55bdc2c9d3f3c70e4a3bb1a5fcc5c2b21cb8ce6ea3f8ca5eb7b19b"
);

// PayoutRedemption(address indexed redeemer, IERC20 indexed collateralToken, bytes32 indexed parentCollectionId, bytes32 conditionId, uint256[] indexSets, uint256 payout)
const PAYOUT_REDEMPTION_TOPIC: [u8; 32] = hex_literal::hex!(
    "2682012a0bdd2cdfc287e4f7e8d73d6b5e0f5a5d5f5ae3809f52c7f22cd63dbb"
);

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let mut total_condition_preparation = 0;
    let mut total_condition_resolution = 0;
    let mut total_position_split = 0;
    let mut total_positions_merge = 0;
    let mut total_payout_redemption = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            if log.topics.is_empty() {
                continue;
            }

            let topic0: [u8; 32] = match log.topics[0].as_slice().try_into() {
                Ok(t) => t,
                Err(_) => continue,
            };

            // ConditionPreparation event
            if topic0 == CONDITION_PREPARATION_TOPIC && log.topics.len() >= 4 {
                if let Some(event) = decode_condition_preparation(log) {
                    total_condition_preparation += 1;
                    let event = pb::log::Log::ConditionPreparation(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // ConditionResolution event
            if topic0 == CONDITION_RESOLUTION_TOPIC && log.topics.len() >= 4 {
                if let Some(event) = decode_condition_resolution(log) {
                    total_condition_resolution += 1;
                    let event = pb::log::Log::ConditionResolution(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // PositionSplit event
            if topic0 == POSITION_SPLIT_TOPIC && log.topics.len() >= 4 {
                if let Some(event) = decode_position_split(log) {
                    total_position_split += 1;
                    let event = pb::log::Log::PositionSplit(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // PositionsMerge event
            if topic0 == POSITIONS_MERGE_TOPIC && log.topics.len() >= 4 {
                if let Some(event) = decode_positions_merge(log) {
                    total_positions_merge += 1;
                    let event = pb::log::Log::PositionsMerge(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // PayoutRedemption event
            if topic0 == PAYOUT_REDEMPTION_TOPIC && log.topics.len() >= 4 {
                if let Some(event) = decode_payout_redemption(log) {
                    total_payout_redemption += 1;
                    let event = pb::log::Log::PayoutRedemption(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }
        }

        if !transaction.logs.is_empty() {
            events_output.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events_output.transactions.len());
    substreams::log::info!(
        "Total ConditionPreparation events: {}",
        total_condition_preparation
    );
    substreams::log::info!(
        "Total ConditionResolution events: {}",
        total_condition_resolution
    );
    substreams::log::info!("Total PositionSplit events: {}", total_position_split);
    substreams::log::info!("Total PositionsMerge events: {}", total_positions_merge);
    substreams::log::info!("Total PayoutRedemption events: {}", total_payout_redemption);

    Ok(events_output)
}

fn decode_condition_preparation(
    log: &substreams_ethereum::pb::eth::v2::Log,
) -> Option<pb::ConditionPreparation> {
    // Topics: conditionId (indexed), oracle (indexed), questionId (indexed)
    // Data: outcomeSlotCount (uint256)
    if log.topics.len() < 4 || log.data.len() < 32 {
        return None;
    }

    let condition_id = log.topics[1].clone();
    let oracle = extract_address_from_topic(&log.topics[2]);
    let question_id = log.topics[3].clone();
    let outcome_slot_count = decode_uint256(&log.data[0..32]);

    Some(pb::ConditionPreparation {
        condition_id,
        oracle,
        question_id,
        outcome_slot_count,
    })
}

fn decode_condition_resolution(
    log: &substreams_ethereum::pb::eth::v2::Log,
) -> Option<pb::ConditionResolution> {
    // Topics: conditionId (indexed), oracle (indexed), questionId (indexed)
    // Data: outcomeSlotCount (uint256), payoutNumerators (uint256[])
    if log.topics.len() < 4 || log.data.len() < 64 {
        return None;
    }

    let condition_id = log.topics[1].clone();
    let oracle = extract_address_from_topic(&log.topics[2]);
    let question_id = log.topics[3].clone();
    let outcome_slot_count = decode_uint256(&log.data[0..32]);
    let payout_numerators = decode_uint256_array(&log.data, 32);

    Some(pb::ConditionResolution {
        condition_id,
        oracle,
        question_id,
        outcome_slot_count,
        payout_numerators,
    })
}

fn decode_position_split(
    log: &substreams_ethereum::pb::eth::v2::Log,
) -> Option<pb::PositionSplit> {
    // Topics: stakeholder (indexed), parentCollectionId (indexed), conditionId (indexed)
    // Data: collateralToken (address), partition (uint256[]), amount (uint256)
    if log.topics.len() < 4 || log.data.len() < 96 {
        return None;
    }

    let stakeholder = extract_address_from_topic(&log.topics[1]);
    let parent_collection_id = log.topics[2].clone();
    let condition_id = log.topics[3].clone();

    // Data: collateralToken (32 bytes padded address), then dynamic array offset, then amount offset
    let collateral_token = extract_address_from_bytes(&log.data[0..32]);

    // Decode partition array and amount from data
    // The partition is a dynamic array, so we need to read the offset first
    let (partition, amount) = decode_partition_and_amount(&log.data, 32)?;

    Some(pb::PositionSplit {
        stakeholder,
        collateral_token,
        parent_collection_id,
        condition_id,
        partition,
        amount,
    })
}

fn decode_positions_merge(
    log: &substreams_ethereum::pb::eth::v2::Log,
) -> Option<pb::PositionsMerge> {
    // Topics: stakeholder (indexed), parentCollectionId (indexed), conditionId (indexed)
    // Data: collateralToken (address), partition (uint256[]), amount (uint256)
    if log.topics.len() < 4 || log.data.len() < 96 {
        return None;
    }

    let stakeholder = extract_address_from_topic(&log.topics[1]);
    let parent_collection_id = log.topics[2].clone();
    let condition_id = log.topics[3].clone();

    let collateral_token = extract_address_from_bytes(&log.data[0..32]);
    let (partition, amount) = decode_partition_and_amount(&log.data, 32)?;

    Some(pb::PositionsMerge {
        stakeholder,
        collateral_token,
        parent_collection_id,
        condition_id,
        partition,
        amount,
    })
}

fn decode_payout_redemption(
    log: &substreams_ethereum::pb::eth::v2::Log,
) -> Option<pb::PayoutRedemption> {
    // Topics: redeemer (indexed), collateralToken (indexed), parentCollectionId (indexed)
    // Data: conditionId (bytes32), indexSets (uint256[]), payout (uint256)
    if log.topics.len() < 4 || log.data.len() < 96 {
        return None;
    }

    let redeemer = extract_address_from_topic(&log.topics[1]);
    let collateral_token = extract_address_from_topic(&log.topics[2]);
    let parent_collection_id = log.topics[3].clone();

    // Data: conditionId (32 bytes), then dynamic array offset for indexSets, then payout
    let condition_id = log.data[0..32].to_vec();
    let (index_sets, payout) = decode_index_sets_and_payout(&log.data, 32)?;

    Some(pb::PayoutRedemption {
        redeemer,
        collateral_token,
        parent_collection_id,
        condition_id,
        index_sets,
        payout,
    })
}

// Helper functions

fn extract_address_from_topic(topic: &[u8]) -> Vec<u8> {
    // Address is in the last 20 bytes of a 32-byte topic
    if topic.len() >= 32 {
        topic[12..32].to_vec()
    } else {
        topic.to_vec()
    }
}

fn extract_address_from_bytes(data: &[u8]) -> Vec<u8> {
    // Address is padded to 32 bytes, actual address is in last 20 bytes
    if data.len() >= 32 {
        data[12..32].to_vec()
    } else {
        data.to_vec()
    }
}

fn decode_uint256(data: &[u8]) -> String {
    if data.len() < 32 {
        return "0".to_string();
    }
    let hex_str = Hex::encode(data);
    // Convert hex to decimal string
    if let Ok(num) = primitive_types::U256::from_str_radix(&hex_str, 16) {
        num.to_string()
    } else {
        "0".to_string()
    }
}

fn decode_uint256_array(data: &[u8], offset: usize) -> Vec<String> {
    let mut result = Vec::new();
    if data.len() < offset + 32 {
        return result;
    }

    // Read the offset to the array data
    let array_offset = decode_uint256_as_usize(&data[offset..offset + 32]);
    if array_offset + 32 > data.len() {
        return result;
    }

    // Read the length of the array
    let array_len = decode_uint256_as_usize(&data[array_offset..array_offset + 32]);
    let array_start = array_offset + 32;

    for i in 0..array_len {
        let elem_start = array_start + i * 32;
        if elem_start + 32 > data.len() {
            break;
        }
        result.push(decode_uint256(&data[elem_start..elem_start + 32]));
    }

    result
}

fn decode_uint256_as_usize(data: &[u8]) -> usize {
    if data.len() < 32 {
        return 0;
    }
    let hex_str = Hex::encode(data);
    if let Ok(num) = primitive_types::U256::from_str_radix(&hex_str, 16) {
        num.as_usize()
    } else {
        0
    }
}

fn decode_partition_and_amount(data: &[u8], start: usize) -> Option<(Vec<String>, String)> {
    if data.len() < start + 64 {
        return None;
    }

    // Read offset to partition array
    let partition_offset = decode_uint256_as_usize(&data[start..start + 32]);
    // Read amount (after partition offset)
    let amount = decode_uint256(&data[start + 32..start + 64]);

    // Decode partition array
    if partition_offset + 32 > data.len() {
        return Some((Vec::new(), amount));
    }

    let array_len = decode_uint256_as_usize(&data[partition_offset..partition_offset + 32]);
    let array_start = partition_offset + 32;

    let mut partition = Vec::new();
    for i in 0..array_len {
        let elem_start = array_start + i * 32;
        if elem_start + 32 > data.len() {
            break;
        }
        partition.push(decode_uint256(&data[elem_start..elem_start + 32]));
    }

    Some((partition, amount))
}

fn decode_index_sets_and_payout(data: &[u8], start: usize) -> Option<(Vec<String>, String)> {
    if data.len() < start + 64 {
        return None;
    }

    // Read offset to indexSets array
    let index_sets_offset = decode_uint256_as_usize(&data[start..start + 32]);
    // Read payout
    let payout = decode_uint256(&data[start + 32..start + 64]);

    // Decode indexSets array
    if index_sets_offset + 32 > data.len() {
        return Some((Vec::new(), payout));
    }

    let array_len = decode_uint256_as_usize(&data[index_sets_offset..index_sets_offset + 32]);
    let array_start = index_sets_offset + 32;

    let mut index_sets = Vec::new();
    for i in 0..array_len {
        let elem_start = array_start + i * 32;
        if elem_start + 32 > data.len() {
            break;
        }
        index_sets.push(decode_uint256(&data[elem_start..elem_start + 32]));
    }

    Some((index_sets, payout))
}
