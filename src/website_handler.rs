use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
	public_path: String
}

impl WebsiteHandler {
	pub fn new(public_path: String) -> Self {
	  Self { public_path }
	}

	fn read_file(&self, file_path: &str) -> Option<String> {
		let path = format!("{}/{}" ,self.public_path, file_path);
		println!("{path}");

		match fs::canonicalize(path) {
			Ok(path) => {
				if path.starts_with(&self.public_path) {
					fs::read_to_string(path).ok()
				} else {
					println!("Traversal attack attempted!");
					None
				}
			},
			Err(_) => None,
		}
	}
}

impl Handler for WebsiteHandler {
	fn handle_req(&self, req: &Request) -> Response {
		match req.method() {
			Method::GET => {
				match req.path() {
					"/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
					"/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
					path => match self.read_file(path) {
						Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
						None => Response::new(StatusCode::NotFound, None)
					},
				}
			}
			_ => Response::new(StatusCode::NotFound, None)
		}
	}
}