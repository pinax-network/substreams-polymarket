use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Log, TransactionTrace};

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", Hex::encode(bytes))
}

pub trait CreateLog<E> {
    fn create_log(log: &Log, event: E) -> Self;
}

pub trait CreateTransaction {
    fn create_transaction(trx: &TransactionTrace) -> Self;
}

impl CreateLog<crate::pb::polymarket::v1::log::Log> for crate::pb::polymarket::v1::Log {
    fn create_log(log: &Log, event: crate::pb::polymarket::v1::log::Log) -> Self {
        Self {
            address: log.address.to_vec(),
            ordinal: log.ordinal,
            topics: log.topics.iter().map(|t| t.to_vec()).collect(),
            data: log.data.to_vec(),
            log: Some(event),
        }
    }
}

impl CreateTransaction for crate::pb::polymarket::v1::Transaction {
    fn create_transaction(trx: &TransactionTrace) -> Self {
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
        Self {
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
}
