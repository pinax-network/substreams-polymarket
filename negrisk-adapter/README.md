# Polymarket: Neg Risk Adapter

This crate extracts Neg Risk Adapter market and position events from Polygon.

## Module

- `map_events`: emits `negrisk_adapter.v1.Events`

## Events

- `MarketPrepared`
- `NewAdmin`
- `OutcomeReported`
- `PayoutRedemption`
- `PositionSplit`
- `PositionsConverted`
- `PositionsMerge`
- `QuestionPrepared`
- `RemovedAdmin`

## Contracts

- Neg Risk Adapter: `0xd91e80cf2e7be2e162c6513ced06f1dd0da35296`
- Neg Risk Adapter legacy: `0xf16a3bdffb7b882e3236243e901f6c5953e2ee0d`

## Build

```bash
make build
```
