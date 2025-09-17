use std::{pin::Pin};
use std::error::Error;

use crate::blue::BlueController;

pub struct MockBlue {
    pub connected: bool,
    pub sent_data: Vec<Vec<u8>>,
    pub content: String,
}

impl MockBlue {
    pub fn new() -> Self {
        Self {
            connected: false,
            sent_data: Vec::new(),
            content: String::new(),
        }
    }
}

impl BlueController for MockBlue {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        self.connected = true;
        Box::pin(async { Ok(()) })
    }

    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        self.sent_data.push(bytes.to_vec());
        Box::pin(async { Ok(()) })
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}