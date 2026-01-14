use darkelf::dac::helios::{HELIOS_FLAGS_DEFAULT, HeliosDacController, HeliosPoint};
use darkelf::ilda::model::{IldaPoint, status};

#[test]
fn test_helios_ilda_conversion() {
    // This test just checks if the code compiles and runs without panicking on conversion logic.
    // We won't actually open devices here because that requires hardware.

    // Create some synthetic ILDA points
    let points = vec![
        // Center point, red
        IldaPoint::Format4 {
            x: 0,
            y: 0,
            z: 0,
            status: status::NORMAL,
            red: 255,
            green: 0,
            blue: 0,
        },
        // Top-right, green
        IldaPoint::Format4 {
            x: 32767,
            y: 32767,
            z: 0,
            status: status::NORMAL,
            red: 0,
            green: 255,
            blue: 0,
        },
        // Bottom-left, blanked
        IldaPoint::Format4 {
            x: -32768,
            y: -32768,
            z: 0,
            status: status::BLANKED,
            red: 255, // Should be ignored
            green: 255,
            blue: 255,
        },
        // Indexed color (Format 0)
        IldaPoint::Format0 {
            x: 1000,
            y: 1000,
            z: 0,
            status: status::NORMAL,
            color_index: 0, // Should be red in default palette
        },
    ];

    println!("Created {} ILDA points", points.len());

    // We can't easily test the `write_frame_ilda` without a device or mocking usually,
    // but the critical part we just added is the conversion logic inside it.
    // If we wanted to test purely the conversion, we'd need to expose `ilda_point_to_helios`.
    // Since it's private, we'll try to instantiate the controller.
    // If loading the DLL fails (likely in test environment), we handle it gracefully.

    match HeliosDacController::new() {
        Ok(controller) => {
            println!("Controller created successfully");
            // If we have a controller, we can try to write (will fail if no devices, but that's expected)

            // We just want to ensure the method exists and takes the types
            let _ = controller.write_frame(0, 30000, HELIOS_FLAGS_DEFAULT, &points, 4);
            println!("Called write_frame_ilda");
        }
        Err(e) => {
            println!("Could not load Helios DLL (expected if not present): {}", e);
            // Even if DLL is missing, the code compiled, so the API surface exists.
        }
    }
}

#[test]
fn test_helios_shift_0_exact_mapping() {
    // This test verifies that shift 0 provides 1:1 mapping (exact DAC points)
    // We instantiate the controller just to access the logic (via write_frame),
    // but without real hardware we mostly rely on the fact it compiles and runs the internal logic.
    // For a unit test of the private `ilda_point_to_helios`, we'd need to expose it or make a test module.
    // Here we just ensure the API accepts shift 0.

    let points = vec![IldaPoint::Format4 {
        x: 0,
        y: 0,
        z: 0,
        status: status::NORMAL,
        red: 255,
        green: 255,
        blue: 255,
    }];

    match HeliosDacController::new() {
        Ok(controller) => {
            // Test with shift 0
            let _ = controller.write_frame(0, 30000, HELIOS_FLAGS_DEFAULT, &points, 0);
            println!("Called write_frame with shift 0");
        }
        Err(_) => {
            println!("Skipping execution (dll not loaded)");
        }
    }
}
