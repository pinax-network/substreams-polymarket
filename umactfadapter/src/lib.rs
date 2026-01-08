use common::{CreateLog, CreateTransaction};
use proto::pb::umactfadapter::v1 as pb;
use substreams_abis::evm::polymarket::umactfadapter::v2::events as v2_events;
use substreams_abis::evm::polymarket::umactfadapter::v3::events as v3_events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
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

            // Try V3 events first (V3 has QuestionUnflagged that V2 doesn't have)

            // AncillaryDataUpdated event (same in V2 and V3)
            if let Some(event) = v3_events::AncillaryDataUpdated::match_and_decode(log) {
                total_ancillary_data_updated += 1;
                let event = pb::log::Log::AncillaryDataUpdated(pb::AncillaryDataUpdated {
                    question_id: event.question_id.to_vec(),
                    owner: event.owner.to_vec(),
                    update: event.update,
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // NewAdmin event (same in V2 and V3)
            if let Some(event) = v3_events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::NewAdmin(pb::NewAdmin {
                    admin: event.admin.to_vec(),
                    new_admin_address: event.new_admin_address.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionEmergencyResolved event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionEmergencyResolved::match_and_decode(log) {
                total_question_emergency_resolved += 1;
                let event =
                    pb::log::Log::QuestionEmergencyResolved(pb::QuestionEmergencyResolved {
                        question_id: event.question_id.to_vec(),
                        payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionFlagged event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionFlagged::match_and_decode(log) {
                total_question_flagged += 1;
                let event = pb::log::Log::QuestionFlagged(pb::QuestionFlagged {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionInitialized event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionInitialized::match_and_decode(log) {
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

            // QuestionPaused event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionPaused::match_and_decode(log) {
                total_question_paused += 1;
                let event = pb::log::Log::QuestionPaused(pb::QuestionPaused {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionReset event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionReset::match_and_decode(log) {
                total_question_reset += 1;
                let event = pb::log::Log::QuestionReset(pb::QuestionReset {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionResolved event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionResolved::match_and_decode(log) {
                total_question_resolved += 1;
                let event = pb::log::Log::QuestionResolved(pb::QuestionResolved {
                    question_id: event.question_id.to_vec(),
                    settled_price: event.settled_price.to_string(),
                    payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionUnpaused event (same in V2 and V3)
            if let Some(event) = v3_events::QuestionUnpaused::match_and_decode(log) {
                total_question_unpaused += 1;
                let event = pb::log::Log::QuestionUnpaused(pb::QuestionUnpaused {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // RemovedAdmin event (same in V2 and V3)
            if let Some(event) = v3_events::RemovedAdmin::match_and_decode(log) {
                total_removed_admin += 1;
                let event = pb::log::Log::RemovedAdmin(pb::RemovedAdmin {
                    admin: event.admin.to_vec(),
                    removed_admin: event.removed_admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionUnflagged event (V3 only)
            if let Some(event) = v3_events::QuestionUnflagged::match_and_decode(log) {
                total_question_unflagged += 1;
                let event = pb::log::Log::QuestionUnflagged(pb::QuestionUnflagged {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // Fall back to V2 events for any that might not have matched V3
            // AncillaryDataUpdated event (V2)
            if let Some(event) = v2_events::AncillaryDataUpdated::match_and_decode(log) {
                total_ancillary_data_updated += 1;
                let event = pb::log::Log::AncillaryDataUpdated(pb::AncillaryDataUpdated {
                    question_id: event.question_id.to_vec(),
                    owner: event.owner.to_vec(),
                    update: event.update,
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // NewAdmin event (V2)
            if let Some(event) = v2_events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::NewAdmin(pb::NewAdmin {
                    admin: event.admin.to_vec(),
                    new_admin_address: event.new_admin_address.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionEmergencyResolved event (V2)
            if let Some(event) = v2_events::QuestionEmergencyResolved::match_and_decode(log) {
                total_question_emergency_resolved += 1;
                let event =
                    pb::log::Log::QuestionEmergencyResolved(pb::QuestionEmergencyResolved {
                        question_id: event.question_id.to_vec(),
                        payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionFlagged event (V2)
            if let Some(event) = v2_events::QuestionFlagged::match_and_decode(log) {
                total_question_flagged += 1;
                let event = pb::log::Log::QuestionFlagged(pb::QuestionFlagged {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionInitialized event (V2)
            if let Some(event) = v2_events::QuestionInitialized::match_and_decode(log) {
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

            // QuestionPaused event (V2)
            if let Some(event) = v2_events::QuestionPaused::match_and_decode(log) {
                total_question_paused += 1;
                let event = pb::log::Log::QuestionPaused(pb::QuestionPaused {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionReset event (V2)
            if let Some(event) = v2_events::QuestionReset::match_and_decode(log) {
                total_question_reset += 1;
                let event = pb::log::Log::QuestionReset(pb::QuestionReset {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionResolved event (V2)
            if let Some(event) = v2_events::QuestionResolved::match_and_decode(log) {
                total_question_resolved += 1;
                let event = pb::log::Log::QuestionResolved(pb::QuestionResolved {
                    question_id: event.question_id.to_vec(),
                    settled_price: event.settled_price.to_string(),
                    payouts: event.payouts.iter().map(|p| p.to_string()).collect(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // QuestionUnpaused event (V2)
            if let Some(event) = v2_events::QuestionUnpaused::match_and_decode(log) {
                total_question_unpaused += 1;
                let event = pb::log::Log::QuestionUnpaused(pb::QuestionUnpaused {
                    question_id: event.question_id.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // RemovedAdmin event (V2)
            if let Some(event) = v2_events::RemovedAdmin::match_and_decode(log) {
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
            events.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events.transactions.len());
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

    Ok(events)
}
