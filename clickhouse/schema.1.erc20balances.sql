-- ERC20 Balance --
CREATE TABLE IF NOT EXISTS erc20_balance (
    -- block --
    block_num                   UInt32,
    block_hash                  String,
    timestamp                   DateTime('UTC'),
    minute                      UInt32 COMMENT 'toRelativeMinuteNum(timestamp)',

    -- balance information --
    contract                    String COMMENT 'Token contract address',
    address                     String COMMENT 'Account address',
    balance                     UInt256 COMMENT 'Token balance'
)
ENGINE = MergeTree
ORDER BY (
    minute, timestamp, block_num
)
COMMENT 'ERC-20 Balance snapshots';
