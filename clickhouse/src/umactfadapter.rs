use common::bytes_to_hex;
use proto::pb::uma_ctf_adapter::v1 as uma_ctf_adapter;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &uma_ctf_adapter::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(uma_ctf_adapter::log::Log::AncillaryDataUpdated(event)) => {
                    process_ancillary_data_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(uma_ctf_adapter::log::Log::NewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionEmergencyResolved(event)) => {
                    process_question_emergency_resolved(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(uma_ctf_adapter::log::Log::QuestionFlagged(event)) => {
                    process_question_flagged(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionInitialized(event)) => {
                    process_question_initialized(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(uma_ctf_adapter::log::Log::QuestionPaused(event)) => {
                    process_question_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionReset(event)) => {
                    process_question_reset(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionResolved(event)) => {
                    process_question_resolved(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionUnpaused(event)) => {
                    process_question_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::RemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(uma_ctf_adapter::log::Log::QuestionUnflagged(event)) => {
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::AncillaryDataUpdated,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::NewAdmin,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionEmergencyResolved,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionFlagged,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionInitialized,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionPaused,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionReset,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionResolved,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionUnpaused,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::RemovedAdmin,
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
    tx: &uma_ctf_adapter::Transaction,
    log: &uma_ctf_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &uma_ctf_adapter::QuestionUnflagged,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("umactfadapter_question_unflagged", key);

    set_clock(clock, row);
    set_umactfadapter_tx(tx, tx_index, row);
    set_umactfadapter_log(log, log_index, row);

    row.set("question_id", bytes_to_hex(&event.question_id));
}

fn set_umactfadapter_tx(tx: &uma_ctf_adapter::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_umactfadapter_log(log: &uma_ctf_adapter::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
