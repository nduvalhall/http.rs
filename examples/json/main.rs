use http::{HttpError, IntoJson, Json, JsonValue, Request, Response, Route, Server};

struct Car {
    brand: String,
    model: String,
    doors: u8,
    year: u16,
}

impl IntoJson for Car {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonObject(vec![
            ("brand".into(), JsonValue::JsonString(self.brand)),
            ("model".into(), JsonValue::JsonString(self.model)),
            ("doors".into(), JsonValue::JsonUint(self.doors.into())),
            ("year".into(), JsonValue::JsonUint(self.year.into())),
        ])
    }
}

struct Person {
    name: String,
    age: u64,
    gender: String,
    occupation: String,
    car: Car,
}

impl IntoJson for Person {
    fn into_json(self) -> JsonValue {
        JsonValue::JsonObject(vec![
            ("name".into(), JsonValue::JsonString(self.name)),
            ("age".into(), JsonValue::JsonUint(self.age)),
            ("gender".into(), JsonValue::JsonString(self.gender)),
            ("occupation".into(), JsonValue::JsonString(self.occupation)),
            ("car".into(), self.car.into_json()),
        ])
    }
}

fn get_person(_: &mut (), _: Request) -> Result<Response, HttpError> {
    Ok(Response::ok(Json(Person {
        name: "James Smith".into(),
        age: 30,
        gender: "Male".into(),
        occupation: "Blacksmith".into(),
        car: Car {
            brand: "Ford".into(),
            model: "Fiesta".into(),
            doors: 5,
            year: 2007,
        },
    })))
}

fn main() {
    Server::new("localhost:8080", ())
        .route(Route::new("GET", "/", get_person))
        .run();
}
