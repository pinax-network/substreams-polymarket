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
- [x] Open Interest
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

| Event | Description |
| ----- | ----------- |
| OrderFilled | Emitted when an order is filled (swap events) |
| FeeCharged | Emitted when a fee is charged on a trade |
| NewAdmin | Emitted when a new admin is added |
| NewOperator | Emitted when a new operator is added |
| OrderCancelled | Emitted when an order is cancelled |
| OrdersMatched | Emitted when orders are matched |
| ProxyFactoryUpdated | Emitted when the proxy factory address is updated |
| RemovedAdmin | Emitted when an admin is removed |
| RemovedOperator | Emitted when an operator is removed |
| SafeFactoryUpdated | Emitted when the safe factory address is updated |
| TokenRegistered | Emitted when a new token pair is registered |
| TradingPaused | Emitted when trading is paused |
| TradingUnpaused | Emitted when trading is unpaused |

### ConditionalTokens

Events from the Conditional Tokens (CTF) contract for condition preparation and resolution.

| Event | Description |
| ----- | ----------- |
| ConditionPreparation | Emitted upon the successful preparation of a condition |
| ConditionResolution | Emitted when a condition is resolved |
| PositionSplit | Emitted when a position is successfully split |
| PositionsMerge | Emitted when positions are successfully merged |
| PayoutRedemption | Emitted when payout is redeemed |

### ERC1155

Events from the ERC1155 token standard for token transfers and approvals.

| Event | Description |
| ----- | ----------- |
| TransferSingle | Emitted when a single token is transferred |
| TransferBatch | Emitted when multiple tokens are transferred in a batch |
| ApprovalForAll | Emitted when an operator is approved or revoked for all tokens |
| URI | Emitted when a token URI is updated |

### NegRiskAdapter

Events from the Neg Risk Adapter contract for market preparation and position management.

| Event | Description |
| ----- | ----------- |
| MarketPrepared | Emitted when a new market is prepared |
| NewAdmin | Emitted when a new admin is added |
| OutcomeReported | Emitted when an outcome is reported for a question |
| PayoutRedemption | Emitted when payout is redeemed |
| PositionSplit | Emitted when a position is split |
| PositionsConverted | Emitted when positions are converted |
| PositionsMerge | Emitted when positions are merged |
| QuestionPrepared | Emitted when a new question is prepared for a market |
| RemovedAdmin | Emitted when an admin is removed |

### FeeModule

Events from the Fee Module contract for fee management.

| Event | Description |
| ----- | ----------- |
| FeeRefunded | Emitted when a fee is refunded |
| FeeWithdrawn | Emitted when a fee is withdrawn |
| NewAdmin | Emitted when a new admin is added |
| RemovedAdmin | Emitted when an admin is removed |

### SafeProxyFactory

Events from the Safe Proxy Factory contract for proxy wallet creation.

| Event | Description |
| ----- | ----------- |
| ProxyCreation | Emitted when a new Safe proxy is created |
| ProxyCreationL2 | Emitted when a new Safe proxy is created with L2 metadata |
| ChainSpecificProxyCreationL2 | Emitted when a chain-specific Safe proxy is created |

### UmaCtfAdapter

Events from the UMA CTF Adapter contract for question initialization and resolution.

| Event | Description |
| ----- | ----------- |
| AncillaryDataUpdated | Emitted when ancillary data is updated for a question |
| NewAdmin | Emitted when a new admin is added |
| QuestionEmergencyResolved | Emitted when a question is resolved via emergency resolution |
| QuestionFlagged | Emitted when a question is flagged |
| QuestionInitialized | Emitted when a new question is initialized |
| QuestionPaused | Emitted when a question is paused |
| QuestionReset | Emitted when a question is reset |
| QuestionResolved | Emitted when a question is resolved |
| QuestionUnpaused | Emitted when a question is unpaused |
| RemovedAdmin | Emitted when an admin is removed |
| QuestionUnflagged | Emitted when a question is unflagged (V3 only) |

## Clickhouse Aggregated State

The following tables describe the aggregated state available in Clickhouse.

### Open Interest

Aggregated Open Interest calculated from `conditionaltokens_position_split` and `conditionaltokens_positions_merge` events.

| Table | Description |
| ----- | ----------- |
| state_open_interest | Open Interest aggregated by time interval (1m, 5m, 10m, 30m, 1h, 4h, 1d, 1w) |

**Key Fields:**
- `parent_collection_id`: Parent collection ID. Global OI uses `0x0000000000000000000000000000000000000000000000000000000000000000`
- `condition_id`: Condition ID for market-specific OI
- `net_open_interest`: Net open interest change (splits - merges)
- `split_amount`: Total split amount (increases OI)
- `merge_amount`: Total merge amount (decreases OI)

**Query Examples:**

```sql
-- Get Market Open Interest for a specific condition at 1-hour intervals
SELECT
    timestamp,
    sum(net_open_interest) AS open_interest
FROM state_open_interest
WHERE interval_min = 60
  AND condition_id = '0x...'
GROUP BY timestamp
ORDER BY timestamp;

-- Get Global Open Interest at daily intervals
SELECT
    timestamp,
    sum(net_open_interest) AS global_open_interest
FROM state_open_interest
WHERE interval_min = 1440
  AND parent_collection_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
GROUP BY timestamp
ORDER BY timestamp;
```
