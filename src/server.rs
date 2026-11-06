use std::{io::Read, net::TcpListener};
use crate::http::request::Request;
use std::convert::TryFrom;


pub struct Server{
    addr: String
}

impl Server{
    pub fn new(addr:String) -> Server {
        Server {addr:addr}
    }

    pub fn run(self) {
        let listner = TcpListener::bind(&self.addr).unwrap();
        println!("Server Running on: http://{}",&self.addr);
        loop {
            match listner.accept() {
                Ok((mut stream,_sock_addr)) => {
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse the request {}", e)
                            }
                        }
                        Err(e) =>{
                            println!("Error reading from tcp steam: {}", e);
                        }
                    }
                }
                Err(e) => print!("Failed to estalish a connection: {}", e),
                
            }
        }
    } 
}