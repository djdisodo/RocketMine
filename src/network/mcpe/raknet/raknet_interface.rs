use std::sync::{Arc, Mutex};
use crate::Server;
use crate::network::network::Network;
use crate::network::mcpe::raknet::RaknetServer;
use crate::network::mcpe::network_session::NetworkSession;
use raknet_rs::server::ipc::RaknetToUserThreadMessageReceiver;
use std::io::Read;

pub struct RaknetInterface<R: Read> {
	server: Arc<Mutex<Server>>,
	network: Arc<Mutex<Network>>,
	raknet_server_id: u64,
	raknet: Arc<Mutex<RaknetServer>>,
	sessions: Vec<NetworkSession>,
	event_receiver: RaknetToUserThreadMessageReceiver<R>,
}