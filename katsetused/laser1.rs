use darkelf::laser::LaserController;
use darkelf::ble_provider::{BleImplementation, get_ble_controller};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a BLE controller implementation
    let ble_controller = get_ble_controller(BleImplementation::Windows).await?;
    
    // Create LaserController with the BLE controller
    let mut controller = LaserController::new(true, false, Some(ble_controller)).await?;
    
    controller.discover_characteristics().await?;
    Ok(())
}