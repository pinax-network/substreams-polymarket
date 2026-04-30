use crate::common::{CreateLog, CreateTransaction};
use crate::pb::polymarket::v1 as pb;
use substreams::Hex;
use substreams_abis::prediction::polymarket::v1::conditionaltokens::events as conditional_tokens;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

pub fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_condition_preparation = 0;
    let mut total_condition_resolution = 0;
    let mut total_position_split = 0;
    let mut total_positions_merge = 0;
    let mut total_payout_redemption = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // ConditionPreparation event
            if let Some(event) = conditional_tokens::ConditionPreparation::match_and_decode(log) {
                total_condition_preparation += 1;
                let event = pb::log::Log::ConditionalTokensConditionPreparation(
                    pb::ConditionalTokensConditionPreparation {
                        condition_id: event.condition_id.to_vec(),
                        oracle: event.oracle.to_vec(),
                        question_id: event.question_id.to_vec(),
                        outcome_slot_count: event.outcome_slot_count.to_string(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // ConditionResolution event
            if let Some(event) = conditional_tokens::ConditionResolution::match_and_decode(log) {
                total_condition_resolution += 1;
                let event = pb::log::Log::ConditionalTokensConditionResolution(
                    pb::ConditionalTokensConditionResolution {
                        condition_id: event.condition_id.to_vec(),
                        oracle: event.oracle.to_vec(),
                        question_id: event.question_id.to_vec(),
                        outcome_slot_count: event.outcome_slot_count.to_string(),
                        payout_numerators: event
                            .payout_numerators
                            .iter()
                            .map(|n| n.to_string())
                            .collect(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // PositionSplit event
            if let Some(event) = conditional_tokens::PositionSplit::match_and_decode(log) {
                total_position_split += 1;
                let event = pb::log::Log::ConditionalTokensPositionSplit(
                    pb::ConditionalTokensPositionSplit {
                        stakeholder: event.stakeholder.to_vec(),
                        collateral_token: event.collateral_token.to_vec(),
                        parent_collection_id: event.parent_collection_id.to_vec(),
                        condition_id: event.condition_id.to_vec(),
                        partition: event.partition.iter().map(|p| p.to_string()).collect(),
                        amount: event.amount.to_string(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // PositionsMerge event
            if let Some(event) = conditional_tokens::PositionsMerge::match_and_decode(log) {
                total_positions_merge += 1;
                let event = pb::log::Log::ConditionalTokensPositionsMerge(
                    pb::ConditionalTokensPositionsMerge {
                        stakeholder: event.stakeholder.to_vec(),
                        collateral_token: event.collateral_token.to_vec(),
                        parent_collection_id: event.parent_collection_id.to_vec(),
                        condition_id: event.condition_id.to_vec(),
                        partition: event.partition.iter().map(|p| p.to_string()).collect(),
                        amount: event.amount.to_string(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // PayoutRedemption event
            if let Some(event) = conditional_tokens::PayoutRedemption::match_and_decode(log) {
                total_payout_redemption += 1;
                let event = pb::log::Log::ConditionalTokensPayoutRedemption(
                    pb::ConditionalTokensPayoutRedemption {
                        redeemer: event.redeemer.to_vec(),
                        collateral_token: event.collateral_token.to_vec(),
                        parent_collection_id: event.parent_collection_id.to_vec(),
                        condition_id: event.condition_id.to_vec(),
                        index_sets: event.index_sets.iter().map(|i| i.to_string()).collect(),
                        payout: event.payout.to_string(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }
        }

        if !transaction.logs.is_empty() {
            events.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events.transactions.len());
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

    Ok(events)
}
