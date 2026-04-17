use http::{Request, Response, Server};

fn hello_world(_request: Request) -> Response {
    println!("Hello world!!!!");
    Response {
        message: "HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 13

Hello, world!\r\n\r\n"
            .into(),
    }
}

fn hello_world2(_request: Request) -> Response {
    println!("Hello world22222!!!!");
    Response {
        message: "HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 13

Hello, world222!!\r\n\r\n"
            .into(),
    }
}

fn main() {
    let addr = "127.0.0.1:42069";

    Server::bind(addr)
        .get("/hello-world", hello_world)
        .get("/hello-world2", hello_world2)
        .run();
}
