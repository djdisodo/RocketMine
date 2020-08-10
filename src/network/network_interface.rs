
pub trait NetworkInterface {
	/**
	 * Performs actions needed to start the interface after it is registered.
	 */
	fn start(&mut self);

	fn set_name(&mut self, name: String);

	/**
	 * Called every tick to process events on the interface.
	 */
	fn tick(&mut self);

	/**
	 * Gracefully shuts down the network interface.
	 */
	fn shutdown(&mut self);
}