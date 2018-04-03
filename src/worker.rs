//! A worker thread that will update the state of a connection as long
//! as that connection is open.

use std::{thread, time};
use std::sync::{Arc, Mutex, Weak};
use ::connection::Connection;
use serial_unix;

pub struct Worker {
    inner: Weak<Mutex<Connection<serial_unix::TTYPort>>>
}

impl Worker {
    pub fn spawn(ref conn: &Arc<Mutex<Connection<serial_unix::TTYPort>>>) -> thread::JoinHandle<()> {
        let inner = Arc::downgrade(&conn);
        thread::spawn(move || Worker { inner }.run())
    }

    pub fn run(&self) {
        while let Some(lock) = self.inner.upgrade() {
            {
                let mut conn = lock.lock().unwrap();
                conn.update();
            }
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}
