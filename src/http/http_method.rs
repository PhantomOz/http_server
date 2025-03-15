use std::str::FromStr;

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}

impl FromStr for HTTPMethod {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "DELETE" => Ok(HTTPMethod::DELETE),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "TRACE" => Ok(HTTPMethod::TRACE),
            "CONNECT" => Ok(HTTPMethod::CONNECT),
            _ => Err(MethodError::InvalidMethod),
        }
    }
}

pub enum MethodError {
    InvalidMethod,
}
