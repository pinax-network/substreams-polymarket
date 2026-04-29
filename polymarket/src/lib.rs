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
pub fn db_out(
    clock: Clock,
    events_ctf_exchange: proto::pb::ctf_exchange::v1::Events,
    events_uma_ctf_adapter: proto::pb::uma_ctf_adapter::v1::Events,
    events_negrisk_adapter: proto::pb::negrisk_adapter::v1::Events,
    events_conditional_tokens: proto::pb::conditional_tokens::v1::Events,
    events_safe_proxy_factory: proto::pb::safe_proxy_factory::v1::Events,
    events_fee_module: proto::pb::fee_module::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    ctf_exchange::process_events(&mut tables, &clock, &events_ctf_exchange);
    uma_ctf_adapter::process_events(&mut tables, &clock, &events_uma_ctf_adapter);
    negrisk_adapter::process_events(&mut tables, &clock, &events_negrisk_adapter);
    conditional_tokens::process_events(&mut tables, &clock, &events_conditional_tokens);
    safe_proxy_factory::process_events(&mut tables, &clock, &events_safe_proxy_factory);
    fee_module::process_events(&mut tables, &clock, &events_fee_module);

    // ONLY include blocks if events are present
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
