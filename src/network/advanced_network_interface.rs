use crate::network::NetworkInterface;
use std::net::IpAddr;
use std::time::Duration;
use crate::network::network::Network;
use regex::bytes::Regex;

pub trait AdvancedNetworkInterface: NetworkInterface {
	/**
	 * Prevents packets received from the IP address getting processed for the given timeout.
	 *
	 */
	fn block_address(&mut self, address: IpAddr) {

	}
	fn block_address_for(&mut self, address: IpAddr, timeout: Duration);

	/**
	 * Unblocks a previously-blocked address.
	 */
	fn unblock_address(&mut self, address: &IpAddr);

	fn set_network(&mut self, network: Network);

	/**
	 * Sends a raw packet to the network interface, bypassing any sessions.
	 */
	fn send_raw_packet(&mut self, address: &IpAddr, payload: &[u8]);

	/**
	 * Adds a regex filter for raw packets to this network interface. This filter should be used to check validity of
	 * raw packets before relaying them to the main thread.
	 */
	fn add_raw_packet_filter(&mut self, regex: Regex);
}