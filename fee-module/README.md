# Polymarket: Fee Module

This crate extracts fee accounting and admin events from Polymarket fee module contracts on Polygon.

## Module

- `map_events`: emits `fee_module.v1.Events`

## Events

- `FeeRefunded`
- `FeeWithdrawn`
- `NewAdmin`
- `RemovedAdmin`

## Contracts

- FeeModule v2: `0xe3f18acc55091e2c48d883fc8c8413319d4ab7b0`
- FeeModule legacy: `0x56c79347e95530c01a2fc76e732f9566da16e113`
- NegRiskFeeModule v2: `0xb768891e3130f6df18214ac804d4db76c2c37730`
- NegRiskFeeModule v1: `0x78769d50be1763ed1ca0d5e878d93f05aabff29e`
