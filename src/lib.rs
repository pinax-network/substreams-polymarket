pub mod common;
pub mod pb;

mod collateral_token;
mod conditional_tokens;
mod ctf_exchange;
mod fee_module;
mod negrisk_adapter;
mod safe_proxy_factory;
mod uma_ctf_adapter;

use crate::common::{CreateLog, CreateTransaction};
use substreams::errors::Error;
use substreams::Hex;

#[substreams::handlers::map]
pub fn map_events(
    block: substreams_ethereum::pb::eth::v2::Block,
) -> Result<pb::polymarket::v1::Events, Error> {
    let mut events = pb::polymarket::v1::Events::default();

    for trx in block.transactions() {
        let mut transaction = pb::polymarket::v1::Transaction::create_transaction(trx);

        for log_view in trx.receipt().logs() {
            let log = log_view.log;
            let address = format!("0x{}", Hex::encode(&log.address));

            let event = if matches_any(&address, CORE_TRADING_CTF_EXCHANGE_ADDRESSES) {
                ctf_exchange::parse_log(log)?
            } else if matches_any(&address, COLLATERAL_TOKEN_ADDRESSES) {
                collateral_token::parse_log(log)?
            } else if matches_any(&address, CORE_TRADING_NEGRISK_ADAPTER_ADDRESSES) {
                negrisk_adapter::parse_log(log)?
            } else if matches_any(&address, CORE_TRADING_CONDITIONAL_TOKENS_ADDRESSES) {
                conditional_tokens::parse_log(log)?
            } else if matches_any(&address, WALLET_FACTORY_ADDRESSES) {
                safe_proxy_factory::parse_log(log)?
            } else if matches_any(&address, RESOLUTION_ADDRESSES) {
                uma_ctf_adapter::parse_log(log)?
            } else if matches_any(&address, FEE_MODULE_ADDRESSES) {
                fee_module::parse_log(log)?
            } else if matches_any(&address, COLLATERAL_CONTRACT_ADDRESSES) {
                None
            } else {
                None
            };

            if let Some(event) = event {
                transaction
                    .logs
                    .push(pb::polymarket::v1::Log::create_log(log, event));
            };
        }

        if !transaction.logs.is_empty() {
            events.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events.transactions.len());

    Ok(events)
}

fn matches_any(address: &str, candidates: &[&str]) -> bool {
    candidates.iter().any(|candidate| *candidate == address)
}

// Core Trading Contracts
const CORE_TRADING_CTF_EXCHANGE_ADDRESSES: &[&str] = &[
    "0xe111180000d2663c0091e4f400237545b87b996b", // V2 CTF Exchange
    "0xe2222d279d744050d28e00520010520000310f59", // V2 Neg Risk CTF Exchange
    "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e", // V1 CTF Exchange, confirmed in eab4a690
    "0xc5d563a36ae78145c45a50134d48a1215220f80a", // V1 Neg Risk CTF Exchange, confirmed in eab4a690
];

const CORE_TRADING_NEGRISK_ADAPTER_ADDRESSES: &[&str] = &[
    "0xd91e80cf2e7be2e162c6513ced06f1dd0da35296", // V2 docs current; V1 legacy reused, confirmed in eab4a690
    "0xf16a3bdffb7b882e3236243e901f6c5953e2ee0d", // V1 legacy NegRiskAdapter, confirmed in eab4a690
];

const CORE_TRADING_CONDITIONAL_TOKENS_ADDRESSES: &[&str] = &[
    "0x4d97dcd97ec945f40cf65f87097ace5ea0476045", // V2 docs current; unchanged from V1, confirmed in eab4a690
];

// Collateral Contracts
const COLLATERAL_TOKEN_ADDRESSES: &[&str] = &[
    "0xc011a7e12a19f7b1f670d46f03b03f3342e82dfb", // V2 pUSD CollateralToken proxy
    "0x6bbcef9f7ef3b6c592c99e0f206a0de94ad0925f", // V2 pUSD CollateralToken implementation
];

const COLLATERAL_CONTRACT_ADDRESSES: &[&str] = &[
    "0x93070a847efef7f70739046a929d47a521f5b8ee", // V2 CollateralOnramp
    "0x2957922eb93258b93368531d39facca3b4dc5854", // V2 CollateralOfframp
    "0xebc2459ec962869ca4c0bd1e06368272732bcb08", // V2 PermissionedRamp
    "0xada100874d00e3331d00f2007a9c336a65009718", // V2 CtfCollateralAdapter
    "0xada200001000ef00d07553cee7006808f895c6f1", // V2 NegRiskCtfCollateralAdapter
];

// Wallet Factory Contracts
const WALLET_FACTORY_ADDRESSES: &[&str] = &[
    "0xaacfeea03eb1561c4e67d661e40682bd20e3541b", // V2 docs current; V1 Gnosis Safe Factory, confirmed in eab4a690
    "0xab45c5a4b0c941a2f231c04c3f49182e1a254052", // V2 Polymarket Proxy Factory
];

// Resolution Contracts
const RESOLUTION_ADDRESSES: &[&str] = &[
    "0x6a9d222616c90fca5754cd1333cfd9b7fb6a4f74", // V2 UMA Adapter; V1 legacy reused, confirmed in eab4a690
    "0xcb1822859cef82cd2eb4e6276c7916e692995130", // V2 UMA Optimistic Oracle
    "0x2f5e3684cb1f318ec51b00edba38d79ac2c0aa9d", // V1 UMA CTF Adapter v3, confirmed in eab4a690
];

// Legacy Fee Module Contracts
const FEE_MODULE_ADDRESSES: &[&str] = &[
    "0xe3f18acc55091e2c48d883fc8c8413319d4ab7b0", // V1 FeeModule v2, confirmed in eab4a690
    "0x56c79347e95530c01a2fc76e732f9566da16e113", // V1 FeeModule v0.0.1, confirmed in eab4a690
    "0xb768891e3130f6df18214ac804d4db76c2c37730", // V1 NegRiskFeeModule v2, confirmed in eab4a690
    "0x78769d50be1763ed1ca0d5e878d93f05aabff29e", // V1 NegRiskFeeModule v1, confirmed in eab4a690
];
