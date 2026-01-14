use darkelf::dac::helios::{HELIOS_FLAGS_DEFAULT, HeliosDacController};
use darkelf::ilda::model::IldaSection;
use darkelf::ilda::reader::read_ilda_from_bytes;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn test_play_dna_ilda() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;

    if num_devices == 0 {
        println!("No Helios devices found, skipping playback test.");
        return Ok(());
    }

    // Load ILDA file
    let file_path = "tests/data/DNA.ild";
    let bytes = fs::read(file_path)?;
    let ilda_file = read_ilda_from_bytes(&bytes)?;

    println!(
        "Loaded ILDA file: {} ({} sections)",
        file_path,
        ilda_file.sections.len()
    );

    // Extract all frames, skipping empty ones and logging details
    let mut frames = Vec::new();
    for (i, section) in ilda_file.sections.iter().enumerate() {
        match section {
            IldaSection::Frame { header, points } => {
                println!(
                    "Section {}: FRAME - Format {:?}, Name: '{}', Records: {}",
                    i, header.format_code, header.frame_or_palette_name, header.num_records
                );
                if points.is_empty() {
                    println!("  -> Skipping empty frame");
                    continue;
                }
                println!("  -> Found {} points", points.len());
                frames.push(points.clone());
            }
            IldaSection::Palette { header, colors } => {
                println!(
                    "Section {}: PALETTE - Format {:?}, Name: '{}', Colors: {}",
                    i,
                    header.format_code,
                    header.frame_or_palette_name,
                    colors.len()
                );
            }
        }
    }

    if frames.is_empty() {
        println!("No frames found in ILDA file!");
        return Ok(());
    }

    println!("Found {} frames", frames.len());

    // Play for 10 seconds
    let total_start = Instant::now();
    let play_duration = Duration::from_secs(10);
    let mut frame_count = 0;

    println!("Playing DNA.ild for 10 seconds...");

    let mut frames_iter = frames.iter().enumerate().cycle();

    while total_start.elapsed() < play_duration {
        let (i, points) = frames_iter.next().unwrap();

        if i == 0 {
            println!("Animation loop starting (Frame 0)...");
        }

        // Wait for DAC to be ready
        let mut attempts = 0;
        while attempts < 1000 {
            if controller.get_status(0)? {
                break;
            }
            attempts += 1;
            std::thread::yield_now();
        }

        if attempts == 1000 {
            println!("Warning: DAC not ready after 1000 attempts, skipping frame.");
            continue;
        }

        // Write frame with default shift 4 (1x zoom)
        controller.write_frame(0, 30000, HELIOS_FLAGS_DEFAULT, points, 4)?;
        frame_count += 1;
    }

    println!("Finished playing. Total frames sent: {}", frame_count);

    controller.stop(0)?;
    controller.close_devices()?;

    Ok(())
}
