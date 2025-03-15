use crate::http::http_method::MethodError;
use crate::http::{HTTPMethod, QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct HTTPRequest<'buf> {
    method: HTTPMethod,
    path: &'buf str,
    query_params: Option<QueryString<'buf>>,
}

impl<'buf> TryFrom<&'buf [u8]> for HTTPRequest<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, req) = get_next_word(request, 0).ok_or(ParseError::InvalidMethod)?;
        let (mut path, req) = get_next_word(request, req + 1).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request, req + 1).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HTTPMethod = method.parse()?;
        let mut query_params = None;

        if let Some(i) = path.find('?') {
            query_params = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            method,
            path,
            query_params,
        })
    }
}

fn get_next_word(request: &str, start: usize) -> Option<(&str, usize)> {
    for (i, c) in request[start..].chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[start..start + i], start + i));
        }
    }
    None
}

pub enum ParseError {
    InvalidMethod,
    InvalidRequest,
    InvalidProtocol,
    InvalidStatus,
    InvalidEncoding,
    InvalidHeader,
    InvalidBody,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid method",
            Self::InvalidRequest => "Invalid request",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidStatus => "Invalid status",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidHeader => "Invalid header",
            Self::InvalidBody => "Invalid body",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
