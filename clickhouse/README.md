# Polymarket: `ClickHouse`

This package imports `polymarket_database_change` and applies the ClickHouse
schema in `schema.sql`.

## Deploying

The schema is **fresh-install only** — `CREATE TABLE IF NOT EXISTS` skips
when a table already exists, so column additions/type changes between releases
won't land on a DB that already has the previous shape. Cut over by backfilling
into a new database, then flipping the consumer overlay.

The substreams sink doesn't create `CREATE MATERIALIZED VIEW … REFRESH …`
statements; the refresh MVs in `schema.sql` must be applied manually after
`substreams-sink-sql setup`. One-shot apply:

```bash
clickhouse client --host <host> --database <db> --multiquery < schema.sql
substreams-sink-sql setup --system-tables-only <dsn> <spkg>
```

(The schema-then-system-tables order works around `substreams-sink-sql`'s
schema parser, which rejects the trailing newline in `schema.sql` with
`code 62, message: Empty query`.)

## ClickHouse SQL Names

### Schema Files

- `schema.0.blocks.sql`
- `schema.0.templates.sql`
- `schema.1.collateral_token.sql`
- `schema.1.conditional_tokens.sql`
- `schema.1.ctf_exchange.sql`
- `schema.1.fee_module.sql`
- `schema.1.negrisk_adapter.sql`
- `schema.1.safe_proxy_factory.sql`
- `schema.1.uma_ctf_adapter.sql`
- `schema.2.mv.state_collateral_flow.sql`
- `schema.2.mv.state_fee.sql`
- `schema.2.mv.state_market_position.sql`
- `schema.2.mv.state_open_interest.sql`
- `schema.2.mv.state_orderbook.sql`
- `schema.2.mv.state_user_condition_position.sql`
- `schema.2.mv.state_user_position.sql`
- `schema.3.mv.state_latest_price.sql`
- `schema.3.mv.state_platform.sql`
- `schema.3.mv.state_user.sql`
- `schema.4.view.collateral_flow.sql`
- `schema.4.view.fee.sql`
- `schema.4.view.market_position.sql`
- `schema.4.view.open_interest.sql`
- `schema.4.view.orderbook.sql`
- `schema.4.view.user_condition_position.sql`
- `schema.4.view.user_position.sql`

### Raw Tables

- `blocks`
- `collateral_token_wrapped`
- `collateral_token_unwrapped`
- `conditionaltokens_condition_preparation`
- `conditionaltokens_condition_resolution`
- `conditionaltokens_position_split`
- `conditionaltokens_positions_merge`
- `conditionaltokens_payout_redemption`
- `ctfexchange_order_filled`
- `ctfexchange_fee_charged`
- `ctfexchange_new_admin`
- `ctfexchange_new_operator`
- `ctfexchange_order_cancelled`
- `ctfexchange_orders_matched`
- `ctfexchange_proxy_factory_updated`
- `ctfexchange_removed_admin`
- `ctfexchange_removed_operator`
- `ctfexchange_safe_factory_updated`
- `ctfexchange_token_registered`
- `ctfexchange_trading_paused`
- `ctfexchange_trading_unpaused`
- `ctfexchange_fee_receiver_updated`
- `ctfexchange_max_fee_rate_updated`
- `ctfexchange_order_preapproved`
- `ctfexchange_order_preapproval_invalidated`
- `ctfexchange_user_paused`
- `ctfexchange_user_unpaused`
- `ctfexchange_user_pause_block_interval_updated`
- `feemodule_fee_refunded`
- `feemodule_fee_withdrawn`
- `feemodule_new_admin`
- `feemodule_removed_admin`
- `negriskadapter_market_prepared`
- `negriskadapter_new_admin`
- `negriskadapter_outcome_reported`
- `negriskadapter_payout_redemption`
- `negriskadapter_position_split`
- `negriskadapter_positions_converted`
- `negriskadapter_positions_merge`
- `negriskadapter_question_prepared`
- `negriskadapter_removed_admin`
- `safeproxyfactory_proxy_creation`
- `safeproxyfactory_proxy_creation_l2`
- `safeproxyfactory_chain_specific_proxy_creation_l2`
- `umactfadapter_ancillary_data_updated`
- `umactfadapter_new_admin`
- `umactfadapter_question_emergency_resolved`
- `umactfadapter_question_flagged`
- `umactfadapter_question_initialized`
- `umactfadapter_question_paused`
- `umactfadapter_question_reset`
- `umactfadapter_question_resolved`
- `umactfadapter_question_unpaused`
- `umactfadapter_removed_admin`
- `umactfadapter_question_unflagged`

### State Tables

- `state_collateral_flow`
- `state_fee`
- `state_market_position`
- `state_open_interest`
- `state_orderbook`
- `state_user_condition_position`
- `state_user_position`
- `state_latest_price`
- `state_platform`
- `state_user`

### Materialized Views

Continuous MVs (sink-managed):

- `mv_state_collateral_flow_wrapped`
- `mv_state_collateral_flow_unwrapped`
- `mv_state_fee`
- `mv_state_fee_refund`
- `mv_state_open_interest_split`
- `mv_state_open_interest_merge`
- `mv_state_orderbook`
- `mv_state_latest_price`
- `mv_state_platform_orderbook`
- `mv_state_platform_oi`
- `mv_state_platform_fee`

Refresh MVs (must be applied manually after `substreams-sink-sql setup`;
`substreams-sink-sql` does not create `CREATE MATERIALIZED VIEW … REFRESH …`):

- `mv_refresh_state_user_position`
- `mv_refresh_state_market_position`
- `mv_refresh_state_user_condition_position`
- `mv_refresh_state_user`

### Query Views

- `collateral_flow`
- `fee`
- `market_position`
- `market_position_by_token`
- `open_interest`
- `orderbook`
- `user_condition_position`
- `user_condition_position_by_user`
- `user_condition_position_by_condition`
- `user_position`
- `user_position_by_user`
- `user_position_by_token`
