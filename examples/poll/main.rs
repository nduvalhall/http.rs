use http::prelude::*;

struct Poll {
    yes: u32,
    no: u32,
    maybe: u32,
}

fn index(_: &mut Poll, _: Request) -> Response {
    Response::ok(include_str!("poll.html").to_string())
}

fn get_votes(poll: &mut Poll, _: Request) -> Response {
    let results = format!(
        r#"{{"yes":{},"no":{},"maybe":{}}}"#,
        poll.yes, poll.no, poll.maybe
    );
    Response::ok(results)
}

fn vote_yes(poll: &mut Poll, _: Request) {
    poll.yes += 1;
    println!("yes: {}", poll.yes);
}

fn vote_no(poll: &mut Poll, _: Request) {
    poll.no += 1;
    println!("no: {}", poll.no);
}

fn vote_maybe(poll: &mut Poll, _: Request) {
    poll.maybe += 1;
    println!("maybe: {}", poll.maybe);
}

fn main() {
    let poll = Poll {
        yes: 0,
        no: 0,
        maybe: 0,
    };

    let mut server = Server::new("0.0.0.0:8086", poll);

    server.add_route(Route::get("/", index));
    server.add_route(Route::get("/votes", get_votes));
    server.add_route(Route::post("/vote/yes", vote_yes));
    server.add_route(Route::post("/vote/no", vote_no));
    server.add_route(Route::post("/vote/maybe", vote_maybe));

    server.run();
}
