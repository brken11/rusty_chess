use super::GraphicsBackend;

use std::thread;

pub struct X11Backend {
    
}

impl GraphicsBackend for X11Backend {
    fn new() -> Self {
        X11Backend {
            
        }
    }
    fn start(self) -> thread::JoinHandle<Self> {
        todo!()
    }
    fn stop(self) {
        todo!()
    }
}