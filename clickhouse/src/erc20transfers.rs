use common::bytes_to_hex;
use proto::pb::erc20transfers::v1 as erc20transfers;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &erc20transfers::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(erc20transfers::log::Log::Transfer(event)) => {
                    process_transfer(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc20transfers::log::Log::Approval(event)) => {
                    process_approval(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc20transfers::log::Log::Deposit(event)) => {
                    process_deposit(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(erc20transfers::log::Log::Withdrawal(event)) => {
                    process_withdrawal(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_transfer(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc20transfers::Transaction,
    log: &erc20transfers::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc20transfers::Transfer,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("erc20_transfer", key);

    set_clock(clock, row);
    set_erc20transfers_tx(tx, tx_index, row);
    set_erc20transfers_log(log, log_index, row);

    row.set("from_address", bytes_to_hex(&event.from));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("amount", &event.amount);
}

fn process_approval(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc20transfers::Transaction,
    log: &erc20transfers::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc20transfers::Approval,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("erc20_approval", key);

    set_clock(clock, row);
    set_erc20transfers_tx(tx, tx_index, row);
    set_erc20transfers_log(log, log_index, row);

    row.set("owner", bytes_to_hex(&event.owner));
    row.set("spender", bytes_to_hex(&event.spender));
    row.set("value", &event.value);
}

fn process_deposit(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc20transfers::Transaction,
    log: &erc20transfers::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc20transfers::Deposit,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("weth_deposit", key);

    set_clock(clock, row);
    set_erc20transfers_tx(tx, tx_index, row);
    set_erc20transfers_log(log, log_index, row);

    row.set("dst", bytes_to_hex(&event.dst));
    row.set("wad", &event.wad);
}

fn process_withdrawal(
    tables: &mut Tables,
    clock: &Clock,
    tx: &erc20transfers::Transaction,
    log: &erc20transfers::Log,
    tx_index: usize,
    log_index: usize,
    event: &erc20transfers::Withdrawal,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("weth_withdrawal", key);

    set_clock(clock, row);
    set_erc20transfers_tx(tx, tx_index, row);
    set_erc20transfers_log(log, log_index, row);

    row.set("src", bytes_to_hex(&event.src));
    row.set("wad", &event.wad);
}

fn set_erc20transfers_tx(tx: &erc20transfers::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_erc20transfers_log(log: &erc20transfers::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
