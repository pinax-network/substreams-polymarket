mod ctfexchange;
mod erc1155;
mod logs;
mod transactions;
mod umactfadapter;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    events_polymarket: proto::pb::polymarket::v1::Events,
    events_erc1155: proto::pb::erc1155::v1::Events,
    events_umactfadapter: proto::pb::umactfadapter::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    ctfexchange::process_events(&mut tables, &clock, &events_polymarket);
    erc1155::process_events(&mut tables, &clock, &events_erc1155);
    umactfadapter::process_events(&mut tables, &clock, &events_umactfadapter);

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
