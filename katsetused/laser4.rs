use laser_control::LaserController;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the LaserController with BLE and mock mode disabled
    let mut controller = LaserController::new(true, false).await?;

    // Define the device testing configuration (mocked UUIDs for example)
    let test = laser_control::DeviceTesting {
        device_id: "example_device_id".to_string(),
        service_id: "example_service_id".to_string(),
        read_uuid: "0000ffe1-0000-1000-8000-00805f9b34fb".to_string(),
        write_uuid: "0000ffe2-0000-1000-8000-00805f9b34fb".to_string(),
        test_status: "Init".to_string(),
        test_result: "".to_string(),
        test_msg: "".to_string(),
        can_send: true,
        conn: false,
    };
    controller.device_tested.push(test.clone());
    controller.device_testing = Some(test);

    // Connect to the BLE device
    if let Err(e) = controller.connect_ble().await {
        eprintln!("Failed to connect BLE: {}", e);
        return Err(e.into());
    }
    println!("Connected to BLE device at 11:20 PM EEST, September 05, 2025");

    // Define animation sequence range (117 to 126)
    let animation_range = 117..=126;
    let ch2_speed = 100;  // Speed value (0-255), medium speed
    let ch7_param = 200;  // Additional parameter (e.g., high color randomization)

    // Iterate through each animation sequence
    for ch6_effect in animation_range {
        println!("Setting animation sequence: CH6 = {}", ch6_effect);

        // Build the DMX-like payload: Header + CH1-CH8 + Footer
        let payload = vec![
            0x00,          // CH1 (default 0)
            ch2_speed,     // CH2: Speed
            0x00,          // CH3 (default 0)
            0x00,          // CH4 (default 0)
            0x00,          // CH5 (default 0)
            ch6_effect,    // CH6: Animation effect (one of 117-126)
            ch7_param,     // CH7: Additional parameter (color randomization depth)
            0x00,          // CH8 (default 0)
        ];
        let cmd_hex = format!(
            "E0E1E2E3{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}E4E5E6E7",
            payload[0], payload[1], payload[2], payload[3],
            payload[4], payload[5], payload[6], payload[7]
        );
        println!("Sending command: {}", cmd_hex);

        // Send the command
        if let Err(e) = controller.send(&cmd_hex, true, None).await {
            eprintln!("Failed to send command for CH6 = {}: {}", ch6_effect, e);
        } else {
            println!("Command for CH6 = {} sent successfully", ch6_effect);
        }

        // Wait 5 seconds to observe each animation
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    // Start a test sequence to monitor the final effect
    if let Some(testing) = controller.device_testing.as_mut() {
        controller.start_test(testing, 10).await;
        println!("Monitoring animation sequences");
    }

    // Keep the program running for a while to observe the last effect
    tokio::time::sleep(Duration::from_secs(30)).await;
    println!("Animation sequence test completed");

    // Cleanup
    controller.close_cnn_and_run(None).await;
    controller.close_bluetooth_adapter().await;

    Ok(())
}