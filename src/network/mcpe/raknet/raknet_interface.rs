use std::sync::{Arc, Mutex};
use crate::Server;
use crate::network::network::Network;
use crate::network::mcpe::raknet::RaknetServer;
use crate::network::mcpe::network_session::NetworkSession;
use std::io::Read;
use raknet_rs::server::ipc::{RaknetToUserThreadEventReceiver, UserToRaknetMessageSender};
use std::collections::VecDeque;

pub struct RaknetInterface<'a> {
	server: &'a Server,
	network: Option<Arc<Mutex<Network>>>,
	raknet_server_id: u64,
	raknet: RaknetServer,
	sessions: Vec<NetworkSession>,
	event_receiver: RaknetToUserThreadEventReceiver,
	interface: UserToRaknetMessageSender
}

impl RaknetInterface {
	const MCPE_RAKNET_PROTOCOL_VERSION: u8 = 10;
	const MCPE_RAKNET_PACKET_ID: u8 = 0xfe;
	pub fn new(server: &Server) -> Self {
		let raknet_server_id: u64 = rand::random();
		let main_to_thread = Arc::new(Mutex::new(VecDeque::new()));
		let thread_to_main = Arc::new(Mutex::new(VecDeque::new()));
		let raknet = RaknetServer::new(
			server.get_logger(),
			main_to_thread.clone(),
			thread_to_main.clone(),
			server.get_address(),
			raknet_server_id,
			1492, //$this->server->getConfigGroup()->getProperty("network.max-mtu-size", 1492)
			Some(Self::MCPE_RAKNET_PROTOCOL_VERSION),
			//$this->sleeper
		);
		let event_listener = RaknetToUserThreadEventReceiver::new(thread_to_main);
		let interface = UserToRaknetMessageSender::new(main_to_thread);
		Self {
			server,
			network: None,
			raknet_server_id,
			raknet,
			sessions: Vec::default(),
			event_receiver,
			interface
		}
	}
}