use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v1::feemodule::events;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(log: &Log) -> Result<Option<pb::log::Log>, substreams::errors::Error> {
    // FeeRefunded event
    if let Some(event) = events::FeeRefunded::match_and_decode(log) {
        let event = pb::log::Log::FeeModuleFeeRefunded(pb::FeeModuleFeeRefunded {
            order_hash: event.order_hash.to_vec(),
            to: event.to.to_vec(),
            id: event.id.to_string(),
            refund: event.refund.to_string(),
            fee_charged: event.fee_charged.to_string(),
        });
        return Ok(Some(event));
    }

    // FeeWithdrawn event
    if let Some(event) = events::FeeWithdrawn::match_and_decode(log) {
        let event = pb::log::Log::FeeModuleFeeWithdrawn(pb::FeeModuleFeeWithdrawn {
            token: event.token.to_vec(),
            to: event.to.to_vec(),
            id: event.id.to_string(),
            amount: event.amount.to_string(),
        });
        return Ok(Some(event));
    }

    // NewAdmin event
    if let Some(event) = events::NewAdmin::match_and_decode(log) {
        let event = pb::log::Log::FeeModuleNewAdmin(pb::FeeModuleNewAdmin {
            admin: event.admin.to_vec(),
            new_admin_address: event.new_admin_address.to_vec(),
        });
        return Ok(Some(event));
    }

    // RemovedAdmin event
    if let Some(event) = events::RemovedAdmin::match_and_decode(log) {
        let event = pb::log::Log::FeeModuleRemovedAdmin(pb::FeeModuleRemovedAdmin {
            admin: event.admin.to_vec(),
            removed_admin: event.removed_admin.to_vec(),
        });
        return Ok(Some(event));
    }

    Ok(None)
}
