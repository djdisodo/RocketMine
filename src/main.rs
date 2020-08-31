use log4rs::Logger;
use std::net::SocketAddr;

pub mod network;
pub mod thread;

fn main() {
    println!("Hello, world!");
}


pub struct Server {
    //TODO
}

impl Server {

    pub fn get_address(&self) -> SocketAddr {
        unimplemented!()
    }
}