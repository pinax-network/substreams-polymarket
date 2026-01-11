use common::{CreateLog, CreateTransaction};
use proto::pb::fpmm::v1 as pb;
use substreams::Hex;
use substreams_abis::evm::polymarket::fixedproductmarketmaker::events as events;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
fn map_events(params: String, block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let matcher = substreams::expr_matcher(&params);
    let mut total_fpmm_funding_added = 0;
    let mut total_fpmm_funding_removed = 0;
    let mut total_fpmm_buy = 0;
    let mut total_fpmm_sell = 0;
    let mut total_transfer = 0;
    let mut total_approval = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            // Skip logs that don't match the filter (if params provided)
            if !matcher.matches_keys(&vec![format!("evt_addr:0x{}", Hex::encode(&log.address))]) {
                continue;
            }

            // FPMMFundingAdded event
            if let Some(event) = events::FpmmFundingAdded::match_and_decode(log) {
                total_fpmm_funding_added += 1;
                let event = pb::log::Log::FpmmFundingAdded(pb::FpmmFundingAdded {
                    funder: event.funder.to_vec(),
                    amounts_added: event.amounts_added.iter().map(|a| a.to_string()).collect(),
                    shares_minted: event.shares_minted.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // FPMMFundingRemoved event
            if let Some(event) = events::FpmmFundingRemoved::match_and_decode(log) {
                total_fpmm_funding_removed += 1;
                let event = pb::log::Log::FpmmFundingRemoved(pb::FpmmFundingRemoved {
                    funder: event.funder.to_vec(),
                    amounts_removed: event.amounts_removed.iter().map(|a| a.to_string()).collect(),
                    collateral_removed_from_fee_pool: event
                        .collateral_removed_from_fee_pool
                        .to_string(),
                    shares_burnt: event.shares_burnt.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // FPMMBuy event
            if let Some(event) = events::FpmmBuy::match_and_decode(log) {
                total_fpmm_buy += 1;
                let event = pb::log::Log::FpmmBuy(pb::FpmmBuy {
                    buyer: event.buyer.to_vec(),
                    investment_amount: event.investment_amount.to_string(),
                    fee_amount: event.fee_amount.to_string(),
                    outcome_index: event.outcome_index.to_string(),
                    outcome_tokens_bought: event.outcome_tokens_bought.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // FPMMSell event
            if let Some(event) = events::FpmmSell::match_and_decode(log) {
                total_fpmm_sell += 1;
                let event = pb::log::Log::FpmmSell(pb::FpmmSell {
                    seller: event.seller.to_vec(),
                    return_amount: event.return_amount.to_string(),
                    fee_amount: event.fee_amount.to_string(),
                    outcome_index: event.outcome_index.to_string(),
                    outcome_tokens_sold: event.outcome_tokens_sold.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // Transfer event
            if let Some(event) = events::Transfer::match_and_decode(log) {
                total_transfer += 1;
                let event = pb::log::Log::Transfer(pb::Transfer {
                    from: event.from.to_vec(),
                    to: event.to.to_vec(),
                    value: event.value.to_string(),
                });
                transaction.logs.push(pb::Log::create_log(log, event));
                continue;
            }

            // Approval event
            if let Some(event) = events::Approval::match_and_decode(log) {
                total_approval += 1;
                let event = pb::log::Log::Approval(pb::Approval {
                    owner: event.owner.to_vec(),
                    spender: event.spender.to_vec(),
                    value: event.value.to_string(),
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
    substreams::log::info!(
        "Total FPMMFundingAdded events: {}",
        total_fpmm_funding_added
    );
    substreams::log::info!(
        "Total FPMMFundingRemoved events: {}",
        total_fpmm_funding_removed
    );
    substreams::log::info!("Total FPMMBuy events: {}", total_fpmm_buy);
    substreams::log::info!("Total FPMMSell events: {}", total_fpmm_sell);
    substreams::log::info!("Total Transfer events: {}", total_transfer);
    substreams::log::info!("Total Approval events: {}", total_approval);

    Ok(events_output)
}
