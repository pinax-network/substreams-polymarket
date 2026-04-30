use crate::common::bytes_to_hex;
use crate::{
    logs::{log_key, set_template_log},
    set_clock,
    transactions::set_template_tx,
};
use polymarket::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &polymarket::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(polymarket::log::Log::CollateralTokenWrapped(event)) => {
                    process_wrapped(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CollateralTokenUnwrapped(event)) => {
                    process_unwrapped(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_wrapped(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CollateralTokenWrapped,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("collateral_token_wrapped", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("caller", bytes_to_hex(&event.caller));
    row.set("asset", bytes_to_hex(&event.asset));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("amount", &event.amount);
}

fn process_unwrapped(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CollateralTokenUnwrapped,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("collateral_token_unwrapped", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("caller", bytes_to_hex(&event.caller));
    row.set("asset", bytes_to_hex(&event.asset));
    row.set("to_address", bytes_to_hex(&event.to));
    row.set("amount", &event.amount);
}
