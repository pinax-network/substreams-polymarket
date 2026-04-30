use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v2::collateraltoken::events;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(log: &Log) -> Result<Option<pb::log::Log>, substreams::errors::Error> {
    if let Some(event) = events::Wrapped::match_and_decode(log) {
        let event = pb::log::Log::CollateralTokenWrapped(pb::CollateralTokenWrapped {
            caller: event.caller.to_vec(),
            asset: event.asset.to_vec(),
            to: event.to.to_vec(),
            amount: event.amount.to_string(),
        });
        return Ok(Some(event));
    }

    if let Some(event) = events::Unwrapped::match_and_decode(log) {
        let event = pb::log::Log::CollateralTokenUnwrapped(pb::CollateralTokenUnwrapped {
            caller: event.caller.to_vec(),
            asset: event.asset.to_vec(),
            to: event.to.to_vec(),
            amount: event.amount.to_string(),
        });
        return Ok(Some(event));
    }

    Ok(None)
}
