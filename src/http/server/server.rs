use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{
    IntoError,
    http::{
        error::Error,
        middleware::Middleware,
        request::Request,
        response::{IntoResponse, Response},
        server::route::Route,
    },
};

pub struct Server<C> {
    addr: String,
    ctx: C,
    middlewares: Vec<Middleware<C>>,
    routes: Vec<Route<C>>,
}

impl<C: 'static> Server<C> {
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

    fn dispatch(&mut self, mut req: Request) -> Result<Response, Error> {
        for middleware in self.middlewares.iter() {
            let path = &middleware.path;
            if path == "*" || path == &req.path {
                req = middleware.call(&mut self.ctx, req)?
            }
        }

        let path_routes: Vec<&Route<C>> =
            self.routes.iter().filter(|r| r.path == req.path).collect();

        if path_routes.is_empty() {
            return Err(Error::new(
                404,
                &format!("No endpoint found for {}", &req.path),
            ));
        }

        match path_routes.iter().find(|&r| *r.method == req.method) {
            Some(&route) => Ok(route.call(&mut self.ctx, req)?),
            None => Err(Error::new(
                405,
                &format!(
                    "No endpoint found for {} with method {}",
                    req.path, req.method
                ),
            )),
        }
    }

    fn send_error(&self, stream: &mut impl Write) {
        if let Ok(bytes) = Response::new().status_code(500).into_bytes() {
            stream.write_all(&bytes).ok();
        }
    }

    fn handle_connection(&mut self, stream: TcpStream) {
        if let Ok(addr) = stream.peer_addr() {
            println!("Connection from {}", addr);
        }

        let (mut stream, request) = Request::from_stream(stream);
        let request = match request {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to read request: {}", e);
                self.send_error(&mut stream);
                return;
            }
        };

        let response = match self.dispatch(request) {
            Ok(r) => r,
            Err(e) => match e.into_response() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to build error response: {}", e.get_detail());
                    self.send_error(&mut stream);
                    return;
                }
            },
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
