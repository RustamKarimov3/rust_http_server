use std::net::TcpListener;
use std::io::{Read};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub struct Server {
	addr: String,
}

pub trait Handler {
	fn handle_req(&self, req: &Request) -> Response;
	fn handle_bad_req(&self, req: &ParseError) -> Response {
		println!("Failed parse");
		Response::new(StatusCode::BadRequest, None)
	}
}

impl Server {
	pub fn new(addr: String) -> Self {
			Self {
					addr
			}
	}

	pub fn start(self, mut handler: impl Handler) {
			println!("Listening on {}", self.addr);
			let listener = TcpListener::bind(&self.addr).unwrap();

			loop {
				match listener.accept() {
					Ok((mut stream, _)) => {
						let mut buffer = [0; 1024];

						match stream.read(&mut buffer) {
							Ok(_) => {
								println!("Received a request: {}", String::from_utf8_lossy(&buffer));

								let response = match Request::try_from(&buffer[..]) {
									Ok(req) => handler.handle_req(&req),
									Err(e) => handler.handle_bad_req(&e),
								};

								if let Err(e) = response.send(&mut stream) {
									println!("Failed {}", e);
								};
						}

							Err(e) => {
								println!("Crashed with: {}", e)
							}
						}
					}
					_ => println!("UNCATCHED")
				}
			}
	}
}