use common::{CreateLog, CreateTransaction};
use proto::pb::safeproxyfactory::v1 as pb;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, Log};

// Event signatures for SafeProxyFactory
// ProxyCreation(address indexed proxy, address singleton)
// keccak256("ProxyCreation(address,address)")
const PROXY_CREATION_TOPIC: [u8; 32] = hex_literal::hex!(
    "4f51faf6c4561ff95f067657e43439f0f856d97c04d9ec9070a6199ad418e235"
);

// ProxyCreationL2(address indexed proxy, address singleton, bytes initializer, uint256 saltNonce)
// keccak256("ProxyCreationL2(address,address,bytes,uint256)")
const PROXY_CREATION_L2_TOPIC: [u8; 32] = hex_literal::hex!(
    "8b30b9fb5ea69b59e9b733e9c64c069137bfda6ff4c0ca3e5e5b764ec8ce3df6"
);

// ChainSpecificProxyCreationL2(address indexed proxy, address singleton, bytes initializer, uint256 saltNonce, uint256 chainId)
// keccak256("ChainSpecificProxyCreationL2(address,address,bytes,uint256,uint256)")
const CHAIN_SPECIFIC_PROXY_CREATION_L2_TOPIC: [u8; 32] = hex_literal::hex!(
    "ce0722fe61c79bd87c76fe79ea1ca6fb5c121a3f3e09a40cc7ea3626f1e23d2a"
);

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, substreams::errors::Error> {
    let mut events_output = pb::Events::default();
    let mut total_proxy_creation = 0;
    let mut total_proxy_creation_l2 = 0;
    let mut total_chain_specific_proxy_creation_l2 = 0;

    for trx in block.transactions() {
        let mut transaction = pb::Transaction::create_transaction(trx);
        for log_view in trx.receipt().logs() {
            let log = log_view.log;

            if log.topics.is_empty() {
                continue;
            }

            let topic0: [u8; 32] = match log.topics[0].as_slice().try_into() {
                Ok(t) => t,
                Err(_) => continue,
            };

            // ProxyCreation event
            if topic0 == PROXY_CREATION_TOPIC && log.topics.len() >= 2 {
                if let Some(event) = decode_proxy_creation(log) {
                    total_proxy_creation += 1;
                    let event = pb::log::Log::ProxyCreation(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // ProxyCreationL2 event
            if topic0 == PROXY_CREATION_L2_TOPIC && log.topics.len() >= 2 {
                if let Some(event) = decode_proxy_creation_l2(log) {
                    total_proxy_creation_l2 += 1;
                    let event = pb::log::Log::ProxyCreationL2(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }

            // ChainSpecificProxyCreationL2 event
            if topic0 == CHAIN_SPECIFIC_PROXY_CREATION_L2_TOPIC && log.topics.len() >= 2 {
                if let Some(event) = decode_chain_specific_proxy_creation_l2(log) {
                    total_chain_specific_proxy_creation_l2 += 1;
                    let event = pb::log::Log::ChainSpecificProxyCreationL2(event);
                    transaction.logs.push(pb::Log::create_log(log, event));
                }
                continue;
            }
        }

        if !transaction.logs.is_empty() {
            events_output.transactions.push(transaction);
        }
    }

    substreams::log::info!("Total Transactions: {}", block.transaction_traces.len());
    substreams::log::info!("Total Events: {}", events_output.transactions.len());
    substreams::log::info!("Total ProxyCreation events: {}", total_proxy_creation);
    substreams::log::info!("Total ProxyCreationL2 events: {}", total_proxy_creation_l2);
    substreams::log::info!(
        "Total ChainSpecificProxyCreationL2 events: {}",
        total_chain_specific_proxy_creation_l2
    );

    Ok(events_output)
}

fn decode_proxy_creation(log: &Log) -> Option<pb::ProxyCreation> {
    // ProxyCreation(address indexed proxy, address singleton)
    // topic[0]: event signature
    // topic[1]: proxy (indexed)
    // data: singleton

    if log.topics.len() < 2 {
        return None;
    }

    let proxy = log.topics[1][12..32].to_vec(); // address is last 20 bytes of 32-byte word
    
    if log.data.len() < 32 {
        return None;
    }
    let singleton = extract_address_from_bytes(&log.data[0..32]);

    Some(pb::ProxyCreation { proxy, singleton })
}

fn decode_proxy_creation_l2(log: &Log) -> Option<pb::ProxyCreationL2> {
    // ProxyCreationL2(address indexed proxy, address singleton, bytes initializer, uint256 saltNonce)
    // topic[0]: event signature
    // topic[1]: proxy (indexed)
    // data: singleton, initializer, saltNonce (ABI encoded)

    if log.topics.len() < 2 {
        return None;
    }

    let proxy = log.topics[1][12..32].to_vec(); // address is last 20 bytes of 32-byte word

    // Parse ABI-encoded data
    if log.data.len() < 32 {
        return None;
    }

    let singleton = extract_address_from_bytes(&log.data[0..32]);

    // The rest of the data contains initializer and saltNonce (ABI encoded)
    // For now, decode the dynamic bytes and uint256
    // This is simplified - full ABI decoding would be more complex
    let mut offset = 32;

    // Read offset to initializer
    let initializer_offset = read_u32_from_offset(&log.data, offset)? as usize;
    offset += 32;

    // Read saltNonce
    if log.data.len() < offset + 32 {
        return None;
    }
    let salt_nonce_bytes = &log.data[offset..offset + 32];
    let salt_nonce = Hex::encode(salt_nonce_bytes);

    // Read initializer length
    if log.data.len() < initializer_offset + 32 {
        return None;
    }
    let initializer_length = read_u32_from_offset(&log.data, initializer_offset)? as usize;

    // Read initializer data
    let initializer_start = initializer_offset + 32;
    if log.data.len() < initializer_start + initializer_length {
        return None;
    }
    let initializer = log.data[initializer_start..initializer_start + initializer_length].to_vec();

    Some(pb::ProxyCreationL2 {
        proxy,
        singleton,
        initializer,
        salt_nonce,
    })
}

fn decode_chain_specific_proxy_creation_l2(log: &Log) -> Option<pb::ChainSpecificProxyCreationL2> {
    // ChainSpecificProxyCreationL2(address indexed proxy, address singleton, bytes initializer, uint256 saltNonce, uint256 chainId)
    // topic[0]: event signature
    // topic[1]: proxy (indexed)
    // data: singleton, initializer, saltNonce, chainId (ABI encoded)

    if log.topics.len() < 2 {
        return None;
    }

    let proxy = log.topics[1][12..32].to_vec(); // address is last 20 bytes of 32-byte word

    // Parse ABI-encoded data
    if log.data.len() < 32 {
        return None;
    }

    let singleton = extract_address_from_bytes(&log.data[0..32]);

    // The rest contains initializer, saltNonce, chainId
    let mut offset = 32;

    // Read offset to initializer
    let initializer_offset = read_u32_from_offset(&log.data, offset)? as usize;
    offset += 32;

    // Read saltNonce
    if log.data.len() < offset + 32 {
        return None;
    }
    let salt_nonce_bytes = &log.data[offset..offset + 32];
    let salt_nonce = Hex::encode(salt_nonce_bytes);
    offset += 32;

    // Read chainId
    if log.data.len() < offset + 32 {
        return None;
    }
    let chain_id_bytes = &log.data[offset..offset + 32];
    let chain_id = Hex::encode(chain_id_bytes);

    // Read initializer length
    if log.data.len() < initializer_offset + 32 {
        return None;
    }
    let initializer_length = read_u32_from_offset(&log.data, initializer_offset)? as usize;

    // Read initializer data
    let initializer_start = initializer_offset + 32;
    if log.data.len() < initializer_start + initializer_length {
        return None;
    }
    let initializer = log.data[initializer_start..initializer_start + initializer_length].to_vec();

    Some(pb::ChainSpecificProxyCreationL2 {
        proxy,
        singleton,
        initializer,
        salt_nonce,
        chain_id,
    })
}

// Helper functions

fn extract_address_from_bytes(data: &[u8]) -> Vec<u8> {
    // Address is padded to 32 bytes, actual address is in last 20 bytes
    if data.len() >= 32 {
        data[12..32].to_vec()
    } else {
        data.to_vec()
    }
}

fn read_u32_from_offset(data: &[u8], offset: usize) -> Option<u32> {
    if data.len() < offset + 32 {
        return None;
    }
    Some(u32::from_be_bytes([
        data[offset + 28],
        data[offset + 29],
        data[offset + 30],
        data[offset + 31],
    ]))
}
