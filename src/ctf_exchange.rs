use crate::common::{CreateLog, CreateTransaction};
use crate::pb::polymarket::v1 as pb;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_abis::prediction::polymarket::{v1, v2};
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

pub fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_fee_charged = 0;
    let mut total_new_admin = 0;
    let mut total_new_operator = 0;
    let mut total_order_cancelled = 0;
    let mut total_order_filled = 0;
    let mut total_order_filled_self_reference_skipped = 0;
    let mut total_orders_matched = 0;
    let mut total_proxy_factory_updated = 0;
    let mut total_removed_admin = 0;
    let mut total_removed_operator = 0;
    let mut total_safe_factory_updated = 0;
    let mut total_token_registered = 0;
    let mut total_trading_paused = 0;
    let mut total_trading_unpaused = 0;
    let mut total_fee_receiver_updated = 0;
    let mut total_max_fee_rate_updated = 0;
    let mut total_order_preapproved = 0;
    let mut total_order_preapproval_invalidated = 0;
    let mut total_user_paused = 0;
    let mut total_user_unpaused = 0;
    let mut total_user_pause_block_interval_updated = 0;
    let mut total_wrapped = 0;
    let mut total_unwrapped = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // FeeCharged event
            if let Some(event) = v1::ctfexchange::events::FeeCharged::match_and_decode(log) {
                total_fee_charged += 1;
                let event = pb::log::Log::CtfExchangeFeeCharged(pb::CtfExchangeFeeCharged {
                    receiver: event.receiver.to_vec(),
                    token_id: Some(event.token_id.to_string()),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::FeeCharged::match_and_decode(log) {
                total_fee_charged += 1;
                let event = pb::log::Log::CtfExchangeFeeCharged(pb::CtfExchangeFeeCharged {
                    receiver: event.receiver.to_vec(),
                    token_id: None,
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // NewAdmin event
            if let Some(event) = v1::ctfexchange::events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::CtfExchangeNewAdmin(pb::CtfExchangeNewAdmin {
                    new_admin_address: event.new_admin_address.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // NewOperator event
            if let Some(event) = v1::ctfexchange::events::NewOperator::match_and_decode(log) {
                total_new_operator += 1;
                let event = pb::log::Log::CtfExchangeNewOperator(pb::CtfExchangeNewOperator {
                    new_operator_address: event.new_operator_address.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrderCancelled event
            if let Some(event) = v1::ctfexchange::events::OrderCancelled::match_and_decode(log) {
                total_order_cancelled += 1;
                let event =
                    pb::log::Log::CtfExchangeOrderCancelled(pb::CtfExchangeOrderCancelled {
                        order_hash: event.order_hash.to_vec(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrderFilled event
            if let Some(event) = v1::ctfexchange::events::OrderFilled::match_and_decode(log) {
                if is_self_referential_taker(&event.taker, &log.address) {
                    total_order_filled_self_reference_skipped += 1;
                    continue;
                }

                total_order_filled += 1;
                let event = pb::log::Log::CtfExchangeOrderFilled(pb::CtfExchangeOrderFilled {
                    order_hash: event.order_hash.to_vec(),
                    maker: event.maker.to_vec(),
                    taker: event.taker.to_vec(),
                    maker_asset_id: Some(event.maker_asset_id.to_string()),
                    taker_asset_id: Some(event.taker_asset_id.to_string()),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    fee: event.fee.to_string(),
                    side: None,
                    token_id: None,
                    builder: None,
                    metadata: None,
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::OrderFilled::match_and_decode(log) {
                if is_self_referential_taker(&event.taker, &log.address) {
                    total_order_filled_self_reference_skipped += 1;
                    continue;
                }

                total_order_filled += 1;
                let event = pb::log::Log::CtfExchangeOrderFilled(pb::CtfExchangeOrderFilled {
                    order_hash: event.order_hash.to_vec(),
                    maker: event.maker.to_vec(),
                    taker: event.taker.to_vec(),
                    maker_asset_id: None,
                    taker_asset_id: None,
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    fee: event.fee.to_string(),
                    side: Some(side_to_u32(&event.side)?),
                    token_id: Some(event.token_id.to_string()),
                    builder: Some(event.builder.to_vec()),
                    metadata: Some(event.metadata.to_vec()),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // OrdersMatched event
            if let Some(event) = v1::ctfexchange::events::OrdersMatched::match_and_decode(log) {
                total_orders_matched += 1;
                let event = pb::log::Log::CtfExchangeOrdersMatched(pb::CtfExchangeOrdersMatched {
                    taker_order_hash: event.taker_order_hash.to_vec(),
                    taker_order_maker: event.taker_order_maker.to_vec(),
                    maker_asset_id: Some(event.maker_asset_id.to_string()),
                    taker_asset_id: Some(event.taker_asset_id.to_string()),
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    side: None,
                    token_id: None,
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::OrdersMatched::match_and_decode(log) {
                total_orders_matched += 1;
                let event = pb::log::Log::CtfExchangeOrdersMatched(pb::CtfExchangeOrdersMatched {
                    taker_order_hash: event.taker_order_hash.to_vec(),
                    taker_order_maker: event.taker_order_maker.to_vec(),
                    maker_asset_id: None,
                    taker_asset_id: None,
                    maker_amount_filled: event.maker_amount_filled.to_string(),
                    taker_amount_filled: event.taker_amount_filled.to_string(),
                    side: Some(side_to_u32(&event.side)?),
                    token_id: Some(event.token_id.to_string()),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // ProxyFactoryUpdated event
            if let Some(event) = v1::ctfexchange::events::ProxyFactoryUpdated::match_and_decode(log)
            {
                total_proxy_factory_updated += 1;
                let event = pb::log::Log::CtfExchangeProxyFactoryUpdated(
                    pb::CtfExchangeProxyFactoryUpdated {
                        old_proxy_factory: event.old_proxy_factory.to_vec(),
                        new_proxy_factory: event.new_proxy_factory.to_vec(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // RemovedAdmin event
            if let Some(event) = v1::ctfexchange::events::RemovedAdmin::match_and_decode(log) {
                total_removed_admin += 1;
                let event = pb::log::Log::CtfExchangeRemovedAdmin(pb::CtfExchangeRemovedAdmin {
                    removed_admin: event.removed_admin.to_vec(),
                    admin: event.admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // RemovedOperator event
            if let Some(event) = v1::ctfexchange::events::RemovedOperator::match_and_decode(log) {
                total_removed_operator += 1;
                let event =
                    pb::log::Log::CtfExchangeRemovedOperator(pb::CtfExchangeRemovedOperator {
                        removed_operator: event.removed_operator.to_vec(),
                        admin: event.admin.to_vec(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // SafeFactoryUpdated event
            if let Some(event) = v1::ctfexchange::events::SafeFactoryUpdated::match_and_decode(log)
            {
                total_safe_factory_updated += 1;
                let event = pb::log::Log::CtfExchangeSafeFactoryUpdated(
                    pb::CtfExchangeSafeFactoryUpdated {
                        old_safe_factory: event.old_safe_factory.to_vec(),
                        new_safe_factory: event.new_safe_factory.to_vec(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TokenRegistered event
            if let Some(event) = v1::ctfexchange::events::TokenRegistered::match_and_decode(log) {
                total_token_registered += 1;
                let event =
                    pb::log::Log::CtfExchangeTokenRegistered(pb::CtfExchangeTokenRegistered {
                        condition_id: event.condition_id.to_vec(),
                        token0: event.token0.to_string(),
                        token1: event.token1.to_string(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TradingPaused event
            if let Some(event) = v1::ctfexchange::events::TradingPaused::match_and_decode(log) {
                total_trading_paused += 1;
                let event = pb::log::Log::CtfExchangeTradingPaused(pb::CtfExchangeTradingPaused {
                    pauser: event.pauser.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            // TradingUnpaused event
            if let Some(event) = v1::ctfexchange::events::TradingUnpaused::match_and_decode(log) {
                total_trading_unpaused += 1;
                let event =
                    pb::log::Log::CtfExchangeTradingUnpaused(pb::CtfExchangeTradingUnpaused {
                        pauser: event.pauser.to_vec(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::FeeReceiverUpdated::match_and_decode(log)
            {
                total_fee_receiver_updated += 1;
                let event = pb::log::Log::CtfExchangeFeeReceiverUpdated(
                    pb::CtfExchangeFeeReceiverUpdated {
                        fee_receiver: event.fee_receiver.to_vec(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::MaxFeeRateUpdated::match_and_decode(log) {
                total_max_fee_rate_updated += 1;
                let event =
                    pb::log::Log::CtfExchangeMaxFeeRateUpdated(pb::CtfExchangeMaxFeeRateUpdated {
                        max_fee_rate: event.max_fee_rate.to_string(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::OrderPreapproved::match_and_decode(log) {
                total_order_preapproved += 1;
                let event =
                    pb::log::Log::CtfExchangeOrderPreapproved(pb::CtfExchangeOrderPreapproved {
                        order_hash: event.order_hash.to_vec(),
                    });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) =
                v2::ctfexchange::events::OrderPreapprovalInvalidated::match_and_decode(log)
            {
                total_order_preapproval_invalidated += 1;
                let event = pb::log::Log::CtfExchangeOrderPreapprovalInvalidated(
                    pb::CtfExchangeOrderPreapprovalInvalidated {
                        order_hash: event.order_hash.to_vec(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::UserPaused::match_and_decode(log) {
                total_user_paused += 1;
                let event = pb::log::Log::CtfExchangeUserPaused(pb::CtfExchangeUserPaused {
                    user: event.user.to_vec(),
                    effective_pause_block: event.effective_pause_block.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::ctfexchange::events::UserUnpaused::match_and_decode(log) {
                total_user_unpaused += 1;
                let event = pb::log::Log::CtfExchangeUserUnpaused(pb::CtfExchangeUserUnpaused {
                    user: event.user.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) =
                v2::ctfexchange::events::UserPauseBlockIntervalUpdated::match_and_decode(log)
            {
                total_user_pause_block_interval_updated += 1;
                let event = pb::log::Log::CtfExchangeUserPauseBlockIntervalUpdated(
                    pb::CtfExchangeUserPauseBlockIntervalUpdated {
                        old_interval: event.old_interval.to_string(),
                        new_interval: event.new_interval.to_string(),
                    },
                );
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::collateraltoken::events::Wrapped::match_and_decode(log) {
                total_wrapped += 1;
                let event = pb::log::Log::CtfExchangeWrapped(pb::CtfExchangeWrapped {
                    caller: event.caller.to_vec(),
                    asset: event.asset.to_vec(),
                    to: event.to.to_vec(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }

            if let Some(event) = v2::collateraltoken::events::Unwrapped::match_and_decode(log) {
                total_unwrapped += 1;
                let event = pb::log::Log::CtfExchangeUnwrapped(pb::CtfExchangeUnwrapped {
                    caller: event.caller.to_vec(),
                    asset: event.asset.to_vec(),
                    to: event.to.to_vec(),
                    amount: event.amount.to_string(),
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
    substreams::log::info!(
        "Total self-referential OrderFilled events skipped: {}",
        total_order_filled_self_reference_skipped
    );
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
    substreams::log::info!(
        "Total FeeReceiverUpdated events: {}",
        total_fee_receiver_updated
    );
    substreams::log::info!(
        "Total MaxFeeRateUpdated events: {}",
        total_max_fee_rate_updated
    );
    substreams::log::info!("Total OrderPreapproved events: {}", total_order_preapproved);
    substreams::log::info!(
        "Total OrderPreapprovalInvalidated events: {}",
        total_order_preapproval_invalidated
    );
    substreams::log::info!("Total UserPaused events: {}", total_user_paused);
    substreams::log::info!("Total UserUnpaused events: {}", total_user_unpaused);
    substreams::log::info!(
        "Total UserPauseBlockIntervalUpdated events: {}",
        total_user_pause_block_interval_updated
    );
    substreams::log::info!("Total Wrapped events: {}", total_wrapped);
    substreams::log::info!("Total Unwrapped events: {}", total_unwrapped);
    Ok(events)
}

fn is_self_referential_taker(taker: &[u8], log_address: &[u8]) -> bool {
    taker == log_address
}

fn side_to_u32(side: &BigInt) -> Result<u32, substreams::errors::Error> {
    let value = u64::try_from(side)
        .map_err(|err| substreams::errors::Error::msg(format!("invalid V2 side: {err}")))?;

    u32::try_from(value)
        .map_err(|err| substreams::errors::Error::msg(format!("V2 side out of range: {err}")))
}
