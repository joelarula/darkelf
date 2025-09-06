// Usage example
fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = LaserController::new(false, false)?; // DMX mode, no mock
    let cmd = "A0A1A2A30000C0C1C2C3...A4A5A6A7"; // Example command
    controller.send(cmd, true, Some(&mut |status, progress| {
        println!("Status: {}, Progress: {}", status, progress);
    }))?;
    let response = "C0C1C2C300112233...C4C5C6C7"; // Example response
    controller.parse_response(response)?;
    println!("Parsed: {:?}", controller.project_data);
    Ok(())
}