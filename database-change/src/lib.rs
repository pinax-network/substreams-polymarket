mod collateral_token;
mod common;
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
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    events: polymarket::pb::polymarket::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    ctf_exchange::process_events(&mut tables, &clock, &events);
    collateral_token::process_events(&mut tables, &clock, &events);
    uma_ctf_adapter::process_events(&mut tables, &clock, &events);
    negrisk_adapter::process_events(&mut tables, &clock, &events);
    conditional_tokens::process_events(&mut tables, &clock, &events);
    safe_proxy_factory::process_events(&mut tables, &clock, &events);
    fee_module::process_events(&mut tables, &clock, &events);

    if tables.all_row_count() > 0 {
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
