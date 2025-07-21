use std::thread;

use super::GraphicsBackend;

pub struct WIN32Backend {

}

impl GraphicsBackend for WIN32Backend {
    fn new() -> Self {
        WIN32Backend {

        }
    }
    fn start(self) -> thread::JoinHandle<Self>{
        todo!()
    }
    fn stop(self) {
        todo!()
    }
}