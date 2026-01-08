# Substreams Polymarket

## Packages

- [x] Polymarket
  - [x] CTF Exchange (`0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`)
  - [x] Neg Risk CTF Exchange (`0xC5d563A36AE78145C45a50134d48A1215220f80a`)
- [x] ERC1155
  - [x] Conditional Tokens (`0x4D97DCd97eC945f40cF65F87097ACe5EA0476045`)
- [x] NegRiskAdapter (`0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296`)

## Data Features

- [ ] Orders
- [ ] Positions
- [ ] Activity
- [ ] Open Interest
- [ ] PNL

| Smart contract name                                 | EVM address                                |
| --------------------------------------------------- | ------------------------------------------ |
| USDC (collateral)                                   | 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174 |
| Conditional Tokens (CTF / ERC-1155)                 | 0x4d97dcd97ec945f40cf65f87097ace5ea0476045 |
| CTF Exchange (CTFExchange)                          | 0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E |
| Neg Risk CTF Exchange (NegRiskCTFExchange)          | 0xC5d563A36AE78145C45a50134d48A1215220f80a |
| Neg Risk Adapter (current)                          | 0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296 |
| Proxy Wallet Factory                                | 0xaB45c5A4B0c941a2F231C04C3f49182e1A254052 |
| Safe Factory                                        | 0xaacfeea03eb1561c4e67d661e40682bd20e3541b |
| FeeModule (CTF Exchange fee module, v2)             | 0xE3f18aCc55091e2c48d883fc8C8413319d4Ab7b0 |
| FeeModule (CTF Exchange fee module, v0.0.1 / older) | 0x56C79347e95530c01A2FC76E732f9566dA16E113 |
| NegRiskFeeModule (v2)                               | 0xb768891e3130f6df18214ac804d4db76c2c37730 |
| NegRiskFeeModule (v1 / older)                       | 0x78769D50Be1763ed1CA0D5E878D93f05aabff29e |
| NegRiskOperator                                     | 0xf09a3e199e815e1b5d0bf1ef45006875009edb04 |
| NegRiskAdapter (early/legacy deployment)            | 0xf16a3BdFFB7B882E3236243E901f6c5953E2EE0d |
| UmaCtfAdapter v3.0 (one deployment)                 | 0x2f5e3684cb1f318ec51b00edba38d79ac2c0aa9d |
| UmaCtfAdapter v2.0                                  | 0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74 |
| NegRisk UmaCtfAdapter (separate deployment)         | 0x2F5e3684cb1F318ec51b00Edba38d79Ac2c0aA9d |

## Clickhouse Events

The following tables describe the events available in Clickhouse for each Substreams module.

### CTFExchange

Events from the CTF Exchange contract for order matching and trading operations.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| OrderFilled | `ctfexchange_order_filled` | Emitted when an order is filled (swap events) |
| FeeCharged | `ctfexchange_fee_charged` | Emitted when a fee is charged on a trade |
| NewAdmin | `ctfexchange_new_admin` | Emitted when a new admin is added |
| NewOperator | `ctfexchange_new_operator` | Emitted when a new operator is added |
| OrderCancelled | `ctfexchange_order_cancelled` | Emitted when an order is cancelled |
| OrdersMatched | `ctfexchange_orders_matched` | Emitted when orders are matched |
| ProxyFactoryUpdated | `ctfexchange_proxy_factory_updated` | Emitted when the proxy factory address is updated |
| RemovedAdmin | `ctfexchange_removed_admin` | Emitted when an admin is removed |
| RemovedOperator | `ctfexchange_removed_operator` | Emitted when an operator is removed |
| SafeFactoryUpdated | `ctfexchange_safe_factory_updated` | Emitted when the safe factory address is updated |
| TokenRegistered | `ctfexchange_token_registered` | Emitted when a new token pair is registered |
| TradingPaused | `ctfexchange_trading_paused` | Emitted when trading is paused |
| TradingUnpaused | `ctfexchange_trading_unpaused` | Emitted when trading is unpaused |

### ConditionalTokens

