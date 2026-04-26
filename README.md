# celerity/http

A lightweight, single-threaded HTTP API framework for Rust. Attach handlers to routes and carry state through a typed context.

## Philosophy

- **Single-threaded** — no `Arc`, no `Mutex`, no surprise contention. Each request runs sequentially.
- **Typed context** — your application state flows through every handler as a `&mut C`.
- **Trait-driven handlers** — `FromRequest` / `ToResponse` let the type system wire inputs and outputs without macros.
- **Offload heavy work** — handlers run on the single server thread; any blocking or CPU-intensive work must be handed off to a separate thread or a separate service. A slow handler stalls every subsequent request.

## Quick start

```rust
struct Ctx { count: u32 }

fn increment(ctx: &mut Ctx, _: ()) -> () {
    ctx.count += 1;
}

fn get_count(ctx: &mut Ctx, _: ()) -> Response {
    Response::ok(format!("{{\"count\":{}}}", ctx.count))
}

fn main() {
    let mut server = Server::new("127.0.0.1:3000", Ctx { count: 0 });
    server.add_route(Route::new("POST", "/increment", increment));
    server.add_route(Route::new("GET",  "/count",     get_count));
    server.run();
}
```

## Examples

- `examples/counter/` — stateful counter with an HTML UI
- `examples/poll/` — multi-option poll with live vote totals
- `examples/simple/` — minimal hello-world setup
- `examples/ums/` — user management system demonstrating `FromRequest` on a custom type

Run any example:

```
cargo run --example counter
cargo run --example poll
cargo run --example simple
cargo run --example ums
```

---

## Requirements

Tasks marked **[done]** are already implemented. Everything else is the roadmap.

### Core server

- [x] Bind TCP listener and accept connections
- [x] Parse HTTP request line (method, path, version)
- [x] Parse request body
- [x] Dispatch to matched route handler
- [x] Write HTTP response bytes
- [x] Generic context type `C` shared across all handlers
- [ ] Graceful shutdown on SIGINT / SIGTERM
- [ ] Recover from handler panics without killing the server (catch_unwind)
- [ ] Replace all `unwrap` / `expect` with proper error propagation; log and return 500 instead of crashing

### Routing

- [x] Exact method + path matching
- [x] Trait-based handler wrappers (`FromRequest` / `ToResponse`)
- [ ] Dynamic path segments — `/users/:id` extractable in `FromRequest`
- [ ] Query string access — `?foo=bar` extractable in `FromRequest`
- [ ] Automatic 404 response when no route matches (currently silently drops)
- [ ] Automatic 405 response when path matches but method does not

### Request

- [x] Method and path as `&str`
- [x] Raw body as `String`
- [x] `FromRequest` impl for `Request` (pass-through), `()` (no body needed)
- [ ] Expose request headers as a parsed map (at minimum `Content-Type`, `Content-Length`, `Authorization`)
- [ ] Honor `Content-Length` header to read the correct number of body bytes (current 4 KB buffer silently truncates larger bodies)
- [ ] `FromRequest` impl for `String` (raw body string without wrapping in Request)

### Response

- [x] `StatusCode`: 200 Ok, 204 No Content, 404 Not Found, 500 Internal Server Error
- [x] `Response::ok(body)` builder
- [x] `ToResponse` for `()` → 204, `Result<T,E>` → 500 on error
- [ ] Add status codes: 201 Created, 400 Bad Request, 401 Unauthorized, 403 Forbidden, 409 Conflict, 422 Unprocessable Entity
- [ ] `Response::with_status(code, body)` builder for arbitrary status codes
- [ ] Set arbitrary response headers (at minimum `Content-Type`)
- [ ] `Content-Type: application/json` response variant

### Serialization — Pipe protocol

- [x] Parse `key=value|key=value` body format into a `HashMap`
- [x] `Pipe::get(key)` retrieves values
- [ ] Escape mechanism so `|` and `=` can appear inside values without breaking the parse
- [ ] Return a typed `ParseError` from `Pipe::parse` instead of silently producing a partial map
- [ ] `Pipe::build` — construct a pipe-encoded string from a map (for writing responses)

### Middleware

- [ ] Define a `Middleware` trait (or function signature) that receives a `Request` and returns a `Request` — allowing mutation, enrichment, or early rejection before the handler runs
- [ ] Middleware chain: multiple middleware run in order; any one can short-circuit with a `Response` instead of passing the request forward
- [ ] Built-in middleware: request logging (method, path, response status, duration)
- [ ] Built-in middleware: `Authorization` header extraction / rejection (returns 401 if missing or invalid)
- [ ] Attach middleware globally (all routes) or per-route

### Testing

- [ ] Unit tests for request parsing (malformed lines, missing body, oversized input)
- [ ] Unit tests for `Pipe` (round-trip, escaping, missing keys)
- [ ] Unit tests for response formatting (status line, headers, body)
- [ ] Integration test that spins up a `Server` on a random port and makes real HTTP requests against it
- [ ] Test that a panicking handler does not crash the server (requires panic recovery above)

### Documentation

- [ ] Rustdoc on all public types and traits (`Server`, `Route`, `Request`, `Response`, `Pipe`, `FromRequest`, `ToResponse`)
- [ ] Crate-level doc comment in `lib.rs` with a minimal getting-started example
- [ ] Document the single-threaded contract and the offload warning explicitly
- [ ] `cargo doc --no-deps` produces no warnings

### HTTPS / TLS

- [ ] Optional TLS via `rustls` — enabled through a feature flag so plain HTTP builds have no extra dependencies
- [ ] `Server::with_tls(addr, ctx, cert_path, key_path)` constructor that loads a PEM certificate and private key from disk
- [ ] Serve HTTPS on a separate port alongside HTTP (e.g. 80 + 443), or exclusively over TLS
- [ ] Automatic HTTP → HTTPS redirect when both ports are active
- [ ] ALPN negotiation advertising `http/1.1`
- [ ] Example: `examples/tls/` demonstrating a self-signed cert setup for local development

### Polish

- [ ] Replace all `println!` in handlers with a minimal structured logger (at minimum timestamps + level)
- [ ] Expose server address after bind so the caller knows which port was assigned when `0` is passed
- [ ] `cargo clippy -- -D warnings` passes clean
- [ ] `cargo fmt --check` passes clean
- [ ] Add a `benches/` harness using `criterion` for request throughput
