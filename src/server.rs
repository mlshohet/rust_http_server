use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::{ Read, Write } ;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}


// declaration
pub struct Server {
    addr: String,
}

// implementation is done in a different block
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // methods accept self (this) as first parameter
    // refenrence has to be used, because variables are deallocated
    // after function completes
    pub fn run(self, mut handler: impl Handler) {
        println!("Listen on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                // retur values have to be bound
                // to conditions
                Ok((mut stream, _)) => {
                    // syntax to create an array
                    // length is 1024 of all 0s
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                          
                        },
                        Err(e) => println!("Failed to read: {}", e),
                    }
                },
                Err(e) => println!("Error! Failed to connect: {}", e),
            }

            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }

    }
}
