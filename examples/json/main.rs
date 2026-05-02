use http::{
    FromBytes, FromJson, HttpError, IntoJson, Json, JsonValue, Request, Response, Route, Server,
};

fn take_field(fields: &mut Vec<(String, JsonValue)>, key: &str) -> JsonValue {
    let pos = fields
        .iter()
        .position(|(k, _)| k == key)
        .unwrap_or_else(|| panic!("missing field: {key}"));
    fields.remove(pos).1
}

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

impl FromJson for Car {
    fn from_json(json: JsonValue) -> Self {
        let JsonValue::JsonObject(mut fields) = json else {
            panic!("expected object for Car");
        };
        let JsonValue::JsonString(brand) = take_field(&mut fields, "brand") else {
            panic!("expected string for Car.brand");
        };
        let JsonValue::JsonString(model) = take_field(&mut fields, "model") else {
            panic!("expected string for Car.model");
        };
        let JsonValue::JsonUint(doors) = take_field(&mut fields, "doors") else {
            panic!("expected uint for Car.doors");
        };
        let JsonValue::JsonUint(year) = take_field(&mut fields, "year") else {
            panic!("expected uint for Car.year");
        };
        Car {
            brand,
            model,
            doors: doors as u8,
            year: year as u16,
        }
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

impl FromJson for Person {
    fn from_json(json: JsonValue) -> Self {
        let JsonValue::JsonObject(mut fields) = json else {
            panic!("expected object for Person");
        };
        let JsonValue::JsonString(name) = take_field(&mut fields, "name") else {
            panic!("expected string for Person.name");
        };
        let JsonValue::JsonUint(age) = take_field(&mut fields, "age") else {
            panic!("expected uint for Person.age");
        };
        let JsonValue::JsonString(gender) = take_field(&mut fields, "gender") else {
            panic!("expected string for Person.gender");
        };
        let JsonValue::JsonString(occupation) = take_field(&mut fields, "occupation") else {
            panic!("expected string for Person.occupation");
        };
        let car = Car::from_json(take_field(&mut fields, "car"));
        Person {
            name,
            age,
            gender,
            occupation,
            car,
        }
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

fn post_person(_: &mut (), req: Request) -> Result<Response, HttpError> {
    let body = req
        .body
        .ok_or_else(|| HttpError::new(400, "missing request body"))?;
    let Json(mut person) = Json::<Person>::from_bytes(body)?;
    person.name = "New John".into();
    Ok(Response::ok(Json(person)))
}

fn main() {
    Server::new("localhost:8080", ())
        .route(Route::new("GET", "/person", get_person))
        .route(Route::new("POST", "/person", post_person))
        .run();
}
