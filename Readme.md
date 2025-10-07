### CEX — Minimal Centralized Exchange (Rust + Actix)

A toy centralized exchange (CEX) backend written in Rust. It exposes HTTP endpoints for creating, querying, and canceling orders, and fetching aggregated orderbook depth. Internally it uses an Actix actor-based matching engine and an in-memory `BTreeMap` orderbook with price-time priority.

---

### Status
- Work-in-progress: not compiling yet.
- Known alignment tasks:
  - Align engine messages to routes (`CreateOrder`, `GetDepth`) and remove `EnhancedMatchingEngine`.
  - Fix typos/casing and `impl` blocks in `token.rs`, `market.rs`, `balance.rs`.
  - Consider `edition = "2021"` in `Cargo.toml`.

---

### Requirements
- Rust (stable) and Cargo
- Windows/macOS/Linux

---

### Run (after fixes)
```bash
cargo run
```
Server will listen on:
http://127.0.0.1:8080

- POST `/order`
  - Body:
    ```json
    { "user_id": "u1", "side": "Buy|Sell", "price": "100.5", "quantity": "2" }
    ```
  - Response:
    ```json
    { "status": "Order received", "order_id": "uuid" }
    ```

- GET `/order/{order_id}`
  - Response:
    ```json
    { "order_id": "uuid", "user_id": "u1", "side": "Buy", "price": "100.5", "quantity": "2", "filled_quantity": "0", "timestamp": 0 }
    ```

- DELETE `/order/{order_id}/{user_id}`
  - Response:
    ```json
    { "status": "Cancel request accepted", "order_id": "uuid" }
    ```

- GET `/depth`
  - Response:
    ```json
    { "bids": [["100.5","3"]], "asks": [["101.0","1"]] }
    ```

Example:
```bash
curl -X POST http://127.0.0.1:8080/order \
  -H "Content-Type: application/json" \
  -d '{"user_id":"u1","side":"Buy","price":"100.5","quantity":"2"}'
```

---

### Project Structure
- `src/main.rs`: Starts Actix-Web server, wires routes and engine actor.
- `src/routes.rs`: HTTP handlers for create/get/cancel order and depth.
- `src/engine.rs`: Matching engine actor, in-memory orders, matching helpers.
- `src/orderbook.rs`: `BTreeMap`-backed orderbook (bids/asks) with `VecDeque` at each price.
- `src/input.rs`: Core domain types (`Order`, `Fill`, `Side`).
- `src/output.rs`: Request/response DTOs for the HTTP API.
- `src/token.rs`: Token and `TradingPair` models; simple registry (needs fixes).
- `src/market.rs`: `Market` and `MarketManager` for per-pair orderbooks/liquidity (needs fixes).
- `src/balance.rs`: User balances and seeding a market maker (needs fixes).

---

### Notes
- Prices/quantities use `rust_decimal` to avoid float precision issues; API accepts them as strings.
- All data is in-memory; persistence and auth are out of scope for this toy build.

---

### Roadmap
- Fix compile issues and align engine↔routes.
- Add per-market orderbooks and balance checks.
- Persist trades and add pub/sub.
- Tests and benchmarks.