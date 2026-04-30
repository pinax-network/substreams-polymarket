use crate::common::bytes_to_hex;
use polymarket::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &polymarket::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(polymarket::log::Log::UmaCtfAdapterAncillaryDataUpdated(event)) => {
                    process_ancillary_data_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::UmaCtfAdapterNewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionEmergencyResolved(event)) => {
                    process_question_emergency_resolved(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionFlagged(event)) => {
                    process_question_flagged(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionInitialized(event)) => {
                    process_question_initialized(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionPaused(event)) => {
                    process_question_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionReset(event)) => {
                    process_question_reset(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionResolved(event)) => {
                    process_question_resolved(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionUnpaused(event)) => {
                    process_question_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterRemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::UmaCtfAdapterQuestionUnflagged(event)) => {
                    process_question_unflagged(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_ancillary_data_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterAncillaryDataUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_ancillary_data_updated", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("owner", bytes_to_hex(&event.owner));
    row.set("update_data", bytes_to_hex(&event.update));
}

fn process_new_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterNewAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_new_admin", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("new_admin_address", bytes_to_hex(&event.new_admin_address));
}

fn process_question_emergency_resolved(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionEmergencyResolved,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_emergency_resolved", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("payouts", event.payouts.join(","));
}

fn process_question_flagged(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionFlagged,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_flagged", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn process_question_initialized(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionInitialized,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_initialized", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("request_timestamp", &event.request_timestamp);
    row.set("creator", bytes_to_hex(&event.creator));
    row.set("ancillary_data", bytes_to_hex(&event.ancillary_data));
    row.set("reward_token", bytes_to_hex(&event.reward_token));
    row.set("reward", &event.reward);
    row.set("proposal_bond", &event.proposal_bond);
}

fn process_question_paused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionPaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_paused", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn process_question_reset(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionReset,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_reset", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn process_question_resolved(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionResolved,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_resolved", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("settled_price", &event.settled_price);
    row.set("payouts", event.payouts.join(","));
}

fn process_question_unpaused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionUnpaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_unpaused", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn process_removed_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterRemovedAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_removed_admin", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("removed_admin", bytes_to_hex(&event.removed_admin));
}

fn process_question_unflagged(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::UmaCtfAdapterQuestionUnflagged,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_unflagged", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn set_umactfadapter_tx(tx: &polymarket::Transaction, tx_index: usize, row: &mut Row) {
    let tx_to = match &tx.to {
        Some(addr) => bytes_to_hex(addr),
        None => "".to_string(),
    };
    row.set("tx_index", tx_index as u32);
    row.set("tx_hash", bytes_to_hex(&tx.hash));
    row.set("tx_from", bytes_to_hex(&tx.from));
    row.set("tx_to", tx_to);
    row.set("tx_nonce", tx.nonce);
    row.set("tx_gas_price", tx.gas_price.to_string());
    row.set("tx_gas_limit", tx.gas_limit);
    row.set("tx_gas_used", tx.gas_used);
    row.set("tx_value", tx.value.to_string());
}

fn set_umactfadapter_log(log: &polymarket::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
