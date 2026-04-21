pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn from_string(value: String) -> Self {
        let mut lines = value.lines();

        let line = lines.next().unwrap();
        let mut words = line.split(" ");

        let method = words.next().unwrap();
        let path = words.next().unwrap();

        Request {
            method: method.to_string(),
            path: path.to_string(),
        }
    }
}
