use crate::common::CreateLog;
use crate::pb::polymarket::v1 as pb;
use substreams_abis::prediction::polymarket::v1::safeproxyfactory::events as safe_proxy_factory;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::Event;

pub fn parse_log(
    log: &Log,
    transaction: &mut pb::Transaction,
) -> Result<(), substreams::errors::Error> {
    // ProxyCreation event
    if let Some(event) = safe_proxy_factory::ProxyCreation::match_and_decode(log) {
        let event =
            pb::log::Log::SafeProxyFactoryProxyCreation(pb::SafeProxyFactoryProxyCreation {
                proxy: event.proxy.to_vec(),
                singleton: event.owner.to_vec(), // owner in Polymarket factory = singleton in proto
            });
        transaction.logs.push(pb::Log::create_log(log, event));
        return Ok(());
    }

    Ok(())
}
