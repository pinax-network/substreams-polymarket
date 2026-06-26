# Head-of-chain delivery latency testing

How to measure, and what we observe for, the **head-of-chain delivery latency**
of the Polymarket Substreams on the Pinax Polygon endpoint — i.e. how quickly a
live consumer pinned at the chain tip receives each new block.

This is the metric that matters for latency-sensitive, real-time consumers
(live trading, alerting). It is **not** a measure of historical back-processing
throughput.

## TL;DR

At head, on `polygon.substreams.pinax.network:443`, blocks are delivered
**effectively in real time — within roughly ±1s of their own block timestamp,
worst case ~2s, never exceeding Polygon's ~2s block cadence.** Development mode
and production mode are statistically identical at head; production mode's
parallelism only helps historical ranges, which a head-pinned consumer never
back-processes.

There is **no multi-second block-delivery floor** at the Substreams layer. A
consumer observing a ~13s+ delivery median is measuring something other than
head-of-chain block delivery (e.g. event-time/trade-submission → receipt, which
includes Polygon's own mempool + inclusion time before we index the block; or
back-pressure / finality handling on the consumer side).

## Tool

[`scripts/head-latency.sh`](../scripts/head-latency.sh) streams clock-only
messages starting at head (`-s -1 -t 0`), stamps the local wall-clock arrival
time on each block, and reports the distribution of two quantities:

| metric | meaning |
|---|---|
| **age** | delivery time relative to the block's own timestamp — how far behind real time each block is delivered |
| **gap** | inter-arrival time between consecutive delivered blocks — should track chain cadence; spikes are delivery stalls |

```bash
# duration_seconds  module      endpoint                               mode(dev|prod)
./scripts/head-latency.sh 75 map_events polygon.substreams.pinax.network:443 dev
./scripts/head-latency.sh 75 map_events polygon.substreams.pinax.network:443 prod
```

Defaults: `75s`, `map_events`, Polygon endpoint, `dev`.

### Caveats / how to read it

- **`age` is only as good as your local clock.** Verify NTP sync before trusting
  it: `sntp -t 2 time.apple.com` (offset should be well under 1s). Our reference
  runs were taken with the local clock synced to +14ms.
- **Negative `age` is normal**, not an error: delivery occasionally beats the
  block's assigned timestamp by a fraction. Block timestamps are assigned at
  proposal time and have sub-second variance against wall-clock receipt.
- The reference numbers below are **short samples at head** and reflect a single
  point in time / network path. They characterize the steady-state floor, not
  the long-tail (p99) behavior, which is sensitive to chain stability — Polygon
  is one of the less stable chains to host, so a heavier tail is plausible under
  load and should be measured over longer windows.
- This measures the **clock/block delivery cadence**, which in development mode
  is emitted for every block regardless of module output. To measure latency on
  *event-bearing* blocks specifically, stream the module's actual output instead
  of `-o clock`.

## Reference results

`map_events` at head, `polygon.substreams.pinax.network:443`, local clock synced
to +14ms vs NTP. All values in seconds.

### Development vs production mode (70s samples, 2026-06-26)

| metric | development | production |
|---|---|---|
| blocks delivered | 47 | 48 |
| age — median | −0.279 | −0.150 |
| age — p90 | 0.504 | 0.569 |
| age — max | 1.250 | 2.143 |
| gap — median | 1.535 | 1.390 |
| gap — p90 | 2.329 | 2.473 |
| gap — max | 2.623 | 2.716 |

**Conclusion:** the two modes are indistinguishable at head — both deliver within
block-time variance. Production mode confers **no head-of-chain latency
advantage**; its parallel back-processing only benefits historical ranges, which
a head-pinned live consumer does not traverse. This validates running
**development mode** for live consumption.

### Additional development-mode samples (2026-06-26)

| sample | blocks | age median | age p90 | age max | gap median | gap p90 | gap max |
|---|---|---|---|---|---|---|---|
| 75s | 51 | −0.311 | 0.332 | 1.771 | 1.432 | 2.456 | 2.765 |
| 30s | 22 | −0.344 | 0.335 | 1.298 | 1.401 | 2.071 | 2.149 |

Consistent across runs: sub-2s delivery, gaps tracking the ~2s block cadence,
no stalls.

## Operational notes for live consumers

- **Heartbeat / receive watchdog.** Polygon produces a block every ~2s, and in
  development mode the stream emits a message **per block** even when a server-side
  filter matches nothing — so the stream is never silent at head. A 120s receive
  timeout is very conservative; it would only fire on a genuine delivery stall,
  which is exactly when you want it to fire. There is no separate application-level
  keepalive because the per-block cadence already serves as one. (This holds at
  head; during historical catch-up or a stream restart the cadence differs.)
- **Alerting.** On our side, a block-time delay exceeding ~3 minutes is a critical
  alert. Given the sub-2s steady-state floor measured here, a consumer-side
  watchdog well under that (e.g. 120s) is safe and appropriate.
