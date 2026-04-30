use crate::common::CreateLog;
use crate::pb::polymarket::v1 as pb;
use substreams::scalar::BigInt;
use substreams_abis::prediction::polymarket::{v1, v2};
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(
    log: &Log,
    transaction: &mut pb::Transaction,
) -> Result<(), substreams::errors::Error> {
    // FeeCharged event
    if let Some(event) = v1::ctfexchange::events::FeeCharged::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeFeeCharged(pb::CtfExchangeFeeCharged {
            receiver: event.receiver.to_vec(),
            token_id: Some(event.token_id.to_string()),
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::FeeCharged::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeFeeCharged(pb::CtfExchangeFeeCharged {
            receiver: event.receiver.to_vec(),
            token_id: None,
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // NewAdmin event
    if let Some(event) = v1::ctfexchange::events::NewAdmin::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeNewAdmin(pb::CtfExchangeNewAdmin {
            new_admin_address: event.new_admin_address.to_vec(),
            admin: event.admin.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // NewOperator event
    if let Some(event) = v1::ctfexchange::events::NewOperator::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeNewOperator(pb::CtfExchangeNewOperator {
            new_operator_address: event.new_operator_address.to_vec(),
            admin: event.admin.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // OrderCancelled event
    if let Some(event) = v1::ctfexchange::events::OrderCancelled::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeOrderCancelled(pb::CtfExchangeOrderCancelled {
            order_hash: event.order_hash.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // OrderFilled event
    if let Some(event) = v1::ctfexchange::events::OrderFilled::match_and_decode(log) {
        if is_self_referential_taker(&event.taker, &log.address) {
            return Ok(());
        }
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
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::OrderFilled::match_and_decode(log) {
        if is_self_referential_taker(&event.taker, &log.address) {
            return Ok(());
        }
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
        return Ok(());
    }

    // OrdersMatched event
    if let Some(event) = v1::ctfexchange::events::OrdersMatched::match_and_decode(log) {
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
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::OrdersMatched::match_and_decode(log) {
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
        return Ok(());
    }

    // ProxyFactoryUpdated event
    if let Some(event) = v1::ctfexchange::events::ProxyFactoryUpdated::match_and_decode(log) {
        let event =
            pb::log::Log::CtfExchangeProxyFactoryUpdated(pb::CtfExchangeProxyFactoryUpdated {
                old_proxy_factory: event.old_proxy_factory.to_vec(),
                new_proxy_factory: event.new_proxy_factory.to_vec(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // RemovedAdmin event
    if let Some(event) = v1::ctfexchange::events::RemovedAdmin::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeRemovedAdmin(pb::CtfExchangeRemovedAdmin {
            removed_admin: event.removed_admin.to_vec(),
            admin: event.admin.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // RemovedOperator event
    if let Some(event) = v1::ctfexchange::events::RemovedOperator::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeRemovedOperator(pb::CtfExchangeRemovedOperator {
            removed_operator: event.removed_operator.to_vec(),
            admin: event.admin.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // SafeFactoryUpdated event
    if let Some(event) = v1::ctfexchange::events::SafeFactoryUpdated::match_and_decode(log) {
        let event =
            pb::log::Log::CtfExchangeSafeFactoryUpdated(pb::CtfExchangeSafeFactoryUpdated {
                old_safe_factory: event.old_safe_factory.to_vec(),
                new_safe_factory: event.new_safe_factory.to_vec(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // TokenRegistered event
    if let Some(event) = v1::ctfexchange::events::TokenRegistered::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeTokenRegistered(pb::CtfExchangeTokenRegistered {
            condition_id: event.condition_id.to_vec(),
            token0: event.token0.to_string(),
            token1: event.token1.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // TradingPaused event
    if let Some(event) = v1::ctfexchange::events::TradingPaused::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeTradingPaused(pb::CtfExchangeTradingPaused {
            pauser: event.pauser.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    // TradingUnpaused event
    if let Some(event) = v1::ctfexchange::events::TradingUnpaused::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeTradingUnpaused(pb::CtfExchangeTradingUnpaused {
            pauser: event.pauser.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::FeeReceiverUpdated::match_and_decode(log) {
        let event =
            pb::log::Log::CtfExchangeFeeReceiverUpdated(pb::CtfExchangeFeeReceiverUpdated {
                fee_receiver: event.fee_receiver.to_vec(),
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::MaxFeeRateUpdated::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeMaxFeeRateUpdated(pb::CtfExchangeMaxFeeRateUpdated {
            max_fee_rate: event.max_fee_rate.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::OrderPreapproved::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeOrderPreapproved(pb::CtfExchangeOrderPreapproved {
            order_hash: event.order_hash.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::OrderPreapprovalInvalidated::match_and_decode(log)
    {
        let event = pb::log::Log::CtfExchangeOrderPreapprovalInvalidated(
            pb::CtfExchangeOrderPreapprovalInvalidated {
                order_hash: event.order_hash.to_vec(),
            },
        );
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::UserPaused::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeUserPaused(pb::CtfExchangeUserPaused {
            user: event.user.to_vec(),
            effective_pause_block: event.effective_pause_block.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::ctfexchange::events::UserUnpaused::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeUserUnpaused(pb::CtfExchangeUserUnpaused {
            user: event.user.to_vec(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) =
        v2::ctfexchange::events::UserPauseBlockIntervalUpdated::match_and_decode(log)
    {
        let event = pb::log::Log::CtfExchangeUserPauseBlockIntervalUpdated(
            pb::CtfExchangeUserPauseBlockIntervalUpdated {
                old_interval: event.old_interval.to_string(),
                new_interval: event.new_interval.to_string(),
            },
        );
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::collateraltoken::events::Wrapped::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeWrapped(pb::CtfExchangeWrapped {
            caller: event.caller.to_vec(),
            asset: event.asset.to_vec(),
            to: event.to.to_vec(),
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    if let Some(event) = v2::collateraltoken::events::Unwrapped::match_and_decode(log) {
        let event = pb::log::Log::CtfExchangeUnwrapped(pb::CtfExchangeUnwrapped {
            caller: event.caller.to_vec(),
            asset: event.asset.to_vec(),
            to: event.to.to_vec(),
            amount: event.amount.to_string(),
        });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    Ok(())
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
