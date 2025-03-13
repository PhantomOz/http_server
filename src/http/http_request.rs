use crate::http::http_method::HTTPMethod;

pub struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    query_params: Option<String>,
}