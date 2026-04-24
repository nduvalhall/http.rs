use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::{request::Request, response::Response, route::Route, status_code::StatusCode};

pub struct Server<C> {
    address: &'static str,
    context: C,
    routes: Vec<Route<C>>,
}

impl<C> Server<C> {
    pub fn new(address: &'static str, context: C) -> Self {
        Self {
            address,
            context,
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route<C>) {
        self.routes.push(route);
    }

    fn dispatch(&mut self, request: Request) -> Response {
        if let Some(route) = self
            .routes
            .iter()
            .find(|&r| r.method == request.method && r.path == request.path)
        {
            (route.handler)(&mut self.context, request)
        } else {
            Response::new(StatusCode::NotFound)
        }
    }

    pub fn run(mut self) {
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Listening on {}", self.address);

        for conn in listener.incoming() {
            if let Ok(mut stream) = conn {
                println!("Connection from {}", stream.peer_addr().unwrap());

                let mut buf = [0u8; 4096];
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    continue;
                }

                let request = Request::from_bytes(&buf[0..n]).unwrap();

                let response = self.dispatch(request);
                let response_bytes = response.to_bytes();
                stream.write_all(&response_bytes).unwrap();
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
        }
    }
}
