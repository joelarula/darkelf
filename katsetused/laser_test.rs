use std::error::Error;
use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;

use darkelf::ble_provider::{BleControllerTrait, BleImplementation, get_ble_controller};
use darkelf::laser::LaserController;

// Example of using LaserController with a real BLE implementation
async fn example_with_real_ble_implementation() -> Result<(), Box<dyn Error>> {
    // Get a real BLE controller implementation
    let ble_controller = get_ble_controller(BleImplementation::Windows).await?;
    
    // Create LaserController with the BLE controller
    let mut controller = LaserController::new(true, false, Some(ble_controller)).await?;
    
    // Now you can use the controller
    controller.discover_characteristics().await?;
    
    // Send a command
    controller.send("0102030405", true, None).await?;
    
    Ok(())
}

// Example of using LaserController with a mock BLE implementation
struct MockBleController {
    is_connected_flag: Arc<Mutex<bool>>,
}

impl MockBleController {
    fn new() -> Self {
        Self {
            is_connected_flag: Arc::new(Mutex::new(false)),
        }
    }
}

impl BleControllerTrait for MockBleController {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        let is_connected = self.is_connected_flag.clone();
        Box::pin(async move {
            // Simulate connection
            *is_connected.lock().unwrap() = true;
            Ok(())
        })
    }
    
    fn discover_characteristics<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            // Simulate discovery
            Ok(())
        })
    }
    
    fn send<'a>(&'a mut self, _bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            // Simulate sending
            println!("Mock: Sent bytes");
            Ok(())
        })
    }
    
    fn get_content(&self) -> String {
        "Mock content".to_string()
    }
    
    fn is_connected(&self) -> bool {
        *self.is_connected_flag.lock().unwrap()
    }
}

async fn example_with_mock_ble_implementation() -> Result<(), Box<dyn Error>> {
    // Create a mock BLE controller
    let mock_controller = Box::new(MockBleController::new());
    
    // Create LaserController with the mock controller
    let mut controller = LaserController::new(true, false, Some(mock_controller)).await?;
    
    // Now you can use the controller with the mock implementation
    controller.discover_characteristics().await?;
    
    // Send a command
    controller.send("0102030405", true, None).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running example with mock BLE implementation...");
    example_with_mock_ble_implementation().await?;
    
    println!("\nRunning example with real BLE implementation...");
    // This might fail if no real device is available
    if let Err(e) = example_with_real_ble_implementation().await {
        println!("Error with real implementation: {}", e);
    }
    
    Ok(())
}
