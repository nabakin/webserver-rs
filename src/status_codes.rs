#[derive(Clone)]
pub struct StatusCode {
    pub header: &'static str,
    pub message: &'static str
}

pub fn r400() -> StatusCode {
    StatusCode {
        header: "400 BAD REQUEST",
        message: "400 Bad request"
    }
}

pub fn r404() -> StatusCode {
    StatusCode {
        header: "404 NOT FOUND",
        message: "404 Page not found"
    }
}