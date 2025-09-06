# DarkElf - Laser Control Library

This library provides a unified interface for controlling laser devices using either Bluetooth Low Energy (BLE) or Serial/DMX connections.

## LaserController API

The `LaserController` class has been refactored to accept a BLE implementation through dependency injection, allowing for better testing and more flexible configuration.

### Creating a LaserController

```rust
use darkelf::laser::LaserController;
use darkelf::ble_provider::{BleImplementation, get_ble_controller};

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Get a specific BLE controller implementation
    let ble_controller = get_ble_controller(BleImplementation::Windows).await?;
    
    // Create LaserController with the BLE controller
    // Parameters:
    // - is_ble: true to use BLE, false to use Serial/DMX
    // - mock_mode: true to run in mock mode (no actual device communication)
    // - ble_controller: The BLE controller implementation to use
    let mut controller = LaserController::new(true, false, Some(ble_controller)).await?;
    
    // Use the controller
    controller.discover_characteristics().await?;
    controller.send("0102030405", true, None).await?;
    
    Ok(())
}
```

### Using Mock Implementations for Testing

You can create a mock implementation of the `BleControllerTrait` for testing:

```rust
struct MockBleController {
    // Your mock implementation
}

impl BleControllerTrait for MockBleController {
    // Implement the required methods
}

async fn test_with_mock() -> Result<(), Box<dyn std::error::Error>> {
    let mock_controller = Box::new(MockBleController::new());
    let mut controller = LaserController::new(true, false, Some(mock_controller)).await?;
    
    // Test your code with the mock controller
    
    Ok(())
}
```

### Available BLE Implementations

The library provides two built-in BLE implementations:

1. `BleImplementation::Windows` - Uses Windows-specific BLE APIs
2. `BleImplementation::Btleplug` - Uses the cross-platform btleplug crate

Choose the appropriate implementation based on your platform and requirements.

## Serial/DMX Mode

To use Serial/DMX mode instead of BLE:

```rust
let mut controller = LaserController::new(false, false, None).await?;
```

## Running in Mock Mode

For testing without actual hardware:

```rust
let mut controller = LaserController::new(true, true, None).await?;
```

This allows you to develop and test your application without connecting to real hardware.
