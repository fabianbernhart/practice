pub enum StatusCode {
    OK = 200,
    Notfound = 404,
}

impl StatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::OK => "HTTP/1.1 200 OK",
            StatusCode::Notfound => "HTTP/1.1 404 NOT FOUND",
        }
    }
}
