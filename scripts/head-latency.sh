#!/usr/bin/env bash
#
# head-latency.sh — measure head-of-chain delivery latency of a Substreams
# endpoint in DEVELOPMENT mode (the linear, live-streaming path a latency-
# sensitive consumer uses).
#
# It streams clock-only messages starting at head (-s -1, -t 0 = run forever),
# stamps the local wall-clock arrival time on each block, and reports the
# distribution of:
#   * age   = delivery time relative to the block's own timestamp
#             (≈ how far behind real time each block is delivered)
#   * gap   = inter-arrival time between consecutive delivered blocks
#             (should track the chain's block cadence; spikes = delivery stalls)
#
# Development mode is used on purpose: it is the linear streaming path, with no
# parallel back-processing, matching how a live consumer runs at head. We do NOT
# pass --production-mode and do NOT pass --final-blocks-only.
#
# Usage:
#   ./scripts/head-latency.sh [duration_seconds] [module] [endpoint] [mode]
#
#   mode = dev (default) | prod
#     dev  : linear streaming path (no --production-mode) — how a live
#            latency-sensitive consumer runs at head.
#     prod : --production-mode. At head there is no history to back-process,
#            so this measures whether production mode's batching/optimizations
#            change head-of-chain delivery vs the linear path.
#
# Examples:
#   ./scripts/head-latency.sh                       # 75s, map_events, polygon, dev
#   ./scripts/head-latency.sh 300                   # 5 min sample
#   ./scripts/head-latency.sh 120 map_events polygon.substreams.pinax.network:443 prod
#
# Requires: substreams CLI, perl. A valid SUBSTREAMS_API_TOKEN/KEY in the env.
# NOTE: the `age` metric is only as trustworthy as the local clock. Verify sync
# first, e.g.:  sntp -t 2 time.apple.com   (offset should be well under 1s).

set -euo pipefail

DURATION="${1:-75}"
MODULE="${2:-map_events}"
ENDPOINT="${3:-polygon.substreams.pinax.network:443}"
MODE="${4:-dev}"
MANIFEST="${MANIFEST:-substreams.yaml}"

PROD_FLAG=()
if [ "$MODE" = "prod" ]; then PROD_FLAG=(--production-mode); fi

# gtimeout (coreutils) on macOS, timeout on Linux.
if command -v gtimeout >/dev/null 2>&1; then TIMEOUT=gtimeout; else TIMEOUT=timeout; fi

echo "endpoint=$ENDPOINT module=$MODULE manifest=$MANIFEST duration=${DURATION}s mode=$MODE" >&2
echo "streaming at head (-s -1 -t 0, clock-only)..." >&2

"$TIMEOUT" "$DURATION" \
  substreams run -e "$ENDPOINT" "$MANIFEST" "$MODULE" ${PROD_FLAG[@]+"${PROD_FLAG[@]}"} -s -1 -t 0 -o clock 2>/dev/null \
  | perl -ne 'use Time::HiRes qw(time); printf("%.3f | %s", time(), $_)' \
  | tee /dev/stderr \
  | MODE="$MODE" perl -ne '
      if (/^([\d.]+) .*age=(-?[\d.]+)(ms|s)\b/) {
        my ($recv,$v,$u)=($1,$2,$3);
        $v/=1000 if $u eq "ms";
        push @recv,$recv; push @age,$v;
      }
      END {
        my $n=@age;
        if ($n < 2) { print "\n[head-latency] not enough blocks captured ($n)\n"; exit 1; }
        my @s = sort {$a<=>$b} @age;
        my $sum=0; $sum+=$_ for @age;
        my @gap; push @gap, $recv[$_]-$recv[$_-1] for (1..$#recv);
        my @g = sort {$a<=>$b} @gap; my $gn=@g; my $gs=0; $gs+=$_ for @gap;
        my $mode = $ENV{MODE} || "dev";
        printf "\n========== head-latency summary (%s mode) ==========\n", $mode;
        printf "blocks delivered      : %d\n", $n;
        printf "age rel. block time(s): min=%.3f  median=%.3f  p90=%.3f  max=%.3f  mean=%.3f\n",
          $s[0],$s[int($n*0.5)],$s[int($n*0.9)],$s[-1],$sum/$n;
        printf "inter-arrival gap (s) : min=%.3f  median=%.3f  p90=%.3f  max=%.3f  mean=%.3f\n",
          $g[0],$g[int($gn*0.5)],$g[int($gn*0.9)],$g[-1],$gs/$gn;
        printf "=============================================================\n";
        printf "Interpretation: |age| ~ chain block-time variance and max gap ~ block\n";
        printf "cadence means delivery is effectively real-time at head. A multi-second\n";
        printf "age or gap is a delivery stall worth investigating.\n";
      }'
