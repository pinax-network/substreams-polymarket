use common::bytes_to_hex;
use proto::pb::erc20::balances::v1 as erc20_balances;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;

use crate::set_clock;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &erc20_balances::Events) {
    for (index, balance) in events.balances.iter().enumerate() {
        process_balance(tables, clock, index, balance);
    }
}

fn process_balance(
    tables: &mut Tables,
    clock: &Clock,
    index: usize,
    balance: &erc20_balances::Balance,
) {
    let key = [
        ("block_num", clock.number.to_string()),
        ("index", index.to_string()),
    ];
    let row = tables.create_row("erc20_balance", key);

    set_clock(clock, row);

    let contract = match &balance.contract {
        Some(addr) => bytes_to_hex(addr),
        None => "".to_string(),
    };
    row.set("contract", contract);
    row.set("address", bytes_to_hex(&balance.address));
    row.set("balance", &balance.balance);
}
