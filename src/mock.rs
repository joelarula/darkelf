use crate::blue::BlueController;
use log::info;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;
use std::error::Error;

#[derive(Clone)]
pub struct MockController {
    callback: Arc<Mutex<Option<Box<dyn Fn(String) + Send + Sync>>>>,
}

impl MockController {
    pub fn new() -> Self {
        Self {
            callback: Arc::new(Mutex::new(None)),
        }
    }
}

impl BlueController for MockController {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        info!("MockController connected");
        Box::pin(async { Ok(()) })
    }

    fn send<'a>(&'a mut self, command: &str) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        let cmd = command.to_string();
        let cb = self.callback.clone();
        Box::pin(async move {
            info!("MockController sending command: {}", cmd);
            if let Some(callback) = cb.lock().unwrap().as_ref() {
                // Simulate a response after sending the command
                let response = format!("Response to command: {}", cmd);
                callback(response);
            }
            Ok(())
        })
    }

    fn is_connected(&self) -> bool {
        info!("is_connected called");
        true
    }

    fn disconnect<'a>(&'a mut self, ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        info!("Received disconnect");
        Box::pin(async { Ok(()) })
    }

    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        self.callback.lock().unwrap().replace(callback);
    }

    fn clear_receiver_callback(&mut self) {
        self.callback.lock().unwrap().take();
    }
}

