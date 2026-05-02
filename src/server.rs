use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{HttpError, IntoResponse, Middleware, Request, Response, Route};

pub struct Server<C> {
    addr: String,
    ctx: C,
    middlewares: Vec<Middleware<C>>,
    routes: Vec<Route<C>>,
}

impl<C> Server<C> {
    pub fn new(addr: &str, ctx: C) -> Self {
        Self {
            addr: addr.into(),
            ctx,
            middlewares: Vec::new(),
            routes: Vec::new(),
        }
    }

    pub fn route(self, route: Route<C>) -> Self {
        let mut routes = self.routes;
        routes.push(route);
        Self { routes, ..self }
    }

    pub fn middleware(self, middleware: Middleware<C>) -> Self {
        let mut middlewares = self.middlewares;
        middlewares.push(middleware);
        Self {
            middlewares,
            ..self
        }
    }

    fn dispatch(&mut self, mut req: Request) -> Result<Response, HttpError> {
        for middleware in self.middlewares.iter() {
            let path = &middleware.path;
            if path == "*" || path == &req.path {
                req = middleware.call(&mut self.ctx, req)?
            }
        }

        let path_routes: Vec<&Route<C>> =
            self.routes.iter().filter(|r| r.path == req.path).collect();

        if path_routes.is_empty() {
            return Err(HttpError::new(
                404,
                &format!("No endpoint found for {}", &req.path),
            ));
        }

        match path_routes.iter().find(|&r| *r.method == req.method) {
            Some(&route) => Ok(route.call(&mut self.ctx, req)?),
            None => Err(HttpError::new(
                405,
                &format!(
                    "No endpoint found for {} with method {}",
                    req.path, req.method
                ),
            )),
        }
    }

    fn send_error(&self, status_code: u16, stream: &mut impl Write) {
        stream
            .write_all(
                &HttpError::new(status_code, "Failed to parse request")
                    .into_response()
                    .into_bytes()
                    .unwrap_or_else(|_| panic!("Failed to send error")),
            )
            .unwrap_or_else(|_| panic!("Failed to write error to stream"));
    }

    fn handle_connection(&mut self, stream: TcpStream) {
        if let Ok(addr) = stream.peer_addr() {
            println!("Connection from {}", addr);
        }

        let (mut stream, request) = Request::from_stream(stream);
        let request = match request {
            Ok(r) => r,
            Err(e) => {
                let Ok(b) = e.into_response().into_bytes() else {
                    self.send_error(500, &mut stream);
                    return;
                };
                stream.write_all(&b).ok();
                return;
            }
        };

        let response = self.dispatch(request).unwrap_or_else(|e| e.into_response());

        let Ok(bytes) = response.into_bytes() else {
            self.send_error(422, &mut stream);
            return;
        };

        stream.write_all(&bytes).unwrap_or_else(|_| {
            eprintln!("Failed to write response");
            return;
        });

        stream
            .shutdown(std::net::Shutdown::Both)
            .unwrap_or_else(|_| {
                eprintln!("Failed to shut down stream");
                return;
            });
    }

    pub fn run(mut self) {
        let listener = TcpListener::bind(&self.addr)
            .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", &self.addr, e));

        println!("Listening on {}", self.addr);

        for conn in listener.incoming() {
            match conn {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }
    }
}
