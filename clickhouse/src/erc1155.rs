use common::bytes_to_hex;
use proto::pb::erc1155::v1 as erc1155;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &erc1155::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(erc1155::log::Log::TransferSingle(event)) => {
                    process_transfer_single(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc1155::log::Log::TransferBatch(event)) => {
                    process_transfer_batch(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc1155::log::Log::ApprovalForAll(event)) => {
                    process_approval_for_all(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc1155::log::Log::Uri(event)) => {
                    process_uri(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_transfer_single(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc1155::Transaction,
    log: &erc1155::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc1155::TransferSingle,
) {
    let key = log_key(clock);
    let row = tables.create_row("erc1155_transfer_single", key);

    set_clock(clock, row);
    set_erc1155_tx(tx, tx_index, row);
    set_erc1155_log(log, log_index, row);

    row.set("operator", bytes_to_hex(&event.operator));
    row.set("from_address", bytes_to_hex(&event.from));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("token_id", &event.id);
    row.set("amount", &event.value);
}

fn process_transfer_batch(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc1155::Transaction,
    log: &erc1155::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc1155::TransferBatch,
) {
    let key = log_key(clock);
    let row = tables.create_row("erc1155_transfer_batch", key);

    set_clock(clock, row);
    set_erc1155_tx(tx, tx_index, row);
    set_erc1155_log(log, log_index, row);

    row.set("operator", bytes_to_hex(&event.operator));
    row.set("from_address", bytes_to_hex(&event.from));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("token_ids", event.ids.join(","));
    row.set("amounts", event.values.join(","));
}

fn process_approval_for_all(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc1155::Transaction,
    log: &erc1155::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc1155::ApprovalForAll,
) {
    let key = log_key(clock);
    let row = tables.create_row("erc1155_approval_for_all", key);

    set_clock(clock, row);
    set_erc1155_tx(tx, tx_index, row);
    set_erc1155_log(log, log_index, row);

    row.set("account", bytes_to_hex(&event.account));
    row.set("operator", bytes_to_hex(&event.operator));
    row.set("approved", event.approved);
}

fn process_uri(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc1155::Transaction,
    log: &erc1155::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc1155::Uri,
) {
    let key = log_key(clock);
    let row = tables.create_row("erc1155_uri", key);

    set_clock(clock, row);
    set_erc1155_tx(tx, tx_index, row);
    set_erc1155_log(log, log_index, row);

    row.set("uri_value", &event.value);
    row.set("token_id", &event.id);
}

fn set_erc1155_tx(tx: &erc1155::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_erc1155_log(log: &erc1155::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
