use common::{CreateLog, CreateTransaction};
use proto::pb::negrisk_adapter::v1 as pb;
use substreams::Hex;
use substreams_abis::prediction::polymarket::v1::negriskadapter::events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_market_prepared = 0;
    let mut total_new_admin = 0;
    let mut total_outcome_reported = 0;
    let mut total_payout_redemption = 0;
    let mut total_position_split = 0;
    let mut total_positions_converted = 0;
    let mut total_positions_merge = 0;
    let mut total_question_prepared = 0;
    let mut total_removed_admin = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // MarketPrepared event
            if let Some(event) = events::MarketPrepared::match_and_decode(log) {
                total_market_prepared += 1;
                let event = pb::log::Log::MarketPrepared(pb::MarketPrepared {
                    market_id: event.market_id.to_vec(),
                    oracle: event.oracle.to_vec(),
                    fee_bips: event.fee_bips.to_string(),
                    data: event.data.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // NewAdmin event
            if let Some(event) = events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::NewAdmin(pb::NewAdmin {
                    admin: event.admin.to_vec(),
                    new_admin_address: event.new_admin_address.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // OutcomeReported event
            if let Some(event) = events::OutcomeReported::match_and_decode(log) {
                total_outcome_reported += 1;
                let event = pb::log::Log::OutcomeReported(pb::OutcomeReported {
                    market_id: event.market_id.to_vec(),
                    question_id: event.question_id.to_vec(),
                    outcome: event.outcome,
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // PayoutRedemption event
            if let Some(event) = events::PayoutRedemption::match_and_decode(log) {
                total_payout_redemption += 1;
                let event = pb::log::Log::PayoutRedemption(pb::PayoutRedemption {
                    redeemer: event.redeemer.to_vec(),
                    condition_id: event.condition_id.to_vec(),
                    amounts: event.amounts.iter().map(|a| a.to_string()).collect(),
                    payout: event.payout.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // PositionSplit event
            if let Some(event) = events::PositionSplit::match_and_decode(log) {
                total_position_split += 1;
                let event = pb::log::Log::PositionSplit(pb::PositionSplit {
                    stakeholder: event.stakeholder.to_vec(),
                    condition_id: event.condition_id.to_vec(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // PositionsConverted event
            if let Some(event) = events::PositionsConverted::match_and_decode(log) {
                total_positions_converted += 1;
                let event = pb::log::Log::PositionsConverted(pb::PositionsConverted {
                    stakeholder: event.stakeholder.to_vec(),
                    market_id: event.market_id.to_vec(),
                    index_set: event.index_set.to_string(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // PositionsMerge event
            if let Some(event) = events::PositionsMerge::match_and_decode(log) {
                total_positions_merge += 1;
                let event = pb::log::Log::PositionsMerge(pb::PositionsMerge {
                    stakeholder: event.stakeholder.to_vec(),
                    condition_id: event.condition_id.to_vec(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionPrepared event
            if let Some(event) = events::QuestionPrepared::match_and_decode(log) {
                total_question_prepared += 1;
                let event = pb::log::Log::QuestionPrepared(pb::QuestionPrepared {
                    market_id: event.market_id.to_vec(),
                    question_id: event.question_id.to_vec(),
                    index: event.index.to_string(),
                    data: event.data.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // RemovedAdmin event
            if let Some(event) = events::RemovedAdmin::match_and_decode(log) {
                total_removed_admin += 1;
                let event = pb::log::Log::RemovedAdmin(pb::RemovedAdmin {
                    admin: event.admin.to_vec(),
                    removed_admin: event.removed_admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }
        }

        if !transaction.logs.is_empty() {
            events_output.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events_output.transactions.len());
    substreams::log::info!("Total MarketPrepared events: {}", total_market_prepared);
    substreams::log::info!("Total NewAdmin events: {}", total_new_admin);
    substreams::log::info!("Total OutcomeReported events: {}", total_outcome_reported);
    substreams::log::info!("Total PayoutRedemption events: {}", total_payout_redemption);
    substreams::log::info!("Total PositionSplit events: {}", total_position_split);
    substreams::log::info!(
        "Total PositionsConverted events: {}",
        total_positions_converted
    );
    substreams::log::info!("Total PositionsMerge events: {}", total_positions_merge);
    substreams::log::info!("Total QuestionPrepared events: {}", total_question_prepared);
    substreams::log::info!("Total RemovedAdmin events: {}", total_removed_admin);

    Ok(events_output)
}
