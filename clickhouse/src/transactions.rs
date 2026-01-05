use proto::pb::polymarket;

use crate::common::bytes_to_hex;

pub fn set_template_tx(
    tx: &polymarket::v1::Transaction,
    tx_index: usize,
    row: &mut substreams_database_change::tables::Row,
) {
    let tx_to = match &tx.to {
        Some(addr) => bytes_to_hex(&addr),
        None => "".to_string(),
    };
    row.set("tx_index", tx_index as u32);
    row.set("tx_hash", bytes_to_hex(&tx.hash));
    row.set("tx_from", bytes_to_hex(&tx.from));
    row.set("tx_to", tx_to);
    row.set("tx_nonce", tx.nonce);
    row.set("tx_gas_price", tx.gas_price.to_string());
    row.set("tx_gas_limit", tx.gas_limit);
    row.set("tx_gas_used", tx.gas_used);
    row.set("tx_value", tx.value.to_string());
}
