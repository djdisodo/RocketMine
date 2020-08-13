use std::sync::{Arc, Mutex};
use crate::Server;
use crate::network::network::Network;
use crate::network::mcpe::raknet::RaknetServer;
use crate::network::mcpe::network_session::NetworkSession;
use std::io::Read;
use raknet_rs::server::ipc::{RaknetToUserThreadEventReceiver, UserToRaknetMessageSender};

pub struct RaknetInterface<'a> {
	server: &'a Server,
	network: Arc<Mutex<Network>>,
	raknet_server_id: u64,
	raknet: Arc<Mutex<RaknetServer>>,
	sessions: Vec<NetworkSession>,
	event_receiver: RaknetToUserThreadEventReceiver,
	interface: UserToRaknetMessageSender
}