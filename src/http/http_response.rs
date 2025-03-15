pub struct HTTPResponse {
    status_code: HTTPStatus,
    body: String,
}

enum HTTPStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl HTTPStatus {
    pub fn new(status_code: u16) -> Self {
        match status_code {
            200 => Self::Ok,
            400 => Self::BadRequest,
            404 => Self::NotFound,
            500 => Self::InternalServerError,
            _ => panic!("Invalid status code"),
        }
    }
}
