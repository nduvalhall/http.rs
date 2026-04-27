use http::{Request, Route, Server};

fn index(_: &mut (), _: Request) -> Result<(), ()> {
    println!("index endpoint called");
    Ok(())
}

fn main() {
    let mut server = Server::new("localhost:8080", ());

    server.add_route(Route::get("/", index));

    server.run();
}
