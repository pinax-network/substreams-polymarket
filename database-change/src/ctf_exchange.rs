use crate::common::bytes_to_hex;
use polymarket::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;

use crate::{
    logs::{log_key, set_template_log},
    set_clock,
    transactions::set_template_tx,
};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &polymarket::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(polymarket::log::Log::CtfExchangeOrderFilled(event)) => {
                    process_order_filled(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeFeeCharged(event)) => {
                    process_fee_charged(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeNewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeNewOperator(event)) => {
                    process_new_operator(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeOrderCancelled(event)) => {
                    process_order_cancelled(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeOrdersMatched(event)) => {
                    process_orders_matched(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeProxyFactoryUpdated(event)) => {
                    process_proxy_factory_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::CtfExchangeRemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeRemovedOperator(event)) => {
                    process_removed_operator(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeSafeFactoryUpdated(event)) => {
                    process_safe_factory_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::CtfExchangeTokenRegistered(event)) => {
                    process_token_registered(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeTradingPaused(event)) => {
                    process_trading_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeTradingUnpaused(event)) => {
                    process_trading_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeFeeReceiverUpdated(event)) => {
                    process_fee_receiver_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::CtfExchangeMaxFeeRateUpdated(event)) => {
                    process_max_fee_rate_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::CtfExchangeOrderPreapproved(event)) => {
                    process_order_preapproved(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeOrderPreapprovalInvalidated(event)) => {
                    process_order_preapproval_invalidated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(polymarket::log::Log::CtfExchangeUserPaused(event)) => {
                    process_user_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeUserUnpaused(event)) => {
                    process_user_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(polymarket::log::Log::CtfExchangeUserPauseBlockIntervalUpdated(event)) => {
                    process_user_pause_block_interval_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                _ => {}
            }
        }
    }
}

fn process_order_filled(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeOrderFilled,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_order_filled", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("order_hash", bytes_to_hex(&event.order_hash));
    row.set("maker", bytes_to_hex(&event.maker));
    row.set("taker", bytes_to_hex(&event.taker));
    let (maker_asset_id, taker_asset_id) = legacy_asset_ids(
        event.side,
        event.token_id.as_deref(),
        event.maker_asset_id.as_deref(),
        event.taker_asset_id.as_deref(),
    );
    row.set("maker_asset_id", &maker_asset_id);
    row.set("taker_asset_id", taker_asset_id.clone());
    row.set(
        "side",
        event.side.unwrap_or_else(|| legacy_side(&maker_asset_id)),
    );
    row.set(
        "token_id",
        event
            .token_id
            .as_deref()
            .unwrap_or_else(|| legacy_token_id(&maker_asset_id, &taker_asset_id)),
    );
    row.set("maker_amount_filled", &event.maker_amount_filled);
    row.set("taker_amount_filled", &event.taker_amount_filled);
    row.set("fee", &event.fee);
    row.set(
        "builder",
        event
            .builder
            .as_deref()
            .map(bytes_to_hex)
            .unwrap_or_default(),
    );
    row.set(
        "metadata",
        event
            .metadata
            .as_deref()
            .map(bytes_to_hex)
            .unwrap_or_default(),
    );
}

fn process_fee_charged(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeFeeCharged,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_fee_charged", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("receiver", bytes_to_hex(&event.receiver));
    row.set("token_id", event.token_id.as_deref().unwrap_or("0"));
    row.set("amount", &event.amount);
}

fn process_new_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeNewAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_new_admin", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("new_admin_address", bytes_to_hex(&event.new_admin_address));
    row.set("admin", bytes_to_hex(&event.admin));
}

fn process_new_operator(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeNewOperator,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_new_operator", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set(
        "new_operator_address",
        bytes_to_hex(&event.new_operator_address),
    );
    row.set("admin", bytes_to_hex(&event.admin));
}

fn process_order_cancelled(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeOrderCancelled,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_order_cancelled", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("order_hash", bytes_to_hex(&event.order_hash));
}

fn process_orders_matched(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeOrdersMatched,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_orders_matched", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("taker_order_hash", bytes_to_hex(&event.taker_order_hash));
    row.set("taker_order_maker", bytes_to_hex(&event.taker_order_maker));
    let (maker_asset_id, taker_asset_id) = legacy_asset_ids(
        event.side,
        event.token_id.as_deref(),
        event.maker_asset_id.as_deref(),
        event.taker_asset_id.as_deref(),
    );
    row.set("maker_asset_id", &maker_asset_id);
    row.set("taker_asset_id", taker_asset_id.clone());
    row.set(
        "side",
        event.side.unwrap_or_else(|| legacy_side(&maker_asset_id)),
    );
    row.set(
        "token_id",
        event
            .token_id
            .as_deref()
            .unwrap_or_else(|| legacy_token_id(&maker_asset_id, &taker_asset_id)),
    );
    row.set("maker_amount_filled", &event.maker_amount_filled);
    row.set("taker_amount_filled", &event.taker_amount_filled);
}

fn process_proxy_factory_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeProxyFactoryUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_proxy_factory_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("old_proxy_factory", bytes_to_hex(&event.old_proxy_factory));
    row.set("new_proxy_factory", bytes_to_hex(&event.new_proxy_factory));
}

fn process_removed_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeRemovedAdmin,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_removed_admin", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("removed_admin", bytes_to_hex(&event.removed_admin));
    row.set("admin", bytes_to_hex(&event.admin));
}

fn process_removed_operator(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeRemovedOperator,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_removed_operator", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("removed_operator", bytes_to_hex(&event.removed_operator));
    row.set("admin", bytes_to_hex(&event.admin));
}

fn process_safe_factory_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeSafeFactoryUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_safe_factory_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("old_safe_factory", bytes_to_hex(&event.old_safe_factory));
    row.set("new_safe_factory", bytes_to_hex(&event.new_safe_factory));
}

fn process_token_registered(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeTokenRegistered,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_token_registered", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("condition_id", bytes_to_hex(&event.condition_id));
    row.set("token0", &event.token0);
    row.set("token1", &event.token1);
}

fn process_trading_paused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeTradingPaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_trading_paused", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("pauser", bytes_to_hex(&event.pauser));
}

fn process_trading_unpaused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeTradingUnpaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_trading_unpaused", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("pauser", bytes_to_hex(&event.pauser));
}

