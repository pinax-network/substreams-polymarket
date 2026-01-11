use common::{CreateLog, CreateTransaction};
use proto::pb::fpmm_factory::v1 as pb;
use substreams::Hex;
use substreams_abis::evm::polymarket::fixedproductmarketmakerfactory::events as events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_fixed_product_market_maker_creation = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // FixedProductMarketMakerCreation event
            if let Some(event) = events::FixedProductMarketMakerCreation::match_and_decode(log) {
                total_fixed_product_market_maker_creation += 1;
                let event = pb::log::Log::FixedProductMarketMakerCreation(pb::FixedProductMarketMakerCreation {
                    creator: event.creator.to_vec(),
                    fixed_product_market_maker: event.fixed_product_market_maker.to_vec(),
                    conditional_tokens: event.conditional_tokens.to_vec(),
                    collateral_token: event.collateral_token.to_vec(),
                    condition_ids: event.condition_ids.iter().map(|id| id.to_vec()).collect(),
                    fee: event.fee.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
            }
        }

        if !transaction.logs.is_empty() {
            events_output.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events_output.transactions.len());
    substreams::log::info!(
        "Total FixedProductMarketMakerCreation events: {}",
        total_fixed_product_market_maker_creation
    );

    Ok(events_output)
}
