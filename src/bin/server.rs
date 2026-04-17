use http::{Method, Request, Response, Server};

fn hello_world(_request: Request) -> Response {
    println!("Hello world!!!!");
    Response {}
}

fn hello_world2(_request: Request) -> Response {
    println!("Hello world22222!!!!");
    Response {}
}

fn main() {
    let addr = "127.0.0.1:42069";
    let mut server = Server::new(addr);

    server.add_route(Method::GET, "/hello-world", hello_world);
    server.add_route(Method::GET, "/hello-world2", hello_world2);

    server.run();
}
