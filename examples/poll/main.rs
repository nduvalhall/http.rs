use http::{Html, HttpError, IntoJson, Json, JsonValue, Request, Response, Route, Server};

struct Poll {
    yes: u32,
    no: u32,
    maybe: u32,
}

impl Default for Poll {
    fn default() -> Self {
        Self {
            yes: 0,
            no: 0,
            maybe: 0,
        }
    }
}

impl IntoJson for &mut Poll {
    fn into_json(self) -> http::JsonValue {
        JsonValue::JsonObject(vec![
            ("yes".into(), JsonValue::JsonUint(self.yes.into())),
            ("no".into(), JsonValue::JsonUint(self.no.into())),
            ("maybe".into(), JsonValue::JsonUint(self.maybe.into())),
        ])
    }
}

fn index(_: &mut Poll, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Html(include_str!("poll.html").to_string())))
}

fn get_votes(poll: &mut Poll, _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Json(poll)))
}

fn vote_yes(poll: &mut Poll, _: Request) -> Result<Response, HttpError> {
    poll.yes += 1;
    println!("yes: {}", poll.yes);
    Ok(Response::no_content())
}

fn vote_no(poll: &mut Poll, _: Request) -> Result<Response, HttpError> {
    poll.no += 1;
    println!("no: {}", poll.no);
    Ok(Response::no_content())
}

fn vote_maybe(poll: &mut Poll, _: Request) -> Result<Response, HttpError> {
    poll.maybe += 1;
    println!("maybe: {}", poll.maybe);
    Ok(Response::no_content())
}

fn main() {
    Server::new("0.0.0.0:8080", Poll::default())
        .route(Route::new("GET", "/", index))
        .route(Route::new("GET", "/votes", get_votes))
        .route(Route::new("POST", "/vote/yes", vote_yes))
        .route(Route::new("POST", "/vote/no", vote_no))
        .route(Route::new("POST", "/vote/maybe", vote_maybe))
        .run();
}
