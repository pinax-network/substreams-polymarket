mod store;
use common::{CreateLog, CreateTransaction};
use proto::pb::ctf_exchange::v1 as pb;
use substreams::Hex;
use substreams_abis::evm::polymarket::ctfexchange as polymarket;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_fee_charged = 0;
    let mut total_new_admin = 0;
    let mut total_new_operator = 0;
    let mut total_order_cancelled = 0;
    let mut total_order_filled = 0;
    let mut total_orders_matched = 0;
    let mut total_proxy_factory_updated = 0;
    let mut total_removed_admin = 0;
    let mut total_removed_operator = 0;
    let mut total_safe_factory_updated = 0;
    let mut total_token_registered = 0;
    let mut total_trading_paused = 0;
    let mut total_trading_unpaused = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // FeeCharged event
            if let Some(event) = polymarket::events::FeeCharged::match_and_decode(log) {
                total_fee_charged += 1;
                let event = pb::log::Log::FeeCharged(pb::FeeCharged {
                    receiver: event.receiver.to_vec(),
                    token_id: event.token_id.to_string(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // NewAdmin event
            if let Some(event) = polymarket::events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::NewAdmin(pb::NewAdmin {
                    new_admin_address: event.new_admin_address.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // NewOperator event
            if let Some(event) = polymarket::events::NewOperator::match_and_decode(log) {
                total_new_operator += 1;
                let event = pb::log::Log::NewOperator(pb::NewOperator {
                    new_operator_address: event.new_operator_address.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrderCancelled event
            if let Some(event) = polymarket::events::OrderCancelled::match_and_decode(log) {
                total_order_cancelled += 1;
                let event = pb::log::Log::OrderCancelled(pb::OrderCancelled {
                    order_hash: event.order_hash.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrderFilled event
            if let Some(event) = polymarket::events::OrderFilled::match_and_decode(log) {
                total_order_filled += 1;
                let event = pb::log::Log::OrderFilled(pb::OrderFilled {
                    order_hash: event.order_hash.to_vec(),
                    maker: event.maker.to_vec(),
                    taker: event.taker.to_vec(),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    fee: event.fee.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrdersMatched event
            if let Some(event) = polymarket::events::OrdersMatched::match_and_decode(log) {
                total_orders_matched += 1;
                let event = pb::log::Log::OrdersMatched(pb::OrdersMatched {
                    taker_order_hash: event.taker_order_hash.to_vec(),
                    taker_order_maker: event.taker_order_maker.to_vec(),
                    maker_asset_id: event.maker_asset_id.to_string(),
                    taker_asset_id: event.taker_asset_id.to_string(),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // ProxyFactoryUpdated event
            if let Some(event) = polymarket::events::ProxyFactoryUpdated::match_and_decode(log) {
                total_proxy_factory_updated += 1;
                let event = pb::log::Log::ProxyFactoryUpdated(pb::ProxyFactoryUpdated {
                    old_proxy_factory: event.old_proxy_factory.to_vec(),
                    new_proxy_factory: event.new_proxy_factory.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // RemovedAdmin event
            if let Some(event) = polymarket::events::RemovedAdmin::match_and_decode(log) {
                total_removed_admin += 1;
                let event = pb::log::Log::RemovedAdmin(pb::RemovedAdmin {
                    removed_admin: event.removed_admin.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // RemovedOperator event
            if let Some(event) = polymarket::events::RemovedOperator::match_and_decode(log) {
                total_removed_operator += 1;
                let event = pb::log::Log::RemovedOperator(pb::RemovedOperator {
                    removed_operator: event.removed_operator.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // SafeFactoryUpdated event
            if let Some(event) = polymarket::events::SafeFactoryUpdated::match_and_decode(log) {
                total_safe_factory_updated += 1;
                let event = pb::log::Log::SafeFactoryUpdated(pb::SafeFactoryUpdated {
                    old_safe_factory: event.old_safe_factory.to_vec(),
                    new_safe_factory: event.new_safe_factory.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TokenRegistered event
            if let Some(event) = polymarket::events::TokenRegistered::match_and_decode(log) {
                total_token_registered += 1;
                let event = pb::log::Log::TokenRegistered(pb::TokenRegistered {
                    condition_id: event.condition_id.to_vec(),
                    token0: event.token0.to_string(),
                    token1: event.token1.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TradingPaused event
            if let Some(event) = polymarket::events::TradingPaused::match_and_decode(log) {
                total_trading_paused += 1;
                let event = pb::log::Log::TradingPaused(pb::TradingPaused {
                    pauser: event.pauser.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TradingUnpaused event
            if let Some(event) = polymarket::events::TradingUnpaused::match_and_decode(log) {
                total_trading_unpaused += 1;
                let event = pb::log::Log::TradingUnpaused(pb::TradingUnpaused {
                    pauser: event.pauser.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }
        }

        if !transaction.logs.is_empty() {
            events.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events.transactions.len());
    substreams::log::info!("Total FeeCharged events: {}", total_fee_charged);
    substreams::log::info!("Total NewAdmin events: {}", total_new_admin);
    substreams::log::info!("Total NewOperator events: {}", total_new_operator);
    substreams::log::info!("Total OrderCancelled events: {}", total_order_cancelled);
    substreams::log::info!("Total OrderFilled events: {}", total_order_filled);
    substreams::log::info!("Total OrdersMatched events: {}", total_orders_matched);
    substreams::log::info!(
        "Total ProxyFactoryUpdated events: {}",
        total_proxy_factory_updated
    );
    substreams::log::info!("Total RemovedAdmin events: {}", total_removed_admin);
    substreams::log::info!("Total RemovedOperator events: {}", total_removed_operator);
    substreams::log::info!(
        "Total SafeFactoryUpdated events: {}",
        total_safe_factory_updated
    );
    substreams::log::info!("Total TokenRegistered events: {}", total_token_registered);
    substreams::log::info!("Total TradingPaused events: {}", total_trading_paused);
    substreams::log::info!("Total TradingUnpaused events: {}", total_trading_unpaused);
    Ok(events)
}
