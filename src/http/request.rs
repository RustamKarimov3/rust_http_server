use super::method::{Method,MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display,Result as FmtResult, Formatter};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
	path: &'buf str,
	query_string: Option<&'buf str>,
	method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
	type Error = ParseError;

	fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
		 let req = str::from_utf8(buf)?;

		 let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
		 let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
		 let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

		 if protocol != "HTTP/1.1" {
			 return Err(ParseError::InvalidProtocol);
		 }

		 let method: Method = method.parse::<Method>()?;
		 let mut query_string = None;

		 if let Some(i) = path.find('?') {
			query_string = Some(&path[i+1..]);
			path = &path[..i];
		 }

		 Ok(
			 Self {
				 path,
				 query_string,
				 method,
			 }
		 )
	}
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i, c) in request.chars().enumerate() {
		if c == ' ' || c == '\r' {
			return Some((&request[..i], &request[i+1..]))
		}
	}

	return None
}

pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidEncoding => "Invalid encoding",
			Self::InvalidRequest => "Invalid request",
			Self::InvalidMethod => "Invalid method",
			Self::InvalidProtocol => "Invalid protocol",
		}
	}
}

impl From<Utf8Error> for ParseError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl From<MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod
	}
}

impl Debug for ParseError {
	fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
		write!(formatter, "{}", self.message())
	}
}

impl Display for ParseError {
	fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
		write!(formatter, "{}", self.message())
	}
}

impl Error for ParseError {

}