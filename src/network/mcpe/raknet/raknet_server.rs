use std::net::SocketAddr;
use log4rs::Logger;
use std::sync::Arc;

pub struct RaknetServer {
	address: SocketAddr,
	logger: Logger,

	clean_shutdown: bool,
	ready: bool,

	main_to_thread_buffer: (),
	thread_to_main_buffer: (),

	main_path: (),

	server_id: u64,
	max_mtu_size: u16,
	protocol_version: u8,

}

impl RaknetServer {
	pub fn new(
		logger: Logger,
		main_to_thread_buffer: (),
		thread_to_main_buffer: (),
		address: SocketAddr,
		server_id: u64,
		max_mtu_size: u16, // default 1492
		override_protocol_version: Option<u8>,
		/* ?SleeperNotifier $sleeper = null */
	) -> Self {
		Self {
			address,
			logger,

			clean_shutdown: false,
			ready: false,

			main_to_thread_buffer,
			thread_to_main_buffer,

			main_path: (), //TODO

			server_id,
			max_mtu_size,
			protocol_version: override_protocol_version.unwrap_or(raknet_rs::DEFAULT_PROTOCOL_VERSION),
		}
	}
}