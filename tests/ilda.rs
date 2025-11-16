#[test]
fn test_read_grid_ilda() {
	use std::fs;
	use std::path::Path;
	use darkelf::ilda::reader::read_ilda_from_bytes;

	let path = Path::new("tests/data/Grid Test.ild");
	let mut txt_path = path.to_path_buf();
	txt_path.set_extension("txt");
	let mut json_path = path.to_path_buf();
	json_path.set_extension("json");

	let debug_str = match fs::read(path) {
		Ok(bytes) => match read_ilda_from_bytes(&bytes) {
			Ok(ilda_file) => format!("Parsed ILDA file: {:#?}", ilda_file),
			Err(e) => format!("Failed to parse ILDA file: {}", e),
		},
		Err(e) => format!("Failed to read ILDA file: {}", e),
	};
	let json_str = match fs::read(path) {
		Ok(bytes) => match read_ilda_from_bytes(&bytes) {
			Ok(ref ilda_file) => {
				serde_json::to_string_pretty(ilda_file).unwrap_or_else(|e| format!("Failed to serialize ILDA file to JSON: {}", e))
			},
			Err(e) => format!("Failed to parse ILDA file: {}", e),
		},
		Err(e) => format!("Failed to read ILDA file: {}", e),
	};

	if let Err(e) = fs::write(&txt_path, &debug_str) {
		panic!("Failed to write debug output: {}", e);
	}
	println!("Wrote debug output to {}", txt_path.display());
	if let Err(e) = fs::write(&json_path, &json_str) {
		panic!("Failed to write JSON output: {}", e);
	}
	println!("Wrote JSON output to {}", json_path.display());
}
