# Code Audit

## Bugs

### `HttpError` Display missing closing parenthesis
`src/http_error.rs:13` — the format string is `"HttpError(status_code: {}, detail: {}"`, missing the closing `)`. Cosmetic but incorrect.

### `JsonValue::JsonChar` produces invalid JSON
`src/json.rs:21` — `to_string()` for `JsonChar(c)` outputs the bare character (e.g. `a`). Valid JSON requires string values to be quoted (`"a"`). Any `JsonChar` value produces malformed JSON.

### `JsonValue::JsonFloat` allows invalid JSON values
`src/json.rs:22` — Rust's `f64::to_string()` can produce `"NaN"` or `"inf"`, neither of which are valid JSON. Serializing a `NaN` or infinite float silently produces a document that parsers will reject.

### `JsonValue::from_bytes` is unimplemented
`src/json.rs:76` — the body is `todo!()`, which means any code path that attempts to parse incoming JSON will panic at runtime. `Json<T: FromJson>::from_bytes` delegates here (`src/json.rs:91`), so request body deserialization is entirely broken and will crash the server process.

### `Html::from_bytes` is unimplemented
`src/html.rs:12` — same `todo!()`. Less impactful than the JSON case since request bodies are rarely HTML, but it is a silent landmine.

### 4 KB read buffer silently truncates large requests
`src/server.rs:86` — the TCP stream is read into a fixed 4096-byte stack buffer in a single call. Any request whose headers plus body exceed 4096 bytes is silently truncated. `Request::from_bytes` may then parse a body that is shorter than `Content-Length` indicates, or miss headers entirely. There is no error, warning, or retry.

### Response body round-trips through UTF-8 conversion unnecessarily
`src/response.rs:101` — the body is stored as `Vec<u8>` but then converted back to a `String` via `String::from_utf8` before writing. This means any truly binary body (e.g. an image, a compressed payload) cannot be sent — the conversion fails and the server returns a 500. For the current use cases (HTML and JSON) this is harmless, but the abstraction promises `Vec<u8>` and then silently restricts it to UTF-8.

---

## Design Issues

### Handlers are function pointers, not closures
`src/route.rs:3`, `src/middleware.rs:3` — `type Handler<C> = fn(&mut C, Request) -> Result<Response, HttpError>`. This is an `fn` pointer, not an `Fn` trait. Handlers cannot capture any environment. Any data needed by a handler must live in `C`. For simple apps this is fine, but it is a significant constraint that will surprise users.

### `FromRequest` trait is dead code
`src/request.rs:52` — `FromRequest` is declared and re-exported but the dispatch loop in `server.rs` never calls it. Handlers always receive a raw `Request`. The trait has no effect on routing or dispatch; anything implementing it gains nothing.

### Middleware path matching is too coarse
`src/server.rs:43` — middleware fires on `"*"` (all routes) or an exact path match only. There is no prefix matching (`/api/*`), no wildcard segments, and no method filtering. A middleware meant for `/api/admin` would need to be registered once per admin endpoint.

### No path parameter extraction
Routes are exact string matches. There is no support for `/users/:id` style segments. User-facing paths with dynamic IDs require a separate route per ID or manual parsing inside the handler.

### `JsonValue` uses `Vec` instead of `HashMap` for objects
`src/json.rs:9`, `src/json.rs:44` — `JsonObject` is `Vec<(String, JsonValue)>`. This allows duplicate keys (which are technically legal in JSON but widely considered a defect) and makes key lookup O(n). For small objects the performance difference is negligible, but the duplicate-key footgun is real.

### `IntoJson` implemented on `&mut Poll`
`examples/poll/main.rs:19` — implementing a serialization trait on a mutable reference is unusual and limits how the trait can be used. The impl should be on the owned type or a shared reference.

### `ContentType` default implementation is misleading
`src/response.rs:15` — the default returns `"text/plain; charset=utf-8"`. It is implemented for `String` and `&str` with that default. A `String` containing JSON or HTML served through a bare `response.body("…")` will be sent with the wrong content type, with no compile-time warning.

### Public fields on `Route` and `Middleware` expose internals
`src/route.rs:6-7`, `src/middleware.rs:6` — `method`, `path`, and `path` are `pub`. The server accesses them directly rather than through a method. This makes the internal layout part of the public API and prevents future refactoring without a breaking change.

### `Server::run` consumes `self` with no way to stop
`src/server.rs:130` — `run` takes ownership and loops forever. There is no shutdown handle, no signal handling, and no way for a caller to stop the server without killing the process.

### `send_error` panics if it cannot write
`src/server.rs:73-78` — if the write to the stream fails, `unwrap_or_else(|_| panic!(…))` crashes the server thread. Since this is the path for 500-level errors, a secondary failure here would be fatal. Errors should be logged and the connection dropped instead.
