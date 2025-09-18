use crate::controller::{DeviceController, AsyncResult};
use log::info;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;
use hex;

pub struct MockController {
    fragments: Arc<Mutex<VecDeque<String>>>,
    callback: Arc<Mutex<Option<Box<dyn Fn(String) + Send + Sync>>>>,
}

impl DeviceController for MockController {
    fn connect(&mut self) -> AsyncResult<()> {
        let fragments = self.fragments.clone();
        let callback = self.callback.clone();

        Box::pin(async move {
            info!("MockController: connect");
            // Simulate successful connection with notification
            let msg = "E0E1E2E3AAAAAAAAE4E5E6E7";
            fragments.lock().unwrap().push_back(msg.to_string());
            if let Some(cb) = callback.lock().unwrap().as_ref() {
                cb(msg.to_string());
            }
            Ok(())
        })
    }

    fn is_connected(&self) -> bool {
        info!("MockController: is_connected");
        true
    }

    fn send(&mut self, data: &str) -> AsyncResult<()> {
        let fragments = self.fragments.clone();
        let callback = self.callback.clone();
        let data = data.to_string();

        Box::pin(async move {
            info!("MockController: send({:?})", data);
            fragments.lock().unwrap().push_back(data.clone());
            if let Some(cb) = callback.lock().unwrap().as_ref() {
                cb(data);
            }
            Ok(())
        })
    }

    fn has_complete_message(&self) -> bool {
        info!("MockController: has_complete_message");
        !self.fragments.lock().unwrap().is_empty()
    }

    fn take_complete_message(&mut self) -> Option<String> {
        info!("MockController: take_complete_message");
        self.fragments.lock().unwrap().pop_front()
    }

    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        info!("MockController: set_receiver_callback");
        *self.callback.lock().unwrap() = Some(callback);
    }

    fn clear_receiver_callback(&mut self) {
        info!("MockController: clear_receiver_callback");
        *self.callback.lock().unwrap() = None;
    }
}

impl MockController {
    pub fn new() -> Self {
        MockController {
            fragments: Arc::new(Mutex::new(VecDeque::new())),
            callback: Arc::new(Mutex::new(None)),
        }
    }
}