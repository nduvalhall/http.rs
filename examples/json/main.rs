use http::prelude::*;

struct Car {
    brand: String,
    model: String,
    doors: u8,
    year: u16,
}

impl IntoJson for Car {
    fn to_json(self) -> Json {
        Json::JsonObject(vec![
            ("brand".into(), Json::JsonString(self.brand)),
            ("model".into(), Json::JsonString(self.model)),
            ("doors".into(), Json::JsonUint(self.doors.into())),
            ("year".into(), Json::JsonUint(self.year.into())),
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
    fn to_json(self) -> http::Json {
        Json::JsonObject(vec![
            ("name".into(), Json::JsonString(self.name)),
            ("age".into(), Json::JsonUint(self.age)),
            ("gender".into(), Json::JsonString(self.gender)),
            ("occupation".into(), Json::JsonString(self.occupation)),
            ("car".into(), self.car.to_json()),
        ])
    }
}

fn get_person(_: &mut (), _: Request) -> Response {
    Response::ok(Person {
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
    })
}

fn main() {
    let mut server = Server::new("localhost:8080", ());

    server.add_route(Route::get("/", get_person));

    server.run();
}
