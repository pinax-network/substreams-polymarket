# Polymarket: CTF Exchange

This crate extracts Polymarket exchange activity from the CTF Exchange contracts on Polygon.

## Modules

- `map_events`: emits `ctf_exchange.v1.Events`
- `store_token`: stores token pairs registered for each condition

## Events

- `FeeCharged`
- `NewAdmin`
- `NewOperator`
- `OrderCancelled`
- `OrderFilled`
- `OrdersMatched`
- `ProxyFactoryUpdated`
- `RemovedAdmin`
- `RemovedOperator`
- `SafeFactoryUpdated`
- `TokenRegistered`
- `TradingPaused`
- `TradingUnpaused`

## Contracts

- CTF Exchange: `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`
- Neg Risk CTF Exchange: `0xc5d563a36ae78145c45a50134d48a1215220f80a`

## Build

```bash
make build
```
