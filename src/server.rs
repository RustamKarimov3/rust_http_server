use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
	addr: String,
}

impl Server {
	pub fn new(addr: String) -> Self {
			Self {
					addr
			}
	}

	pub fn start(self) {
			println!("Listening on {}", self.addr);
			let listener = TcpListener::bind(&self.addr).unwrap();

			loop {
				match listener.accept() {
					Ok((mut stream, _)) => {
						let mut buffer = [0; 1024];

						match stream.read(&mut buffer) {
							Ok(_) => {
								println!("Received a request: {}", String::from_utf8_lossy(&buffer));

								 match Request::try_from(&buffer[..]) {
									Ok(_) => {
									}
									Err(e) => {
										println!("Failed: {}", e);
									}
								}
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