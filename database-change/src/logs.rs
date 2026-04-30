use crate::common::bytes_to_hex;
use polymarket::pb::polymarket::v1 as polymarket;
use substreams::pb::substreams::Clock;

pub fn log_key(clock: &Clock, ordinal: u64) -> [(&'static str, String); 4] {
    let seconds = clock
        .timestamp
        .as_ref()
        .expect("clock.timestamp is required")
        .seconds;
    [
        ("minute", (seconds / 60).to_string()),
        ("timestamp", seconds.to_string()),
        ("block_num", clock.number.to_string()),
        ("ordinal", ordinal.to_string()),
    ]
}

pub fn set_template_log(
    log: &polymarket::Log,
    log_index: usize,
    row: &mut substreams_database_change::tables::Row,
) {
    row.set("log_index", log_index as u32);
    row.set("log_address", bytes_to_hex(&log.address));
    row.set("log_ordinal", log.ordinal);
    row.set("log_topics", {
        let topics: Vec<String> = log.topics.iter().map(|topic| bytes_to_hex(topic)).collect();
        topics.join(",")
    });
    row.set("log_data", bytes_to_hex(&log.data));
}
