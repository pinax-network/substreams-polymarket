use crate::common::CreateLog;
use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v1::conditionaltokens::events as conditional_tokens;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(
    log: &Log,
    transaction: &mut pb::Transaction,
) -> Result<(), substreams::errors::Error> {
    // ConditionPreparation event
    if let Some(event) = conditional_tokens::ConditionPreparation::match_and_decode(log) {
        let event = pb::log::Log::ConditionalTokensConditionPreparation(
            pb::ConditionalTokensConditionPreparation {
                condition_id: event.condition_id.to_vec(),
                oracle: event.oracle.to_vec(),
                question_id: event.question_id.to_vec(),
                outcome_slot_count: event.outcome_slot_count.to_string(),
            },
        );
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // ConditionResolution event
    if let Some(event) = conditional_tokens::ConditionResolution::match_and_decode(log) {
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
        return Ok(());
    }

    // PositionSplit event
    if let Some(event) = conditional_tokens::PositionSplit::match_and_decode(log) {
        let event =
            pb::log::Log::ConditionalTokensPositionSplit(pb::ConditionalTokensPositionSplit {
                stakeholder: event.stakeholder.to_vec(),
                collateral_token: event.collateral_token.to_vec(),
                parent_collection_id: event.parent_collection_id.to_vec(),
                condition_id: event.condition_id.to_vec(),
                partition: event.partition.iter().map(|p| p.to_string()).collect(),
                amount: event.amount.to_string(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PositionsMerge event
    if let Some(event) = conditional_tokens::PositionsMerge::match_and_decode(log) {
        let event =
            pb::log::Log::ConditionalTokensPositionsMerge(pb::ConditionalTokensPositionsMerge {
                stakeholder: event.stakeholder.to_vec(),
                collateral_token: event.collateral_token.to_vec(),
                parent_collection_id: event.parent_collection_id.to_vec(),
                condition_id: event.condition_id.to_vec(),
                partition: event.partition.iter().map(|p| p.to_string()).collect(),
                amount: event.amount.to_string(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PayoutRedemption event
    if let Some(event) = conditional_tokens::PayoutRedemption::match_and_decode(log) {
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
        return Ok(());
    }

    Ok(())
}
