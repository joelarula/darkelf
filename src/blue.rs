use std::error::Error;

use std::future::Future;
use std::pin::Pin;


pub const LASER_DEVICE_PREFIX: &str = "TD5322A";

pub const GENERIC_ACCESS_SERVICE_UUID: &str = "00001800-0000-1000-8000-00805F9B34FB";

pub const DEVICE_INFORMATION_SERVICE_UUID: &str = "0000180A-0000-1000-8000-00805F9B34FB";

pub const LASER_SERVICE_UUID: [&str; 2] = [
    "0000FF00-0000-1000-8000-00805F9B34FB",
    "0000FFE0-0000-1000-8000-00805F9B34FB1"
];

// UUIDs from JavaScript example
pub const WRITE_UUIDS: [&str; 2] = [
    "0000FFE2-0000-1000-8000-00805F9B34FB",
    "0000FF02-0000-1000-8000-00805F9B34FB"
];
pub const NOTIFY_UUIDS: [&str; 2] = [
    "0000FFE1-0000-1000-8000-00805F9B34FB",
    "0000FF01-0000-1000-8000-00805F9B34FB"
];


pub trait BlueController: Send + Sync {
    
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
        
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    
    fn get_content(&self) -> String;
    
    fn is_connected(&self) -> bool;
}
