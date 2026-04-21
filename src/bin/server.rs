use std::sync::{Arc, Mutex};

use http::{Server, request::Request, response::Response};

struct Context {
    message: String,
}

fn hello_world(context: Arc<Mutex<Context>>, _request: Request) -> Response {
    let mut context = context.lock().unwrap();
    println!("Hello world!!!! {}", context.message);
    context.message = String::from("hello world 1");

    Response {
        status_code: http::response::StatusCode::NoContent,
    }
}

fn hello_world2(context: Arc<Mutex<Context>>, _request: Request) -> Response {
    let mut context = context.lock().unwrap();
    println!("Hello world!!!! {}", context.message);
    context.message = String::from("hello world222222");
    println!("Hello world22222!!!!");

    Response {
        status_code: http::response::StatusCode::NoContent,
    }
}

fn main() {
    let addr = "127.0.0.1:42069";
    let context = Context {
        message: String::from("Test"),
    };

    Server::bind(addr, context)
        .add_route("GET", "/hello-world", hello_world)
        .add_route("GET", "/hello-world2", hello_world2)
        .run();
}
