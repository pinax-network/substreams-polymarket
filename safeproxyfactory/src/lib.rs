use common::{CreateLog, CreateTransaction};
use proto::pb::safeproxyfactory::v1 as pb;
use substreams_abis::evm::polymarket::safeproxyfactory::events as safeproxyfactory;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events = pb::Events::default();
    let mut total_proxy_creation = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // ProxyCreation event
            if let Some(event) = safeproxyfactory::ProxyCreation::match_and_decode(log) {
                total_proxy_creation += 1;
                let event = pb::log::Log::ProxyCreation(pb::ProxyCreation {
                    proxy: event.proxy.to_vec(),
                    owner: event.owner.to_vec(),
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
