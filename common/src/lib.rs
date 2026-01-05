use proto::pb::polymarket::v1 as pb;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format! {"0x{}", Hex::encode(bytes)}.to_string()
}

pub fn create_log(log: &Log, event: pb::log::Log) -> pb::Log {
    pb::Log {
        address: log.address.to_vec(),
        ordinal: log.ordinal,
        topics: log.topics.iter().map(|t| t.to_vec()).collect(),
        data: log.data.to_vec(),
        log: Some(event),
    }
}

pub fn create_transaction(
    trx: &substreams_ethereum::pb::eth::v2::TransactionTrace,
) -> pb::Transaction {
    let gas_price = trx
        .clone()
        .gas_price
        .unwrap_or_default()
        .with_decimal(0)
        .to_string();
    let value = trx.clone().value.unwrap_or_default().with_decimal(0);
    let to = if trx.to.is_empty() {
        None
    } else {
        Some(trx.to.to_vec())
    };
    pb::Transaction {
        from: trx.from.to_vec(),
        to,
        hash: trx.hash.to_vec(),
        nonce: trx.nonce,
        gas_price,
        gas_limit: trx.gas_limit,
        gas_used: trx.receipt().receipt.cumulative_gas_used,
        value: value.to_string(),
        logs: vec![],
    }
}
