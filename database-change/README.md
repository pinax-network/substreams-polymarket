# Polymarket: `DatabaseChanges`

This package converts decoded `polymarket.v1.Events` into
`sf.substreams.sink.database.v1.DatabaseChanges`.

## Database-Change Row Names

`database-change` creates rows with the following table names.

### Shared

- `blocks`

### Collateral Token

- `collateral_token_wrapped`
- `collateral_token_unwrapped`

### Conditional Tokens

- `conditionaltokens_condition_preparation`
- `conditionaltokens_condition_resolution`
- `conditionaltokens_position_split`
- `conditionaltokens_positions_merge`
- `conditionaltokens_payout_redemption`

### CTF Exchange

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

### Fee Module

- `feemodule_fee_refunded`
- `feemodule_fee_withdrawn`
- `feemodule_new_admin`
- `feemodule_removed_admin`

### Neg Risk Adapter

- `negriskadapter_market_prepared`
- `negriskadapter_new_admin`
- `negriskadapter_outcome_reported`
- `negriskadapter_payout_redemption`
- `negriskadapter_position_split`
- `negriskadapter_positions_converted`
- `negriskadapter_positions_merge`
- `negriskadapter_question_prepared`
- `negriskadapter_removed_admin`

### Safe Proxy Factory

- `safeproxyfactory_proxy_creation`
- `safeproxyfactory_proxy_creation_l2`
- `safeproxyfactory_chain_specific_proxy_creation_l2`

### UMA CTF Adapter

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
