use common::{CreateLog, CreateTransaction};
use proto::pb::feemodule::v1 as pb;
use substreams::Hex;
use substreams_abis::evm::polymarket::feemodule::events as events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_fee_refunded = 0;
    let mut total_fee_withdrawn = 0;
    let mut total_new_admin = 0;
    let mut total_removed_admin = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // FeeRefunded event
            if let Some(event) = events::FeeRefunded::match_and_decode(log) {
                total_fee_refunded += 1;
                let event = pb::log::Log::FeeRefunded(pb::FeeRefunded {
                    order_hash: event.order_hash.to_vec(),
                    to: event.to.to_vec(),
                    id: event.id.to_string(),
                    refund: event.refund.to_string(),
                    fee_charged: event.fee_charged.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // FeeWithdrawn event
            if let Some(event) = events::FeeWithdrawn::match_and_decode(log) {
                total_fee_withdrawn += 1;
                let event = pb::log::Log::FeeWithdrawn(pb::FeeWithdrawn {
                    token: event.token.to_vec(),
                    to: event.to.to_vec(),
                    id: event.id.to_string(),
                    amount: event.amount.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // NewAdmin event
            if let Some(event) = events::NewAdmin::match_and_decode(log) {
                total_new_admin += 1;
                let event = pb::log::Log::NewAdmin(pb::NewAdmin {
                    admin: event.admin.to_vec(),
                    new_admin_address: event.new_admin_address.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // RemovedAdmin event
            if let Some(event) = events::RemovedAdmin::match_and_decode(log) {
                total_removed_admin += 1;
                let event = pb::log::Log::RemovedAdmin(pb::RemovedAdmin {
                    admin: event.admin.to_vec(),
                    removed_admin: event.removed_admin.to_vec(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }
        }

        if !transaction.logs.is_empty() {
            events_output.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events_output.transactions.len());
    substreams::log::info!("Total FeeRefunded events: {}", total_fee_refunded);
    substreams::log::info!("Total FeeWithdrawn events: {}", total_fee_withdrawn);
    substreams::log::info!("Total NewAdmin events: {}", total_new_admin);
    substreams::log::info!("Total RemovedAdmin events: {}", total_removed_admin);

    Ok(events_output)
}
