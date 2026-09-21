# Load testing

Ad-hoc scripts for finding this server's throughput ceiling, not
automated CI tests — that's why they live outside `src/` and aren't wired
into `cargo test`.

## Setup

```
cargo install oha
chmod +x tests/load/*.sh
```

## Usage

1. Start the server in one terminal: `cargo run`
2. In another terminal, from the project root:
   ```
   ./tests/load/run.sh
   ```
3. Optionally tune load:
   ```
   DURATION=60s CONCURRENCY=200 ./tests/load/run.sh
   ```

## What you're looking for

- **Requests/sec plateauing** as concurrency increases — expected,
  healthy saturation behavior.
- **A large gap between `/health` and `/db-health` throughput** — points
  at the Postgres connection pool (`max_connections` in `src/db.rs`) as
  the bottleneck, not axum itself.
- **Non-2xx responses appearing** in `oha`'s status code breakdown, or
  **panics in the server's own terminal/log file** — these are bugs to
  fix, not just "the app is loaded." A healthy server degrades (slows
  down, queues, eventually returns 5xx) rather than crashing outright.
