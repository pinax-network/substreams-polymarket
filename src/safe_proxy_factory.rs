use crate::common::bytes_to_hex;
use crate::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &polymarket::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(polymarket::log::Log::SafeProxyFactoryProxyCreation(event)) => {
                    process_proxy_creation(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::SafeProxyFactoryProxyCreationL2(event)) => {
                    process_proxy_creation_l2(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::SafeProxyFactoryChainSpecificProxyCreationL2(event)) => {
                    process_chain_specific_proxy_creation_l2(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                _ => {}
            }
        }
    }
}

fn process_proxy_creation(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::SafeProxyFactoryProxyCreation,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("safeproxyfactory_proxy_creation", key);

    set_clock(clock, row);
    set_safeproxyfactory_tx(tx, tx_index, row);
    set_safeproxyfactory_log(log, log_index, row);

    row.set("proxy", bytes_to_hex(&event.proxy));
    row.set("singleton", bytes_to_hex(&event.singleton));
}

fn process_proxy_creation_l2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::SafeProxyFactoryProxyCreationL2,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("safeproxyfactory_proxy_creation_l2", key);

    set_clock(clock, row);
    set_safeproxyfactory_tx(tx, tx_index, row);
    set_safeproxyfactory_log(log, log_index, row);

    row.set("proxy", bytes_to_hex(&event.proxy));
    row.set("singleton", bytes_to_hex(&event.singleton));
    row.set("initializer", bytes_to_hex(&event.initializer));
    row.set("salt_nonce", &event.salt_nonce);
}

fn process_chain_specific_proxy_creation_l2(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::SafeProxyFactoryChainSpecificProxyCreationL2,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("safeproxyfactory_chain_specific_proxy_creation_l2", key);

    set_clock(clock, row);
    set_safeproxyfactory_tx(tx, tx_index, row);
    set_safeproxyfactory_log(log, log_index, row);

    row.set("proxy", bytes_to_hex(&event.proxy));
    row.set("singleton", bytes_to_hex(&event.singleton));
    row.set("initializer", bytes_to_hex(&event.initializer));
    row.set("salt_nonce", &event.salt_nonce);
    row.set("chain_id", &event.chain_id);
}

fn set_safeproxyfactory_tx(tx: &polymarket::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_safeproxyfactory_log(log: &polymarket::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
