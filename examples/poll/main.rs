use http::{Response, Route, Server, StatusCode};

struct Poll {
    yes: u32,
    no: u32,
    maybe: u32,
}

fn index(_: &mut Poll, _: ()) -> Result<Response, ()> {
    Ok(Response::with_body(
        StatusCode::Ok,
        Vec::from(include_str!("poll.html")),
    ))
}

fn get_votes(poll: &mut Poll, _: ()) -> Result<Response, ()> {
    let body = format!(
        r#"{{"yes":{},"no":{},"maybe":{}}}"#,
        poll.yes, poll.no, poll.maybe
    );
    Ok(Response::with_body(
        StatusCode::Ok,
        Vec::from(body.as_bytes()),
    ))
}

fn vote_yes(poll: &mut Poll, _: ()) -> Result<(), ()> {
    poll.yes += 1;
    println!("yes: {}", poll.yes);
    Ok(())
}

fn vote_no(poll: &mut Poll, _: ()) -> Result<(), ()> {
    poll.no += 1;
    println!("no: {}", poll.no);
    Ok(())
}

fn vote_maybe(poll: &mut Poll, _: ()) -> Result<(), ()> {
    poll.maybe += 1;
    println!("maybe: {}", poll.maybe);
    Ok(())
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
