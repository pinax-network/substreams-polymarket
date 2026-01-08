use common::bytes_to_hex;
use proto::pb::feemodule::v1 as feemodule;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &feemodule::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(feemodule::log::Log::FeeRefunded(event)) => {
                    process_fee_refunded(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(feemodule::log::Log::FeeWithdrawn(event)) => {
                    process_fee_withdrawn(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(feemodule::log::Log::NewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(feemodule::log::Log::RemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_fee_refunded(
    tables: &mut Tables,
    clock: &Clock,
    tx: &feemodule::Transaction,
    log: &feemodule::Log,
    tx_index: usize,
    log_index: usize,
    event: &feemodule::FeeRefunded,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("feemodule_fee_refunded", key);

    set_clock(clock, row);
    set_feemodule_tx(tx, tx_index, row);
    set_feemodule_log(log, log_index, row);

    row.set("order_hash", bytes_to_hex(&event.order_hash));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("token_id", &event.id);
    row.set("refund", &event.refund);
    row.set("fee_charged", &event.fee_charged);
}

fn process_fee_withdrawn(
    tables: &mut Tables,
    clock: &Clock,
    tx: &feemodule::Transaction,
    log: &feemodule::Log,
    tx_index: usize,
    log_index: usize,
    event: &feemodule::FeeWithdrawn,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("feemodule_fee_withdrawn", key);

    set_clock(clock, row);
    set_feemodule_tx(tx, tx_index, row);
    set_feemodule_log(log, log_index, row);

    row.set("token", bytes_to_hex(&event.token));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("token_id", &event.id);
    row.set("amount", &event.amount);
}

fn process_new_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &feemodule::Transaction,
    log: &feemodule::Log,
    tx_index: usize,
    log_index: usize,
    event: &feemodule::NewAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("feemodule_new_admin", key);

    set_clock(clock, row);
    set_feemodule_tx(tx, tx_index, row);
    set_feemodule_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("new_admin_address", bytes_to_hex(&event.new_admin_address));
}

fn process_removed_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &feemodule::Transaction,
    log: &feemodule::Log,
    tx_index: usize,
    log_index: usize,
    event: &feemodule::RemovedAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("feemodule_removed_admin", key);

    set_clock(clock, row);
    set_feemodule_tx(tx, tx_index, row);
    set_feemodule_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("removed_admin", bytes_to_hex(&event.removed_admin));
}

fn set_feemodule_tx(tx: &feemodule::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_feemodule_log(log: &feemodule::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