Events from the Conditional Tokens (CTF) contract for condition preparation and resolution.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| ConditionPreparation | `conditionaltokens_condition_preparation` | Emitted upon the successful preparation of a condition |
| ConditionResolution | `conditionaltokens_condition_resolution` | Emitted when a condition is resolved |
| PositionSplit | `conditionaltokens_position_split` | Emitted when a position is successfully split |
| PositionsMerge | `conditionaltokens_positions_merge` | Emitted when positions are successfully merged |
| PayoutRedemption | `conditionaltokens_payout_redemption` | Emitted when payout is redeemed |

### ERC1155

Events from the ERC1155 token standard for token transfers and approvals.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| TransferSingle | `erc1155_transfer_single` | Emitted when a single token is transferred |
| TransferBatch | `erc1155_transfer_batch` | Emitted when multiple tokens are transferred in a batch |
| ApprovalForAll | `erc1155_approval_for_all` | Emitted when an operator is approved or revoked for all tokens |
| URI | `erc1155_uri` | Emitted when a token URI is updated |

### NegRiskAdapter

Events from the Neg Risk Adapter contract for market preparation and position management.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| MarketPrepared | `negriskadapter_market_prepared` | Emitted when a new market is prepared |
| NewAdmin | `negriskadapter_new_admin` | Emitted when a new admin is added |
| OutcomeReported | `negriskadapter_outcome_reported` | Emitted when an outcome is reported for a question |
| PayoutRedemption | `negriskadapter_payout_redemption` | Emitted when payout is redeemed |
| PositionSplit | `negriskadapter_position_split` | Emitted when a position is split |
| PositionsConverted | `negriskadapter_positions_converted` | Emitted when positions are converted |
| PositionsMerge | `negriskadapter_positions_merge` | Emitted when positions are merged |
| QuestionPrepared | `negriskadapter_question_prepared` | Emitted when a new question is prepared for a market |
| RemovedAdmin | `negriskadapter_removed_admin` | Emitted when an admin is removed |

### FeeModule

Events from the Fee Module contract for fee management.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| FeeRefunded | `feemodule_fee_refunded` | Emitted when a fee is refunded |
| FeeWithdrawn | `feemodule_fee_withdrawn` | Emitted when a fee is withdrawn |
| NewAdmin | `feemodule_new_admin` | Emitted when a new admin is added |
| RemovedAdmin | `feemodule_removed_admin` | Emitted when an admin is removed |

### SafeProxyFactory

Events from the Safe Proxy Factory contract for proxy wallet creation.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| ProxyCreation | `safeproxyfactory_proxy_creation` | Emitted when a new Safe proxy is created |
| ProxyCreationL2 | `safeproxyfactory_proxy_creation_l2` | Emitted when a new Safe proxy is created with L2 metadata |
| ChainSpecificProxyCreationL2 | `safeproxyfactory_chain_specific_proxy_creation_l2` | Emitted when a chain-specific Safe proxy is created |

### UmaCtfAdapter

Events from the UMA CTF Adapter contract for question initialization and resolution.

| Event Name | Table Name | Description |
| ---------- | ---------- | ----------- |
| AncillaryDataUpdated | `umactfadapter_ancillary_data_updated` | Emitted when ancillary data is updated for a question |
| NewAdmin | `umactfadapter_new_admin` | Emitted when a new admin is added |
| QuestionEmergencyResolved | `umactfadapter_question_emergency_resolved` | Emitted when a question is resolved via emergency resolution |
| QuestionFlagged | `umactfadapter_question_flagged` | Emitted when a question is flagged |
| QuestionInitialized | `umactfadapter_question_initialized` | Emitted when a new question is initialized |
| QuestionPaused | `umactfadapter_question_paused` | Emitted when a question is paused |
| QuestionReset | `umactfadapter_question_reset` | Emitted when a question is reset |
| QuestionResolved | `umactfadapter_question_resolved` | Emitted when a question is resolved |
| QuestionUnpaused | `umactfadapter_question_unpaused` | Emitted when a question is unpaused |
| RemovedAdmin | `umactfadapter_removed_admin` | Emitted when an admin is removed |
| QuestionUnflagged | `umactfadapter_question_unflagged` | Emitted when a question is unflagged (V3 only) |
