pub mod common;
pub mod events;
pub mod pb;

mod conditional_tokens;
mod ctf_exchange;
mod fee_module;
mod logs;
mod negrisk_adapter;
mod safe_proxy_factory;
mod transactions;
mod uma_ctf_adapter;

use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn map_events(
    params: String,
    block: substreams_ethereum::pb::eth::v2::Block,
) -> Result<pb::polymarket::v1::Events, Error> {
    let mut events = pb::polymarket::v1::Events::default();
    let _source_filter = params;

    events.transactions.extend(
        events::ctf_exchange::map_events(CTF_EXCHANGE_FILTER.to_string(), block.clone())?
            .transactions,
    );
    events.transactions.extend(
        events::uma_ctf_adapter::map_events(UMA_CTF_ADAPTER_FILTER.to_string(), block.clone())?
            .transactions,
    );
    events.transactions.extend(
        events::negrisk_adapter::map_events(NEGRISK_ADAPTER_FILTER.to_string(), block.clone())?
            .transactions,
    );
    events.transactions.extend(
        events::conditional_tokens::map_events(
            CONDITIONAL_TOKENS_FILTER.to_string(),
            block.clone(),
        )?
        .transactions,
    );
    events.transactions.extend(
        events::safe_proxy_factory::map_events(
            SAFE_PROXY_FACTORY_FILTER.to_string(),
            block.clone(),
        )?
        .transactions,
    );
    events
        .transactions
        .extend(events::fee_module::map_events(FEE_MODULE_FILTER.to_string(), block)?.transactions);

    Ok(events)
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, events: pb::polymarket::v1::Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    ctf_exchange::process_events(&mut tables, &clock, &events);
    uma_ctf_adapter::process_events(&mut tables, &clock, &events);
    negrisk_adapter::process_events(&mut tables, &clock, &events);
    conditional_tokens::process_events(&mut tables, &clock, &events);
    safe_proxy_factory::process_events(&mut tables, &clock, &events);
    fee_module::process_events(&mut tables, &clock, &events);

    if !tables.tables.is_empty() {
        set_clock(
            &clock,
            tables.create_row("blocks", [("block_num", clock.number.to_string())]),
        );
    }

    substreams::log::info!("Total rows {}", tables.all_row_count());
    Ok(tables.to_database_changes())
}

pub fn set_clock(clock: &Clock, row: &mut substreams_database_change::tables::Row) {
    row.set("block_num", clock.number);
    row.set("block_hash", format!("0x{}", clock.id));
    if let Some(timestamp) = &clock.timestamp {
        row.set("timestamp", timestamp.seconds);
        row.set("minute", timestamp.seconds / 60);
    }
}

const CTF_EXCHANGE_FILTER: &str = "evt_addr:0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e || evt_addr:0xc5d563a36ae78145c45a50134d48a1215220f80a || evt_addr:0xe111180000d2663c0091e4f400237545b87b996b || evt_addr:0xe2222d279d744050d28e00520010520000310f59 || evt_addr:0xc011a7e12a19f7b1f670d46f03b03f3342e82dfb || evt_addr:0x6bbcef9f7ef3b6c592c99e0f206a0de94ad0925f || evt_addr:0x93070a847efef7f70739046a929d47a521f5b8ee || evt_addr:0x2957922eb93258b93368531d39facca3b4dc5854 || evt_addr:0xebc2459ec962869ca4c0bd1e06368272732bcb08 || evt_addr:0xada100874d00e3331d00f2007a9c336a65009718 || evt_addr:0xada200001000ef00d07553cee7006808f895c6f1";
const UMA_CTF_ADAPTER_FILTER: &str = "evt_addr:0x2f5e3684cb1f318ec51b00edba38d79ac2c0aa9d || evt_addr:0x6a9d222616c90fca5754cd1333cfd9b7fb6a4f74 || evt_addr:0xcb1822859cef82cd2eb4e6276c7916e692995130";
const NEGRISK_ADAPTER_FILTER: &str = "evt_addr:0xd91e80cf2e7be2e162c6513ced06f1dd0da35296 || evt_addr:0xf16a3bdffb7b882e3236243e901f6c5953e2ee0d";
const CONDITIONAL_TOKENS_FILTER: &str = "evt_addr:0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
const SAFE_PROXY_FACTORY_FILTER: &str = "evt_addr:0xaacfeea03eb1561c4e67d661e40682bd20e3541b || evt_addr:0xab45c5a4b0c941a2f231c04c3f49182e1a254052";
const FEE_MODULE_FILTER: &str = "evt_addr:0xe3f18acc55091e2c48d883fc8c8413319d4ab7b0 || evt_addr:0x56c79347e95530c01a2fc76e732f9566da16e113 || evt_addr:0xb768891e3130f6df18214ac804d4db76c2c37730 || evt_addr:0x78769d50be1763ed1ca0d5e878d93f05aabff29e";
