use amoeba::prelude::*;

fn validate(_: &mut (), req: Request) -> Result<Request, HttpError> {
    match &req.body {
        Some(b) => match b.content_type {
            ContentType::PlainText => Ok(req),
            _ => Err(HttpError::new("Only PlainText content supported")),
        },
        None => Err(HttpError::new("No body received")),
    }
}

fn echo(_: &mut (), req: Request) -> Result<Response, HttpError> {
    let body = req
        .body
        .ok_or_else(|| HttpError::new("No body").status(400))?;

    Ok(Response::new()
        .body(body.content_type, body.data)
        .status(200))
}

fn main() -> Result<(), std::io::Error> {
    Server::new("localhost:8080", ())
        .middleware(Middleware::new("POST", "/echo", validate))
        .route(Route::new("POST", "/echo", echo))
        .run()
}
