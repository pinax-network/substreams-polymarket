# Substreams Polymarket

Polymarket Substreams for Polygon. The repository is flattened around one event
protobuf and one block pass:

- `./src` decodes Polymarket contract logs into `polymarket.v1.Events`.
- `./database-change` converts those decoded events into
  `sf.substreams.sink.database.v1.DatabaseChanges`.
- `./clickhouse` provides the ClickHouse SQL schema and sink package that imports
  `database-change`.

## Packages

| Path | Package | Module | Purpose |
| ---- | ------- | ------ | ------- |
| `./substreams.yaml` | `polymarket` | `map_events` | Emits decoded Polymarket events |
| `./database-change/substreams.yaml` | `polymarket_database_change` | `db_out` | Emits database changes for SQL sinks |
| `./clickhouse/substreams.yaml` | `polymarket_clickhouse` | `db_out` | ClickHouse sink package using the SQL schema |

## Build

```bash
make pack
make -C database-change pack
make -C clickhouse pack
```

The package artifacts are written to `./spkg`.

## Event Flow

`map_events` loops through each transaction once, then each receipt log once.
Each supported contract group owns a parser that returns at most one decoded
`Log` event. Transactions with decoded logs are emitted in the root
`polymarket.v1.Events` protobuf.

`database-change/db_out` consumes `polymarket.v1.Events` and writes rows for the
raw event tables plus shared block, transaction, and log fields.

## Contract Filters

The active contract filters live in `src/lib.rs`.

### Core Trading

| Contract | Address | Version |
| -------- | ------- | ------- |
| CTF Exchange | `0xe111180000d2663c0091e4f400237545b87b996b` | V2 |
| Neg Risk CTF Exchange | `0xe2222d279d744050d28e00520010520000310f59` | V2 |
| CTF Exchange | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` | V1 |
| Neg Risk CTF Exchange | `0xc5d563a36ae78145c45a50134d48a1215220f80a` | V1 |
| Neg Risk Adapter | `0xd91e80cf2e7be2e162c6513ced06f1dd0da35296` | V2 / legacy reused |
| Legacy Neg Risk Adapter | `0xf16a3bdffb7b882e3236243e901f6c5953e2ee0d` | V1 |
| Conditional Tokens | `0x4d97dcd97ec945f40cf65f87097ace5ea0476045` | V1 / V2 unchanged |

### Collateral

| Contract | Address | Version |
| -------- | ------- | ------- |
| pUSD CollateralToken proxy | `0xc011a7e12a19f7b1f670d46f03b03f3342e82dfb` | V2 |
| pUSD CollateralToken implementation | `0x6bbcef9f7ef3b6c592c99e0f206a0de94ad0925f` | V2 |
| CollateralOnramp | `0x93070a847efef7f70739046a929d47a521f5b8ee` | V2 |
| CollateralOfframp | `0x2957922eb93258b93368531d39facca3b4dc5854` | V2 |
| PermissionedRamp | `0xebc2459ec962869ca4c0bd1e06368272732bcb08` | V2 |
| CtfCollateralAdapter | `0xada100874d00e3331d00f2007a9c336a65009718` | V2 |
| NegRiskCtfCollateralAdapter | `0xada200001000ef00d07553cee7006808f895c6f1` | V2 |

Only `CollateralToken` events are currently decoded. The other collateral
contracts are filtered so their logs can be handled explicitly as coverage grows.

### Wallets And Resolution

| Contract | Address | Version |
| -------- | ------- | ------- |
| Gnosis Safe Factory | `0xaacfeea03eb1561c4e67d661e40682bd20e3541b` | V1 / V2 |
| Polymarket Proxy Factory | `0xab45c5a4b0c941a2f231c04c3f49182e1a254052` | V1 / V2 |
| UMA Adapter | `0x6a9d222616c90fca5754cd1333cfd9b7fb6a4f74` | V1 / V2 |
| UMA Optimistic Oracle | `0xcb1822859cef82cd2eb4e6276c7916e692995130` | V2 |
| UMA CTF Adapter v3 | `0x2f5e3684cb1f318ec51b00edba38d79ac2c0aa9d` | V1 |

### Legacy Fee Modules

| Contract | Address | Version |
| -------- | ------- | ------- |
| FeeModule v2 | `0xe3f18acc55091e2c48d883fc8c8413319d4ab7b0` | V1 |
| FeeModule v0.0.1 | `0x56c79347e95530c01a2fc76e732f9566da16e113` | V1 |
| NegRiskFeeModule v2 | `0xb768891e3130f6df18214ac804d4db76c2c37730` | V1 |
| NegRiskFeeModule v1 | `0x78769d50be1763ed1ca0d5e878d93f05aabff29e` | V1 |

## Decoded Events

The protobuf includes V1 and V2 event variants for:

- CTF Exchange: order fills, matched orders, fee charges, admin/operator changes,
  proxy/safe factory updates, token registration, trading pause state, and V2
  fee receiver, max fee rate, order preapproval, user pause events.
- CollateralToken: `Wrapped` and `Unwrapped`.
- ConditionalTokens: condition preparation/resolution, splits, merges, and
  payout redemptions.
- FeeModule: fee refunds, withdrawals, and admin changes.
- NegRiskAdapter: market/question preparation, outcome reporting, position
  operations, payout redemption, and admin changes.
- SafeProxyFactory: proxy creation events.
- UmaCtfAdapter: question lifecycle, resolution, flagging, emergency resolution,
  ancillary data updates, and admin changes.

The V2 CTF Exchange schemas keep V1 compatibility by preserving optional
`maker_asset_id`, `taker_asset_id`, and `token_id` fields where only one version
emits them.
