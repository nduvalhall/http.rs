pub mod request;
pub mod response;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::{request::Request, response::Response};

pub struct Route<C> {
    method: String,
    route: String,
    f: fn(C, Request) -> Response,
}

pub struct Server<C> {
    context: Arc<Mutex<C>>,
    listener: TcpListener,
    routes: Vec<Route<Arc<Mutex<C>>>>,
}

impl<C> Server<C> {
    pub fn bind(addr: &str, context: C) -> Self {
        let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to {addr}"));
        Server {
            context: Arc::new(Mutex::new(context)),
            listener,
            routes: Vec::new(),
        }
    }

    pub fn add_route(
        self,
        method: &str,
        path: &str,
        f: fn(Arc<Mutex<C>>, Request) -> Response,
    ) -> Self {
        let mut routes = self.routes;
        routes.push(Route {
            method: method.to_string(),
            route: path.to_string(),
            f,
        });
        Self {
            context: self.context,
            listener: self.listener,
            routes: routes,
        }
    }

    fn handle_connection(&self, address: SocketAddr, mut stream: TcpStream) {
        loop {
            let mut buffer = [0u8; 1024];

            match stream.read(&mut buffer) {
                Err(_) => println!("Error trying to read stream"),
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }

                    let request_str = str::from_utf8(&buffer[0..bytes_read]).unwrap();
                    let request = Request::from_string(request_str.to_string());

                    let route = self
                        .routes
                        .iter()
                        .find(|&route| {
                            route.route == request.path && route.method == request.method
                        })
                        .unwrap();

                    let response = (route.f)(Arc::clone(&self.context), request);

                    stream.write_all(response.to_string().as_bytes()).unwrap()
                }
            };
        }

        println!("Connection with {address} closed");
        println!("---");
    }

    pub fn run(self) {
        let connections = self.listener.incoming();

        for connection in connections {
            let stream = connection.unwrap();
            let peer_addr = stream.peer_addr().unwrap();
            println!("---");
            println!("Connection from {}", peer_addr);

            self.handle_connection(peer_addr, stream);
        }
    }
}
