# amoeba

A lightweight, single-threaded HTTP API framework for Rust. Attach handlers to routes and carry state through a typed context.

## Philosophy

- **Single-threaded** — no `Arc`, no `Mutex`, no surprise contention. Requests are handled sequentially.
- **Typed context** — your application state flows through every handler as `&mut C`.
- **No external dependencies** — built entirely on `std`.
- **Offload heavy work** — a slow handler stalls every subsequent request. Hand CPU-intensive or blocking work off to a separate thread or service.

## Quick start

```rust
use amoeba::prelude::*;

struct Counter(i32);

fn increment(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    counter.0 += 1;
    Ok(Response::new())
}

fn decrement(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    counter.0 -= 1;
    Ok(Response::new())
}

fn get_count(counter: &mut Counter, _: Request) -> Result<Response, HttpError> {
    let count = counter.0.to_string().into_bytes();
    Ok(Response::new()
        .body(ContentType::PlainText, count)
        .status(200))
}

fn main() {
    Server::new("localhost:8080", Counter(0))
        .route(Route::new("GET", "/count", get_count))
        .route(Route::new("POST", "/increment", increment))
        .route(Route::new("POST", "/decrement", decrement))
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
    pub body: Option<Body>,
}
```

Header keys are normalized to lowercase. The body is populated only when a `Content-Length` header is present; if absent, `body` is `None`. Access body bytes via `body.data` and its content type via `body.content_type`.

## Response

`Response::new()` defaults to status `204 No Content`. Use builder methods to set a status and optional body:

```rust
Response::new()
    .body(ContentType::PlainText, b"hello".to_vec())
    .status(200)

Response::new()
    .status(202)
    .header("X-Request-Id", "abc123")
    .body(ContentType::PlainText, b"accepted".to_vec())
```

Responses with a body automatically include `Content-Type` and `Content-Length` headers.

## Errors

`HttpError::new(message)` is the standard error type. It defaults to status `500`; use `.status(code)` to override:

```rust
HttpError::new("something went wrong")       // 500
HttpError::new("not found").status(404)      // 404
HttpError::new("invalid input").status(400)  // 400
```

The server automatically converts an `HttpError` returned from a handler into a plain-text HTTP response with the given status.

## Middleware

Middleware intercepts a request before it reaches the handler. It receives `&mut C` and `Request` and either returns a (possibly modified) `Request` to continue the chain, or an `Err(HttpError)` to short-circuit.

```rust
fn validate(_: &mut (), req: Request) -> Result<Request, HttpError> {
    match &req.body {
        Some(b) => match b.content_type {
            ContentType::PlainText => Ok(req),
            _ => Err(HttpError::new("Only PlainText content supported")),
        },
        None => Err(HttpError::new("No body received")),
    }
}
```

Register with a specific method and path, or `"*"` for either dimension to match all:

```rust
// Applies only to POST /echo
.middleware(Middleware::new("POST", "/echo", validate))

// Applies to every request
.middleware(Middleware::new("*", "*", auth))
```

Middleware runs in registration order.

## Examples

- `examples/counter/` — stateful counter (increment, decrement, read)
- `examples/echo/` — middleware-validated echo endpoint
- `examples/users/` — request body parsing and mutable collection state

```
cargo run --example counter
cargo run --example echo
cargo run --example users
```
