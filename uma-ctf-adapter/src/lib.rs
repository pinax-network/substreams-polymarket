use common::{CreateLog, CreateTransaction};
use proto::pb::uma_ctf_adapter::v1 as pb;
use substreams::Hex;
use substreams_abis::evm::polymarket::umactfadapter::v3::events as events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_ancillary_data_updated = 0;
    let mut total_new_admin = 0;
    let mut total_question_emergency_resolved = 0;
    let mut total_question_flagged = 0;
    let mut total_question_initialized = 0;
    let mut total_question_paused = 0;
    let mut total_question_reset = 0;
    let mut total_question_resolved = 0;
    let mut total_question_unpaused = 0;
    let mut total_removed_admin = 0;
    let mut total_question_unflagged = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // V3 events (same signatures as V2, so these will match both V2 and V3 contract events)

            // AncillaryDataUpdated event
            if let Some(event) = events::AncillaryDataUpdated::match_and_decode(log) {
                total_ancillary_data_updated += 1;
                let event = pb::log::Log::AncillaryDataUpdated(pb::AncillaryDataUpdated {
                    question_id: event.question_id.to_vec(),
                    owner: event.owner.to_vec(),
                    update: event.update,
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

            // QuestionEmergencyResolved event
            if let Some(event) = events::QuestionEmergencyResolved::match_and_decode(log) {
                total_question_emergency_resolved += 1;
                let event =
                    pb::log::Log::QuestionEmergencyResolved(pb::QuestionEmergencyResolved {
                        question_id: event.question_id.to_vec(),
                        payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionFlagged event
            if let Some(event) = events::QuestionFlagged::match_and_decode(log) {
                total_question_flagged += 1;
                let event = pb::log::Log::QuestionFlagged(pb::QuestionFlagged {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionInitialized event
            if let Some(event) = events::QuestionInitialized::match_and_decode(log) {
                total_question_initialized += 1;
                let event = pb::log::Log::QuestionInitialized(pb::QuestionInitialized {
                    question_id: event.question_id.to_vec(),
                    request_timestamp: event.request_timestamp.to_string(),
                    creator: event.creator.to_vec(),
                    ancillary_data: event.ancillary_data,
                    reward_token: event.reward_token.to_vec(),
                    reward: event.reward.to_string(),
                    proposal_bond: event.proposal_bond.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionPaused event
            if let Some(event) = events::QuestionPaused::match_and_decode(log) {
                total_question_paused += 1;
                let event = pb::log::Log::QuestionPaused(pb::QuestionPaused {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionReset event
            if let Some(event) = events::QuestionReset::match_and_decode(log) {
                total_question_reset += 1;
                let event = pb::log::Log::QuestionReset(pb::QuestionReset {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionResolved event
            if let Some(event) = events::QuestionResolved::match_and_decode(log) {
                total_question_resolved += 1;
                let event = pb::log::Log::QuestionResolved(pb::QuestionResolved {
                    question_id: event.question_id.to_vec(),
                    settled_price: event.settled_price.to_string(),
                    payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionUnpaused event
            if let Some(event) = events::QuestionUnpaused::match_and_decode(log) {
                total_question_unpaused += 1;
                let event = pb::log::Log::QuestionUnpaused(pb::QuestionUnpaused {
                    question_id: event.question_id.to_vec(),
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

            // QuestionUnflagged event (V3 only)
            if let Some(event) = events::QuestionUnflagged::match_and_decode(log) {
                total_question_unflagged += 1;
                let event = pb::log::Log::QuestionUnflagged(pb::QuestionUnflagged {
                    question_id: event.question_id.to_vec(),
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
    substreams::log::info!(
        "Total AncillaryDataUpdated events: {}",
        total_ancillary_data_updated
    );
    substreams::log::info!("Total NewAdmin events: {}", total_new_admin);
    substreams::log::info!(
        "Total QuestionEmergencyResolved events: {}",
        total_question_emergency_resolved
    );
    substreams::log::info!("Total QuestionFlagged events: {}", total_question_flagged);
    substreams::log::info!(
        "Total QuestionInitialized events: {}",
        total_question_initialized
    );
    substreams::log::info!("Total QuestionPaused events: {}", total_question_paused);
    substreams::log::info!("Total QuestionReset events: {}", total_question_reset);
    substreams::log::info!("Total QuestionResolved events: {}", total_question_resolved);
    substreams::log::info!(
        "Total QuestionUnpaused events: {}",
        total_question_unpaused
    );
    substreams::log::info!("Total RemovedAdmin events: {}", total_removed_admin);
    substreams::log::info!(
        "Total QuestionUnflagged events: {}",
        total_question_unflagged
    );

    Ok(events_output)
}
