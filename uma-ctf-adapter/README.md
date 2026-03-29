# Polymarket: UMA CTF Adapter

This crate extracts UMA oracle lifecycle events used by Polymarket markets on Polygon.

## Module

- `map_events`: emits `uma_ctf_adapter.v1.Events`

## Events

- `AncillaryDataUpdated`
- `NewAdmin`
- `QuestionEmergencyResolved`
- `QuestionFlagged`
- `QuestionInitialized`
- `QuestionPaused`
- `QuestionReset`
- `QuestionResolved`
- `QuestionUnpaused`
- `RemovedAdmin`
- `QuestionUnflagged`

## Contracts

- UmaCtfAdapter v3: `0x2f5e3684cb1f318ec51b00edba38d79ac2c0aa9d`
- UmaCtfAdapter v2: `0x6a9d222616c90fca5754cd1333cfd9b7fb6a4f74`

## Build

```bash
make build
```
