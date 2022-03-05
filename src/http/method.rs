use std::str::FromStr;




pub enum Method {
	GET,POST, DELETE,PUT, HEAD, CONNECT,OPTIONS, TRACE,PATCH,
}

impl FromStr for Method {
	type Err = MethodError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"GET" => Ok(Self::GET),
			"POST" => Ok(Self::POST),
			"PUT" => Ok(Self::PUT),
			"DELETE" => Ok(Self::DELETE),
			"HEAD" => Ok(Self::HEAD),
			"CONNECT" => Ok(Self::CONNECT),
			"TRACE" => Ok(Self::TRACE),
			"PATCH" => Ok(Self::PATCH),
			"OPTIONS" => Ok(Self::OPTIONS),
			_ => Err(MethodError)
		}
	}
}

pub struct MethodError;