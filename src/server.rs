use std::io::Read;
// crate keyword means the root of the entire crate
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    // Self is alias for the struct name
    pub fn new(address: String) -> Self {
        Self { address }
    }

    // struct methods take a self object
    // pass a reference so
    // ownership of the entire struct is not
    // taken

    pub fn run(self) {
        println!("listening on {}", self.address);

        // the error here should be unrecoverable and the program
        // should be terminated
        // So the Result is unwrapped
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            // creates a slice contains entire array
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            let res: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
