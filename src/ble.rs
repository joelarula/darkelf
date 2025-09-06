use std::error::Error;

use std::future::Future;
use std::pin::Pin;

pub trait BleController: Send + Sync {
    
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    
    fn discover_characteristics<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    
    fn get_content(&self) -> String;
    
    fn is_connected(&self) -> bool;
}

