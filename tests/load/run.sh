#!/usr/bin/env bash

# Runs a small suite of load tests against the running server, from the
# cheapest endpoint (no DB, no auth) up to the most expensive one
# (authenticated, DB-backed). Comparing throughput across these tells you
# WHERE the bottleneck is, rather than just "it broke at some point."
#
# Prerequisite: the server must already be running (`cargo run` in another
# terminal) and `oha` must be installed (`cargo install oha`).

set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:3000}"

# How long each individual test runs, and how many concurrent "virtual
# users" hit the server at once. Passed as env vars so you can override
# them per-run without editing this file, e.g.:
#   CONCURRENCY=200 ./run.sh
DURATION="${DURATION:-20s}"
CONCURRENCY="${CONCURRENCY:-50}"

# Resolve the directory this script lives in, so it can find
# setup_test_user.sh regardless of what directory you invoke run.sh from.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=============================================="
echo "1. Baseline: /health (no DB, no auth)"
echo "=============================================="
# This is the ceiling of your raw axum/tokio/hyper stack with nothing else
# involved — every other number below should be compared against this one.
oha -z "$DURATION" -c "$CONCURRENCY" "$BASE_URL/health"

echo
echo "=============================================="
echo "2. DB-backed: /db-health (hits Postgres, no auth)"
echo "=============================================="
# If this is dramatically slower/lower-throughput than step 1, the
# Postgres connection pool (max_connections in db.rs) is very likely the
# bottleneck — requests are queueing for one of a small number of
# available DB connections.
oha -z "$DURATION" -c "$CONCURRENCY" "$BASE_URL/db-health"

echo
echo "=============================================="
echo "3. Full authenticated flow: /list-contents"
echo "=============================================="
# This exercises auth (JWT decode on every request via the AuthUser
# extractor) AND a real query against the files table for a specific
# user — the closest thing to "realistic production traffic" in this
# test suite.
TOKEN=$("$SCRIPT_DIR/setup_test_user.sh")
oha -z "$DURATION" -c "$CONCURRENCY" \
  -H "Authorization: Bearer $TOKEN" \
  "$BASE_URL/list-contents"

echo
echo "Done. Compare the 'Success rate', 'Requests/sec', and latency"
echo "percentiles (p95/p99) across the three sections above."
echo "Also check your server's own terminal/log file for any ERROR-level"
echo "tracing output or panics that happened during the run."
