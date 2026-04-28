use common::{CreateLog, CreateTransaction};
use proto::pb::safe_proxy_factory::v1 as pb;
use substreams::Hex;
use substreams_abis::prediction::polymarket::v1::safeproxyfactory::events as safe_proxy_factory;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_proxy_creation = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // ProxyCreation event
            if let Some(event) = safe_proxy_factory::ProxyCreation::match_and_decode(log) {
                total_proxy_creation += 1;
                let event = pb::log::Log::ProxyCreation(pb::ProxyCreation {
                    proxy: event.proxy.to_vec(),
                    singleton: event.owner.to_vec(), // owner in Polymarket factory = singleton in proto
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
    substreams::log::info!("Total ProxyCreation events: {}", total_proxy_creation);

    Ok(events)
}
