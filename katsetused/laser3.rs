#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = LaserController::new(true, false).await?;
    controller.get_ble_characteristics().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(true, false).await?;
    controller.send("E0E1E2E30000E4E5E6E7", true, None).await?; // Example query
    tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Allow notifications
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(true, false).await?;
    controller.start_test("0000ffe1-0000-1000-8000-00805f9b34fb"); // Example UUID
    controller.send("E0E1E2E30000C0C1C2C3DATA...C4C5C6C7E4E5E6E7", true, None).await?;
    tokio::time::sleep(std::time::Duration::from_secs(6)).await; // Allow test completion
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(true, false).await?;
    let mut test = DeviceTesting {
        read_uuid: "0000ffe1-0000-1000-8000-00805f9b34fb".to_string(),
        test_status: "Starting".to_string(),
        test_result: "".to_string(),
        test_msg: "".to_string(),
        can_send: false,
        conn: true,
    };
    controller.device_testing = Some(test);
    controller.start_test1().await;
    tokio::time::sleep(Duration::from_secs(15)).await; // Allow test completion
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(true, false).await?;
    let test = DeviceTesting {
        device_id: "example_device_id".to_string(),
        service_id: "example_service_id".to_string(),
        read_uuid: "0000ffe1-0000-1000-8000-00805f9b34fb".to_string(),
        write_uuid: "0000ffe2-0000-1000-8000-00805f9b34fb".to_string(),
        test_status: "Init".to_string(),
        test_result: "".to_string(),
        test_msg: "".to_string(),
        conn: false,
    };
    controller.device_tested.push(test);
    controller.do_start(0).await;
    tokio::time::sleep(Duration::from_secs(15)).await; // Allow test completion
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(true, false).await?;
    let test = DeviceTesting {
        device_id: "example_device_id".to_string(),
        service_id: "example_service_id".to_string(),
        read_uuid: "0000ffe1-0000-1000-8000-00805f9b34fb".to_string(),
        write_uuid: "0000ffe2-0000-1000-8000-00805f9b34fb".to_string(),
        test_status: "Init".to_string(),
        test_result: "".to_string(),
        test_msg: "".to_string(),
        can_send: false,
        conn: false,
    };
    controller.device_tested.push(test);
    controller.do_start(0).await;
    tokio::time::sleep(Duration::from_secs(15)).await;
    Ok(())
}


use laser_control::{LaserController, CommandGenerator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = LaserController::new(true, false).await?;
    let mut generator = CommandGenerator::new();
    let points = vec![LaserPoint::new(100, 100, 0), LaserPoint::new(200, 200, 1)];
    let options = LaserOptions::new();
    if let Some(cmd) = generator.generate_command(vec![(0, points)], 1.0, options, 0, 0) {
        controller.send(&cmd.to_hex_string(), true, None).await?;
    }
    Ok(())
}