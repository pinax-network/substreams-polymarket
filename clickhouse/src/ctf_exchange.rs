use common::bytes_to_hex;
use proto::pb::ctf_exchange::v1 as ctf_exchange;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;

use crate::{
    logs::{log_key, set_template_log},
    set_clock,
    transactions::set_template_tx,
};

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &ctf_exchange::Events) {
    for (tx_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            match &log.log {
                Some(ctf_exchange::log::Log::OrderFilled(event)) => {
                    process_order_filled(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::FeeCharged(event)) => {
                    process_fee_charged(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::NewAdmin(event)) => {
                    process_new_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::NewOperator(event)) => {
                    process_new_operator(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::OrderCancelled(event)) => {
                    process_order_cancelled(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::OrdersMatched(event)) => {
                    process_orders_matched(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::ProxyFactoryUpdated(event)) => {
                    process_proxy_factory_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::RemovedAdmin(event)) => {
                    process_removed_admin(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::RemovedOperator(event)) => {
                    process_removed_operator(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::SafeFactoryUpdated(event)) => {
                    process_safe_factory_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::TokenRegistered(event)) => {
                    process_token_registered(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::TradingPaused(event)) => {
                    process_trading_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::TradingUnpaused(event)) => {
                    process_trading_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::FeeReceiverUpdated(event)) => {
                    process_fee_receiver_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::MaxFeeRateUpdated(event)) => {
                    process_max_fee_rate_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::OrderPreapproved(event)) => {
                    process_order_preapproved(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::OrderPreapprovalInvalidated(event)) => {
                    process_order_preapproval_invalidated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::UserPaused(event)) => {
                    process_user_paused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::UserUnpaused(event)) => {
                    process_user_unpaused(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::UserPauseBlockIntervalUpdated(event)) => {
                    process_user_pause_block_interval_updated(
                        tables, clock, tx, log, tx_index, log_index, event,
                    );
                }
                Some(ctf_exchange::log::Log::Wrapped(event)) => {
                    process_wrapped(tables, clock, tx, log, tx_index, log_index, event);
                }
                Some(ctf_exchange::log::Log::Unwrapped(event)) => {
                    process_unwrapped(tables, clock, tx, log, tx_index, log_index, event);
                }
                _ => {}
            }
        }
    }
}

fn process_order_filled(
    tables: &mut Tables,
    clock: &Clock,
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::OrderFilled,
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
    row.set("maker_asset_id", maker_asset_id);
    row.set("taker_asset_id", taker_asset_id);
    if let Some(side) = event.side {
        row.set("side", side);
    }
    if let Some(token_id) = &event.token_id {
        row.set("token_id", token_id);
    }
    row.set("maker_amount_filled", &event.maker_amount_filled);
    row.set("taker_amount_filled", &event.taker_amount_filled);
    row.set("fee", &event.fee);
    if let Some(builder) = &event.builder {
        row.set("builder", bytes_to_hex(builder));
    }
    if let Some(metadata) = &event.metadata {
        row.set("metadata", bytes_to_hex(metadata));
    }
}

fn process_fee_charged(
    tables: &mut Tables,
    clock: &Clock,
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::FeeCharged,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_fee_charged", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("receiver", bytes_to_hex(&event.receiver));
    if let Some(token_id) = &event.token_id {
        row.set("token_id", token_id);
    }
    row.set("amount", &event.amount);
}

fn process_new_admin(
    tables: &mut Tables,
    clock: &Clock,
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::NewAdmin,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::NewOperator,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::OrderCancelled,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::OrdersMatched,
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
    row.set("maker_asset_id", maker_asset_id);
    row.set("taker_asset_id", taker_asset_id);
    if let Some(side) = event.side {
        row.set("side", side);
    }
    if let Some(token_id) = &event.token_id {
        row.set("token_id", token_id);
    }
    row.set("maker_amount_filled", &event.maker_amount_filled);
    row.set("taker_amount_filled", &event.taker_amount_filled);
}

fn process_proxy_factory_updated(
    tables: &mut Tables,
    clock: &Clock,
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::ProxyFactoryUpdated,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::RemovedAdmin,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::RemovedOperator,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::SafeFactoryUpdated,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::TokenRegistered,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::TradingPaused,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::TradingUnpaused,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::FeeReceiverUpdated,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::MaxFeeRateUpdated,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::OrderPreapproved,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::OrderPreapprovalInvalidated,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::UserPaused,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::UserUnpaused,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::UserPauseBlockIntervalUpdated,
) {
    let key = log_key(clock, log.ordinal);
    let row = tables.create_row("ctfexchange_user_pause_block_interval_updated", key);

    set_clock(clock, row);
    set_template_tx(tx, tx_index, row);
    set_template_log(log, log_index, row);

    row.set("old_interval", &event.old_interval);
    row.set("new_interval", &event.new_interval);
}

fn process_wrapped(
    tables: &mut Tables,
    clock: &Clock,
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::Wrapped,
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
    tx: &ctf_exchange::Transaction,
    log: &ctf_exchange::Log,
    tx_index: usize,
    log_index: usize,
    event: &ctf_exchange::Unwrapped,
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
