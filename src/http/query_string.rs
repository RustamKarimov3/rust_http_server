use std::collections::HashMap;
// use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;

#[derive(Debug)]
pub struct QueryString<'buf> {
	pub data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryString<'buf> {
	fn get(&self, key: &str) -> Option<&Value> {
		self.data.get(key)
	}
}

#[derive(Debug)]
pub enum Value<'buf> {
	Single(&'buf str),
	Multiple(Vec<&'buf str>)
}

// impl<'buf> Value<'buf> {
// 	fn message(&self) -> &str {
// 		match self {
// 			Self::Single(v) => v,
// 			Self::Multiple(v) => {
// 				v.con
// 			},
// 		}
// 	}
// }

// impl<'buf> Display for Value<'buf> {
// 	fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
// 		write!(formatter, "{}", self.m)
// 	}
// }

impl<'buf> From<&'buf str> for QueryString<'buf> {
	fn from(s: &'buf str) -> Self {
		let mut data = HashMap::new();

		for sub_str in s.split("&") {
			let mut key = sub_str;
			let mut value = "";

			if let Some(i) = sub_str.find("=") {
				key = &sub_str[..i];
				value = &sub_str[i+1..];
			}

			data.entry(key)
			.and_modify(|existing: &mut Value | match existing {
				Value::Single(prev) => {
					*existing = Value::Multiple(vec![prev, value]);
				}
				Value::Multiple(vec) => vec.push(value)
			})
		.or_insert(Value::Single(value));

		}

		QueryString { data }
	}
}

// Value::Single(prev_value) => {
// 	let mut vec = Vec::new();

// 	*existing = Value::Multiple(vec![prev_value, value]);
// }
// Value::Multiple(vec) => vec.push(value),