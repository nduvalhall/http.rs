use amoeba::prelude::*;

struct User(String);

impl User {
    fn from_request(req: &Request) -> Result<Self, &'static str> {
        let data = match &req.body {
            Some(b) => Ok(&b.data),
            None => Err("User not provided"),
        }?;

        let Ok(user) = std::str::from_utf8(data) else {
            return Err("Name not UTF-8 compatible");
        };

        Ok(Self(user.into()))
    }
}

struct Users(Vec<User>);

impl Users {
    fn to_str(&self) -> String {
        self.0
            .iter()
            .map(|u| &u.0)
            .fold(String::new(), |mut acc, u| {
                acc.push_str(&format!("{}\n", u));
                acc
            })
    }

    fn find(&self, user: &User) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, u)| u.0 == user.0)
            .map(|(i, _)| i)
    }
}

fn get_users(users: &mut Users, _: Request) -> Result<Response, HttpError> {
    Ok(Response::new()
        .body(ContentType::PlainText, users.to_str().into_bytes())
        .status(200))
}

fn add_user(users: &mut Users, req: Request) -> Result<Response, HttpError> {
    let user = User::from_request(&req).map_err(|e| HttpError::new(e))?;
    users.0.push(user);
    Ok(Response::new())
}

fn remove_user(users: &mut Users, req: Request) -> Result<Response, HttpError> {
    let user = User::from_request(&req).map_err(|e| HttpError::new(e))?;

    let index = users
        .find(&user)
        .ok_or(HttpError::new("User not found").status(404))?;

    users.0.remove(index);

    Ok(Response::new())
}

fn main() {
    Server::new("localhost:8080", Users(Vec::new()))
        .route(Route::new("GET", "/users", get_users))
        .route(Route::new("POST", "/users", add_user))
        .route(Route::new("DELETE", "/users", remove_user))
        .run();
}
