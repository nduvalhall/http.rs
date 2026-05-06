use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{
    http_error::HttpError,
    middleware::Middleware,
    request::Request,
    response::{ContentType, Response},
    route::Route,
};

/// The HTTP server. Generic over application context type `C`.
///
/// Requests are handled one at a time on a single thread. A slow handler blocks
/// all subsequent requests; hand CPU-intensive or I/O-bound work off to a separate thread.
pub struct Server<C> {
    addr: String,
    ctx: C,
    middlewares: Vec<Middleware<C>>,
    routes: Vec<Route<C>>,
}

impl<C> Server<C> {
    /// Creates a new server that will listen on `addr` with the given context.
    pub fn new(addr: &str, ctx: C) -> Self {
        Self {
            addr: addr.into(),
            ctx,
            middlewares: Vec::new(),
            routes: Vec::new(),
        }
    }

    /// Registers a route. Routes are matched in registration order.
    pub fn route(self, route: Route<C>) -> Self {
        let mut routes = self.routes;
        routes.push(route);
        Self { routes, ..self }
    }

    /// Registers middleware. Middleware runs in registration order before the route handler.
    pub fn middleware(self, middleware: Middleware<C>) -> Self {
        let mut middlewares = self.middlewares;
        middlewares.push(middleware);
        Self {
            middlewares,
            ..self
        }
    }

    fn dispatch(&mut self, mut req: Request) -> Result<Response, HttpError> {
        let middleware_routes: Vec<&Middleware<C>> = self
            .middlewares
            .iter()
            .filter(|r| r.path == req.path || r.path == "*")
            .collect();

        let middleware_routes: Vec<&Middleware<C>> = middleware_routes
            .into_iter()
            .filter(|r| r.method == req.method || r.method == "*")
            .collect();

        let mut middleware_iter = middleware_routes.into_iter();

        while let Some(m) = middleware_iter.next() {
            req = ((m.handler)(&mut self.ctx, req))?
        }

        let path_routes: Vec<&Route<C>> =
            self.routes.iter().filter(|r| r.path == req.path).collect();

        if path_routes.is_empty() {
            return Err(HttpError::new(format!("No endpoint found for {}", req.path)).status(404));
        }

        match path_routes.iter().find(|r| r.method == req.method) {
            Some(&route) => (route.handler)(&mut self.ctx, req),
            None => Err(HttpError::new(format!(
                "No endpoint found for {} with method {}",
                req.path, req.method
            ))
            .status(405)),
        }
    }

    fn send_error(&self, stream: &mut impl Write) {
        if let Ok(bytes) = Response::new().status(500).into_bytes() {
            stream.write_all(&bytes).ok();
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        if let Ok(addr) = stream.peer_addr() {
            println!("Connection from {}", addr);
        }

        let request = match Request::from_reader(&mut stream) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to read request: {}", e);
                self.send_error(&mut stream);
                return;
            }
        };

        let response = match self.dispatch(request) {
            Ok(r) => r,
            Err(e) => Response::new()
                .status(e.status)
                .body(ContentType::PlainText, e.message.into_bytes()),
        };

        let bytes = match response.into_bytes() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to serialize response: {}", e);
                self.send_error(&mut stream);
                return;
            }
        };

        if let Err(e) = stream.write_all(&bytes) {
            eprintln!("Failed to write response: {}", e);
        }

        if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
            eprintln!("Failed to shut down stream: {}", e);
        }
    }

    /// Starts listening and blocks forever, handling one request at a time.
    ///
    /// # Panics
    /// Panics if the address cannot be bound.
    pub fn run(mut self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.addr)?;

        println!("Listening on {}", self.addr);

        for conn in listener.incoming() {
            match conn {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }

        Ok(())
    }
}
