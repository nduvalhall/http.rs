pub enum ContentType {
    Json,
    Xml,
    Html,
    PlainText,
    FormUrlEncoded,
    MultipartFormData,
    OctetStream,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        String::from(match self {
            ContentType::Json => "application/json",
            ContentType::Xml => "application/xml",
            ContentType::Html => "text/html",
            ContentType::PlainText => "text/plain",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::OctetStream => "application/octet-stream",
        })
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "application/json" => Some(ContentType::Json),
            "application/xml" => Some(ContentType::Xml),
            "text/html" => Some(ContentType::Html),
            "text/plain" => Some(ContentType::PlainText),
            "application/x-www-form-urlencoded" => Some(ContentType::FormUrlEncoded),
            "multipart/form-data" => Some(ContentType::MultipartFormData),
            "application/octet-stream" => Some(ContentType::OctetStream),
            _ => None,
        }
    }
}
