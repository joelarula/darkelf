/// This module provides a unified interface to choose between btleplug or Windows BLE implementations
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

/// Determines which BLE implementation to use
pub enum BleImplementation {
    /// Use the btleplug implementation (cross-platform)
    Btleplug,
    /// Use the Windows-specific implementation
    Windows,
}

/// A common trait for BLE controllers
pub trait BleControllerTrait: Send + Sync {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    fn discover_characteristics<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    fn get_content(&self) -> String;
    fn is_connected(&self) -> bool;
}

/// Get a BLE controller based on the specified implementation
pub async fn get_ble_controller(implementation: BleImplementation) -> Result<Box<dyn BleControllerTrait>, Box<dyn Error>> {
    match implementation {
        BleImplementation::Btleplug => {
            let controller = crate::blelib::BleController::new().await?;
            Ok(Box::new(BtleplugControllerWrapper(controller)))
        },
        BleImplementation::Windows => {
            let controller = crate::winble::BleController::new().await?;
            Ok(Box::new(WindowsControllerWrapper(controller)))
        }
    }
}

// Wrapper for btleplug implementation
pub struct BtleplugControllerWrapper(crate::blelib::BleController);

// Implementation of the common trait for btleplug
impl BleControllerTrait for BtleplugControllerWrapper {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.0.connect().await
        })
    }
    
    fn discover_characteristics<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.0.discover_characteristics().await
        })
    }
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            self.0.send(bytes).await
        })
    }
    
    fn get_content(&self) -> String {
        self.0.get_content()
    }
    
    fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}

// Wrapper for Windows implementation
pub struct WindowsControllerWrapper(crate::winble::BleController);

// Implementation of the common trait for Windows
impl BleControllerTrait for WindowsControllerWrapper {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.0.connect().await
        })
    }
    
    fn discover_characteristics<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.0.discover_characteristics().await
        })
    }
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            self.0.send(bytes).await
        })
    }
    
    fn get_content(&self) -> String {
        self.0.get_content()
    }
    
    fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}
