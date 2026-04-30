use crate::common::CreateLog;
use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v1::negriskadapter::events;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(
    log: &Log,
    transaction: &mut pb::Transaction,
) -> Result<(), substreams::errors::Error> {
    // MarketPrepared event
    if let Some(event) = events::MarketPrepared::match_and_decode(log) {
        let event = pb::log::Log::NegriskAdapterMarketPrepared(pb::NegRiskAdapterMarketPrepared {
            market_id: event.market_id.to_vec(),
            oracle: event.oracle.to_vec(),
            fee_bips: event.fee_bips.to_string(),
            data: event.data.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // NewAdmin event
    if let Some(event) = events::NewAdmin::match_and_decode(log) {
        let event = pb::log::Log::NegriskAdapterNewAdmin(pb::NegRiskAdapterNewAdmin {
            admin: event.admin.to_vec(),
            new_admin_address: event.new_admin_address.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // OutcomeReported event
    if let Some(event) = events::OutcomeReported::match_and_decode(log) {
        let event =
            pb::log::Log::NegriskAdapterOutcomeReported(pb::NegRiskAdapterOutcomeReported {
                market_id: event.market_id.to_vec(),
                question_id: event.question_id.to_vec(),
                outcome: event.outcome,
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PayoutRedemption event
    if let Some(event) = events::PayoutRedemption::match_and_decode(log) {
        let event =
            pb::log::Log::NegriskAdapterPayoutRedemption(pb::NegRiskAdapterPayoutRedemption {
                redeemer: event.redeemer.to_vec(),
                condition_id: event.condition_id.to_vec(),
                amounts: event.amounts.iter().map(|a| a.to_string()).collect(),
                payout: event.payout.to_string(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PositionSplit event
    if let Some(event) = events::PositionSplit::match_and_decode(log) {
        let event = pb::log::Log::NegriskAdapterPositionSplit(pb::NegRiskAdapterPositionSplit {
            stakeholder: event.stakeholder.to_vec(),
            condition_id: event.condition_id.to_vec(),
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PositionsConverted event
    if let Some(event) = events::PositionsConverted::match_and_decode(log) {
        let event =
            pb::log::Log::NegriskAdapterPositionsConverted(pb::NegRiskAdapterPositionsConverted {
                stakeholder: event.stakeholder.to_vec(),
                market_id: event.market_id.to_vec(),
                index_set: event.index_set.to_string(),
                amount: event.amount.to_string(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // PositionsMerge event
    if let Some(event) = events::PositionsMerge::match_and_decode(log) {
        let event = pb::log::Log::NegriskAdapterPositionsMerge(pb::NegRiskAdapterPositionsMerge {
            stakeholder: event.stakeholder.to_vec(),
            condition_id: event.condition_id.to_vec(),
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // QuestionPrepared event
    if let Some(event) = events::QuestionPrepared::match_and_decode(log) {
        let event =
            pb::log::Log::NegriskAdapterQuestionPrepared(pb::NegRiskAdapterQuestionPrepared {
                market_id: event.market_id.to_vec(),
                question_id: event.question_id.to_vec(),
                index: event.index.to_string(),
                data: event.data.to_vec(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // RemovedAdmin event
    if let Some(event) = events::RemovedAdmin::match_and_decode(log) {
        let event = pb::log::Log::NegriskAdapterRemovedAdmin(pb::NegRiskAdapterRemovedAdmin {
            admin: event.admin.to_vec(),
            removed_admin: event.removed_admin.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    Ok(())
}
