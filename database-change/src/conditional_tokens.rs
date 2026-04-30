use crate::common::bytes_to_hex;
use polymarket::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};

use crate::{logs::log_key, set_clock};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &polymarket::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(polymarket::log::Log::ConditionalTokensConditionPreparation(event)) => {
                    process_condition_preparation(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::ConditionalTokensConditionResolution(event)) => {
                    process_condition_resolution(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::ConditionalTokensPositionSplit(event)) => {
                    process_position_split(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::ConditionalTokensPositionsMerge(event)) => {
                    process_positions_merge(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::ConditionalTokensPayoutRedemption(event)) => {
                    process_payout_redemption(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_condition_preparation(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::ConditionalTokensConditionPreparation,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("conditionaltokens_condition_preparation", key);

    set_clock(clock, row);
    set_conditionaltokens_tx(tx, tx_index, row);
    set_conditionaltokens_log(log, log_index, row);

    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("oracle", bytes_to_hex(&event.oracle));
    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("outcome_slot_count", &event.outcome_slot_count);
}

fn process_condition_resolution(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::ConditionalTokensConditionResolution,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("conditionaltokens_condition_resolution", key);

    set_clock(clock, row);
    set_conditionaltokens_tx(tx, tx_index, row);
    set_conditionaltokens_log(log, log_index, row);

    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("oracle", bytes_to_hex(&event.oracle));
    row.set("question_id", bytes_to_hex(&event.question_id));
    row.set("outcome_slot_count", &event.outcome_slot_count);
    row.set("payout_numerators", event.payout_numerators.join(","));
}

fn process_position_split(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::ConditionalTokensPositionSplit,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("conditionaltokens_position_split", key);

    set_clock(clock, row);
    set_conditionaltokens_tx(tx, tx_index, row);
    set_conditionaltokens_log(log, log_index, row);

    row.set("stakeholder", bytes_to_hex(&event.stakeholder));
    row.set("collateral_token", bytes_to_hex(&event.collateral_token));
    row.set(
        "parent_collection_id",
        bytes_to_hex(&event.parent_collection_id),
    );
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("partition", event.partition.join(","));
    row.set("amount", &event.amount);
}

fn process_positions_merge(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::ConditionalTokensPositionsMerge,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("conditionaltokens_positions_merge", key);

    set_clock(clock, row);
    set_conditionaltokens_tx(tx, tx_index, row);
    set_conditionaltokens_log(log, log_index, row);

    row.set("stakeholder", bytes_to_hex(&event.stakeholder));
    row.set("collateral_token", bytes_to_hex(&event.collateral_token));
    row.set(
        "parent_collection_id",
        bytes_to_hex(&event.parent_collection_id),
    );
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("partition", event.partition.join(","));
    row.set("amount", &event.amount);
}

fn process_payout_redemption(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::ConditionalTokensPayoutRedemption,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("conditionaltokens_payout_redemption", key);

    set_clock(clock, row);
    set_conditionaltokens_tx(tx, tx_index, row);
    set_conditionaltokens_log(log, log_index, row);

    row.set("redeemer", bytes_to_hex(&event.redeemer));
    row.set("collateral_token", bytes_to_hex(&event.collateral_token));
    row.set(
        "parent_collection_id",
        bytes_to_hex(&event.parent_collection_id),
    );
    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("index_sets", event.index_sets.join(","));
    row.set("payout", &event.payout);
}

fn set_conditionaltokens_tx(tx: &polymarket::Transaction, tx_index: usize, row: &mut Row) {
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

fn set_conditionaltokens_log(log: &polymarket::Log, log_index: usize, row: &mut Row) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
