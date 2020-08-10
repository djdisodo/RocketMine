use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use std::ops::{Deref, DerefMut};
use std::any::Any;

pub struct Thread<T: ThreadImplementation> {
	inner: Arc<Mutex<T>>,
	handle: JoinHandle<()>
}

impl<T: ThreadImplementation> Deref for Thread<T> {
	type Target = Mutex<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T: ThreadImplementation> Thread<T> {
	pub fn join(self) -> Arc<Mutex<T>> {
		self.handle.join();
		self.inner
	}
}

pub trait ThreadImplementation: Sized + Send + Sync + 'static {
	fn run(_self: Arc<Mutex<Self>>);
	fn interrupt(_self: Arc<Mutex<Self>>);

	fn start(self) -> Thread<Self> {
		let inner = Arc::new(Mutex::new(self));
		let clone = inner.clone();
		let handle = thread::spawn(move || {
			Self::run(clone);
		});
		Thread {
			inner,
			handle
		}
	}
}