use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::{Middleware, Request, Response, Route};

pub struct Server<C: 'static> {
    address: &'static str,
    context: C,
    middleware: Vec<Middleware<C>>,
    routes: Vec<Route<C>>,
}

impl<C: 'static> Server<C> {
    pub fn new(address: &'static str, context: C) -> Self {
        Self {
            address,
            context,
            middleware: Vec::new(),
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route<C>) {
        self.routes.push(route);
    }

    pub fn add_middleware(&mut self, middleware: Middleware<C>) {
        self.middleware.push(middleware);
    }

    fn dispatch(&mut self, mut request: Request) -> Response {
        for middleware in self.middleware.iter() {
            let path = middleware.get_path();
            if path == "*" || path == request.path {
                request = match (middleware.get_handler())(&mut self.context, request) {
                    Ok(req) => req,
                    Err(response) => return response,
                };
            }
        }

        let path_routes: Vec<&Route<C>> = self
            .routes
            .iter()
            .filter(|r| r.get_path() == request.path)
            .collect();

        if path_routes.is_empty() {
            return Response::not_found();
        }

        match path_routes
            .iter()
            .find(|&r| *r.get_method() == request.method)
        {
            Some(route) => (route.get_handler())(&mut self.context, request),
            None => Response::method_not_allowed(),
        }
    }

    fn send_error_response(stream: &mut impl Write) {
        let _ = stream.write_all(&Response::internal_server_error("").to_bytes());
    }

    pub fn run(mut self) {
        let listener = TcpListener::bind(self.address)
            .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", self.address, e));

        println!("Listening on {}", self.address);

        for conn in listener.incoming() {
            let mut stream = match conn {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                    continue;
                }
            };

            if let Ok(addr) = stream.peer_addr() {
                println!("Connection from {}", addr);
            }

            let mut buf = [0u8; 4096];
            let n = match stream.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    Self::send_error_response(&mut stream);
                    continue;
                }
            };

            if n == 0 {
                continue;
            }

            let request = match Request::from_bytes(&buf[0..n]) {
                Some(r) => r,
                None => {
                    eprintln!("Failed to parse request");
                    Self::send_error_response(&mut stream);
                    continue;
                }
            };

            let response = self.dispatch(request);
            if let Err(e) = stream.write_all(&response.to_bytes()) {
                eprintln!("Failed to write response: {}", e);
                continue;
            }

            if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
                eprintln!("Failed to shut down stream: {}", e);
            }
        }
    }
}
