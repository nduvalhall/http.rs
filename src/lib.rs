//! A lightweight, single-threaded HTTP API framework for Rust.
//!
//! Attach typed handlers to routes and carry application state through a generic context `C`.
//! Import all public types at once with [`prelude`].

mod body;
mod http_error;
mod middleware;
pub mod prelude;
mod request;
mod response;
mod route;
mod server;
