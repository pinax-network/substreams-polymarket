# Polymarket ClickHouse

ClickHouse sink package for Polymarket. This package does not decode chain data
itself; it imports `../spkg/polymarket-database-change-v0.3.0.spkg` and exposes
that package's `db_out` module with the local ClickHouse schema.

## Package

| File | Value |
| ---- | ----- |
| Manifest | `clickhouse/substreams.yaml` |
| Package | `evm_clickhouse_polymarket` |
| Module | `db_out` |
| Schema | `clickhouse/schema.sql` |

## Build And Setup

```bash
make -C clickhouse pack
make -C clickhouse setup
```

`make -C clickhouse pack` builds and packs the upstream `database-change`
package first, then packs this sink into `../spkg`.

`make -C clickhouse setup` runs `substreams-sink-sql setup` against the default
local ClickHouse connection:

```text
clickhouse://default:@localhost:9000/default
```

## Schema Layout

The schema files are ordered by dependency:

- `schema.0.*`: block and shared transaction/log templates.
- `schema.1.*`: raw event tables for each decoded contract group.
- `schema.2.mv.*`: first-level aggregate states.
- `schema.3.mv.*`: derived aggregate states.
- `schema.4.view.*`: query-facing views.

## Raw Event Coverage

Raw tables exist for:

- `collateral_token`
- `conditional_tokens`
- `ctf_exchange`
- `fee_module`
- `negrisk_adapter`
- `safe_proxy_factory`
- `uma_ctf_adapter`

## Aggregates And Views

The schema currently derives ClickHouse state for:

- collateral flow
- fees
- latest price
- market position
- open interest
- order book
- platform totals
- user activity
- user condition positions
- user token positions
