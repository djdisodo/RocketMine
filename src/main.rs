use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::collections::VecDeque;
use raknet_rs::server::ipc::{UserToRaknetMessage, UserToRaknetMessageReceiver};
use raknet_rs::server::{ServerEventListener, ProtocolAcceptor, Server, ServerInterface};
use parking_lot::Mutex;
use std::time::Duration;
use uuid::Uuid;
use simple_logger::SimpleLogger;

//pub mod network;
//pub mod thread;

fn main() {
    server();
}

fn server() {
    SimpleLogger::new().init().unwrap();
    let chan: Arc<Mutex<VecDeque<UserToRaknetMessage>>> = Default::default();
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 19132)).unwrap();
    socket.set_ttl(255);
    log::debug!("start");
    let mut server = Server::new(
        13253860892328930865,
        socket,
        1500,
        PA {},
        UserToRaknetMessageReceiver::new(chan),
        EL {}
    );

    server.internal.lock().set_name("MCPE;Hello, World!;422;1.16.200;0;10;13253860892328930865;가슴이 웅장해진다;Survival;1;19132;19133;".to_owned());
    server.run();

}

struct EL;

impl ServerEventListener for EL {
    fn on_client_connect(&mut self, session_id: usize, address: SocketAddr, client_id: u64) {

    }

    fn on_client_disconnect(&mut self, session_id: usize, reason: &str) {

    }

    fn on_packet_receive(&mut self, session_id: usize, packet: &[u8]) {

    }

    fn on_raw_packet_receive(&mut self, address: SocketAddr, payload: &[u8]) {

    }

    fn on_packet_ack(&mut self, session_id: usize, identifier_ack: u64) {

    }

    fn on_bandwidth_stats_update(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize) {

    }

    fn on_ping_measure(&mut self, session_id: usize, latency: Duration) {

    }
}

struct PA;

impl ProtocolAcceptor for PA {
    fn accepts(&self, version: u8) -> bool {
        true
    }

    fn get_primary_version(&self) -> u8 {
        1
    }
}