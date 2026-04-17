use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

#[derive(PartialEq)]
pub enum Method {
    GET,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            _ => Self::GET,
        }
    }
}

pub struct Request {
    method: Method,
    route: String,
}

pub struct Response {
    pub message: String,
}

impl Response {}

pub struct Route {
    method: Method,
    route: String,
    f: fn(Request) -> Response,
}

pub struct Server {
    local_addr: String,
    listener: TcpListener,
    routes: Vec<Route>,
}

impl Server {
    pub fn bind(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to {addr}"));
        Server {
            local_addr: addr.into(),
            listener,
            routes: Vec::new(),
        }
    }

    pub fn add_route(self, method: Method, path: &str, f: fn(Request) -> Response) -> Self {
        let mut routes = self.routes;
        routes.push(Route {
            method,
            route: path.to_string(),
            f,
        });
        Self {
            local_addr: self.local_addr,
            listener: self.listener,
            routes: routes,
        }
    }

    pub fn get(self, path: &str, f: fn(Request) -> Response) -> Self {
        self.add_route(Method::GET, path, f)
    }

    fn parse_request(request: &str) -> Request {
        let mut lines = request.lines();
        let mut line1 = lines.next().unwrap().split(" ");
        let method = line1.next().unwrap().into();
        let route = line1.next().unwrap().into();

        Request { method, route }
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
                    let request = Self::parse_request(request_str);

                    let route = self
                        .routes
                        .iter()
                        .find(|&route| {
                            route.route == request.route && route.method == request.method
                        })
                        .unwrap();

                    let response = (route.f)(request);

                    stream.write_all(response.message.as_bytes()).unwrap()
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
