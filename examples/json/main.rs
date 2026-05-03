use amoeba::{Error, FromJson, IntoJson, Json, JsonError, JsonResponse, Request, Route, Server};

struct Person {
    name: String,
    age: u32,
    gender: String,
    occupation: String,
}

impl IntoJson for Person {
    fn into_json(&self) -> Json {
        Json::object(vec![
            ("name", Json::string(&self.name)),
            ("age", Json::number(self.age)),
            ("gender", Json::string(&self.gender)),
            ("occupation", Json::string(&self.occupation)),
        ])
    }
}

impl FromJson for Person {
    fn from_json(json: Json) -> Result<Self, JsonError> {
        let mut obj = json.into_object()?;
        Ok(Person {
            name: obj.get("name")?,
            age: obj.get("age")?,
            gender: obj.get("gender")?,
            occupation: obj.get("occupation")?,
        })
    }
}

fn get_person(_: &mut (), _: Request) -> Result<JsonResponse<Person>, Error> {
    Ok(JsonResponse::new(Person {
        name: "James Smith".into(),
        age: 30,
        gender: "Male".into(),
        occupation: "Blacksmith".into(),
    })
    .status_code(200))
}

fn post_person(_: &mut (), req: Request) -> Result<JsonResponse<Person>, JsonError> {
    let body = Json::from_request(req)?;
    let mut person = Person::from_json(body)?;
    person.name = "New John".into();
    Ok(JsonResponse::new(person).status_code(200))
}

fn main() {
    Server::new("localhost:8080", ())
        .route(Route::new("GET", "/person", get_person))
        .route(Route::new("POST", "/person", post_person))
        .run();
}
