use common::bytes_to_hex;
use proto::pb::negrisk_adapter::v1 as negrisk_adapter;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &negrisk_adapter::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(negrisk_adapter::log::Log::MarketPrepared(event)) => {
                    process_market_prepared(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::NewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::OutcomeReported(event)) => {
                    process_outcome_reported(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::PayoutRedemption(event)) => {
                    process_payout_redemption(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::PositionSplit(event)) => {
                    process_position_split(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::PositionsConverted(event)) => {
                    process_positions_converted(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::PositionsMerge(event)) => {
                    process_positions_merge(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::QuestionPrepared(event)) => {
                    process_question_prepared(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(negrisk_adapter::log::Log::RemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_market_prepared(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::MarketPrepared,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_market_prepared", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("market_id", bytes_to_hex(&event.market_id));
    row.set("oracle", bytes_to_hex(&event.oracle));
    row.set("fee_bips", &event.fee_bips);
    row.set("event_data", bytes_to_hex(&event.data));
}

fn process_new_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::NewAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_new_admin", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("new_admin_address", bytes_to_hex(&event.new_admin_address));
}

fn process_outcome_reported(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::OutcomeReported,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_outcome_reported", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("market_id", bytes_to_hex(&event.market_id));
    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("outcome", event.outcome);
}

fn process_payout_redemption(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::PayoutRedemption,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_payout_redemption", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("redeemer", bytes_to_hex(&event.redeemer));
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("amounts", event.amounts.join(","));
    row.set("payout", &event.payout);
}

fn process_position_split(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::PositionSplit,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_position_split", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("stakeholder", bytes_to_hex(&event.stakeholder));
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("amount", &event.amount);
}

fn process_positions_converted(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::PositionsConverted,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_positions_converted", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("stakeholder", bytes_to_hex(&event.stakeholder));
    row.set("market_id", bytes_to_hex(&event.market_id));
    row.set("index_set", &event.index_set);
    row.set("amount", &event.amount);
}

fn process_positions_merge(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::PositionsMerge,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_positions_merge", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("stakeholder", bytes_to_hex(&event.stakeholder));
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("amount", &event.amount);
}

fn process_question_prepared(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::QuestionPrepared,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_question_prepared", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("market_id", bytes_to_hex(&event.market_id));
    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("question_index", &event.index);
    row.set("event_data", bytes_to_hex(&event.data));
}

fn process_removed_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &negrisk_adapter::Transaction,
    log: &negrisk_adapter::Log,
    tx_index: usize,
    log_index: usize,
    event: &negrisk_adapter::RemovedAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("negriskadapter_removed_admin", key);

    set_clock(clock, row);
    set_negriskadapter_tx(tx, tx_index, row);
    set_negriskadapter_log(log, log_index, row);

    row.set("admin", bytes_to_hex(&event.admin));
    row.set("removed_admin", bytes_to_hex(&event.removed_admin));
}

fn set_negriskadapter_tx(tx: &negrisk_adapter::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_negriskadapter_log(log: &negrisk_adapter::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
