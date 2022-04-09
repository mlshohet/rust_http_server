use std::io::Read;
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
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
