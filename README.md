# http

A lightweight, single-threaded HTTP API framework for Rust. Attach handlers to routes and carry state through a typed context.

## Philosophy

- **Single-threaded** — no `Arc`, no `Mutex`, no surprise contention. Requests are handled sequentially.
- **Typed context** — your application state flows through every handler as `&mut C`.
- **No external dependencies** — built entirely on `std`.
- **Offload heavy work** — a slow handler stalls every subsequent request. Hand CPU-intensive or blocking work off to a separate thread or service.

## Quick start

```rust
use http::{HttpError, Request, Response, Route, Server};

struct Ctx {
    count: i32,
}

fn increment(ctx: &mut Ctx, _: Request) -> Result<Response, HttpError> {
    ctx.count += 1;
    Ok(Response::no_content())
}

fn get_count(ctx: &mut Ctx, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(format!("{{\"count\":{}}}", ctx.count)))
}

fn main() {
    Server::new("127.0.0.1:3000", Ctx { count: 0 })
        .route(Route::new("POST", "/increment", increment))
        .route(Route::new("GET", "/count", get_count))
        .run();
}
```

## Routing

Routes are created with `Route::new(method, path, handler)`. The method is any HTTP method string (`"GET"`, `"POST"`, `"PUT"`, `"DELETE"`, etc.). All routes use exact path matching.

Handler signature:

```rust
fn handler(ctx: &mut C, req: Request) -> Result<Response, HttpError>
```

The server returns `404` when no route matches the path and `405` when the path matches but the method does not.

## Request

```rust
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}
```

Header keys are normalized to lowercase. The body is populated from the `Content-Length` header; if absent, `body` is `None`.

## Response

Shorthand constructors:

```rust
Response::ok(body)                    // 200
Response::created(body)               // 201
Response::no_content()                // 204
Response::bad_request(body)           // 400
Response::unauthorized()              // 401
Response::forbidden(body)             // 403
Response::not_found(body)             // 404
Response::internal_server_error(body) // 500
```

Builder-style for custom status codes or headers:

```rust
Response::new()
    .status_code(202)
    .header("X-Request-Id", "abc123")
    .body("accepted")
```

The `body` argument can be a `String`, `&str`, `Html(...)`, or `Json(...)`.

## JSON

Implement `IntoJson` to serialize a type as a response body:

```rust
use http::{HttpError, IntoJson, Json, JsonValue, Request, Response, Route};

struct Point { x: f64, y: f64 }

impl IntoJson for Point {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonObject(vec![
            ("x".into(), JsonValue::JsonFloat(self.x)),
            ("y".into(), JsonValue::JsonFloat(self.y)),
        ])
    }
}

fn get_point(_: &mut (), _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Json(Point { x: 1.0, y: 2.0 })))
}
```

`JsonValue` variants: `JsonNull`, `JsonBool(bool)`, `JsonChar(char)`, `JsonUint(u64)`, `JsonInt(i64)`, `JsonFloat(f64)`, `JsonString(String)`, `JsonList(Vec<JsonValue>)`, `JsonObject(Vec<(String, JsonValue)>)`.

Parsing incoming JSON from a request body is not yet implemented.

## HTML

```rust
use http::Html;

fn index(_: &mut (), _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Html(include_str!("index.html").to_string())))
}
```

## Errors

`HttpError::new(status_code, detail)` is the standard error type. Any handler can return it; the server automatically converts it to a JSON response:

```json
{"detail": "error message here"}
```

## Middleware

Middleware intercepts a request before it reaches the handler. It receives `&mut C` and `Request` and either returns a (possibly modified) `Request` to continue the chain, or an `Err(HttpError)` to short-circuit.

```rust
use http::{HttpError, Middleware, Request};

fn auth(_: &mut Ctx, req: Request) -> Result<Request, HttpError> {
    match req.headers.get("x-api-key") {
        Some(k) if k == "secret" => Ok(req),
        _ => Err(HttpError::new(401, "Invalid API key")),
    }
}
```

Register with `"*"` to apply globally or an exact path to apply only to that route:

```rust
Server::new("localhost:8080", Ctx())
    .middleware(Middleware::new("*", auth))
    .route(Route::new("GET", "/", index))
    .run();
```

Middleware runs in registration order.

## Examples

- `examples/simple/` — minimal single-route server
- `examples/counter/` — stateful counter with an HTML UI and JSON responses
- `examples/json/` — nested struct serialization via `IntoJson`
- `examples/middleware/` — global API-key auth middleware
- `examples/poll/` — multi-option poll with live vote totals and an HTML UI

```
cargo run --example simple
cargo run --example counter
cargo run --example json
cargo run --example middleware
cargo run --example poll
```
