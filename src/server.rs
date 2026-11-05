use std::net::{TcpListener};

pub struct Server{
    addr: String
}

impl Server{
    pub fn new(addr:String) -> Server {
        Server {addr:addr}
    }

    pub fn run(self) {
        let _listner = TcpListener::bind(&self.addr).unwrap();
        println!("Server Running on: http://{}",&self.addr);
    } 
}