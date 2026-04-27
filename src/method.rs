#[derive(PartialEq, Eq, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Option,
}

impl Method {
    pub fn from_str(method: &str) -> Method {
        match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "OPTION" => Method::Option,
            _ => panic!("Unknow method: {}", method),
        }
    }
}