fn process_fee_receiver_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeFeeReceiverUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_fee_receiver_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("fee_receiver", bytes_to_hex(&event.fee_receiver));
}

fn process_max_fee_rate_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeMaxFeeRateUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_max_fee_rate_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("max_fee_rate", &event.max_fee_rate);
}

fn process_order_preapproved(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeOrderPreapproved,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_order_preapproved", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("order_hash", bytes_to_hex(&event.order_hash));
}

fn process_order_preapproval_invalidated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeOrderPreapprovalInvalidated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_order_preapproval_invalidated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("order_hash", bytes_to_hex(&event.order_hash));
}

fn process_user_paused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeUserPaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_user_paused", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("user", bytes_to_hex(&event.user));
    row.set("effective_pause_block", &event.effective_pause_block);
}

fn process_user_unpaused(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeUserUnpaused,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_user_unpaused", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("user", bytes_to_hex(&event.user));
}

fn process_user_pause_block_interval_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &polymarket::Transaction,
    log: &polymarket::Log,
    tx_index: usize,
    log_index: usize,
    event: &polymarket::CtfExchangeUserPauseBlockIntervalUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_user_pause_block_interval_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("old_interval", &event.old_interval);
    row.set("new_interval", &event.new_interval);
}

fn legacy_asset_ids<'a>(
    side: Option<u32>,
    token_id: Option<&'a str>,
    maker_asset_id: Option<&'a str>,
    taker_asset_id: Option<&'a str>,
) -> (String, String) {
    if let (Some(side), Some(token_id)) = (side, token_id) {
        if side == 0 {
            return ("0".to_string(), token_id.to_string());
        }
        return (token_id.to_string(), "0".to_string());
    }

    (
        maker_asset_id.unwrap_or("0").to_string(),
        taker_asset_id.unwrap_or("0").to_string(),
    )
}

fn legacy_side(maker_asset_id: &str) -> u32 {
    if maker_asset_id == "0" {
        0
    } else {
        1
    }
}

fn legacy_token_id<'a>(maker_asset_id: &'a str, taker_asset_id: &'a str) -> &'a str {
    if maker_asset_id == "0" {
        taker_asset_id
    } else {
        maker_asset_id
    }
}
