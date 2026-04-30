# Polymarket Database Changes

Database-change package for Polymarket. This crate consumes the root
`polymarket` event package and converts decoded `polymarket.v1.Events` into
`sf.substreams.sink.database.v1.DatabaseChanges`.

## Package

| File | Value |
| ---- | ----- |
| Manifest | `database-change/substreams.yaml` |
| Package | `polymarket_database_change` |
| Module | `db_out` |
| Input | `events:map_events` from `../spkg/polymarket-v0.3.0.spkg` |
| Dependency | `substreams-database-change` v4.0.0 |

## Build

```bash
make -C database-change pack
```

The pack target first rebuilds the root `polymarket` event package, then writes
the database-change package to `../spkg`.

## Logic

`db_out` receives the block clock and decoded Polymarket events. Each processor
module writes rows for one contract group:

- `collateral_token`
- `conditional_tokens`
- `ctf_exchange`
- `fee_module`
- `negrisk_adapter`
- `safe_proxy_factory`
- `uma_ctf_adapter`

If any event rows are produced for a block, `db_out` also writes a row to
`blocks` with block number, hash, timestamp, and minute.
