use std::net::SocketAddr;
use log4rs::Logger;

pub struct RaknetServer {
	address: SocketAddr,
	pub logger: Logger,

	pub clean_shutdown: bool,
	pub ready: bool,
	//TODO
}