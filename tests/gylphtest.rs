
use darkelf::draw::DrawUtils;
use darkelf::command::CommandGenerator;
use darkelf::model::EncodedCommandData;
use ttf_parser::Face;

struct PathPrinter;


#[test]
fn test_glyphs() {
    


    let fontName = "Roboto-Bold.ttf";
    //let fontName = "laser.regular.ttf";
    let ttf_bytes = std::fs::read(format!("assets/fonts/{}", fontName)).unwrap();
    let face = Face::from_slice(&ttf_bytes, 0).unwrap();


    let text_data = DrawUtils::get_text_lines(&face, "A", None, None);

    let simplified_shapes = DrawUtils::layout_and_simplify_shapes(&text_data, false, true, true);


    // Serialize text_data to JSON and write to file
    let json = serde_json::to_string_pretty(&simplified_shapes).unwrap();
    std::fs::write("darkelf_coordinates.json", json).unwrap();
    println!("Wrote text_data to darkelf_coordinates.json");


    let (segment_points, grouped_segments, n, h, x_offset, group_point_counts, extra_floats) = DrawUtils::generate_segmented_layout_data(&simplified_shapes, 0.5, 0);
    let segment_pointsJson = serde_json::to_string_pretty(&segment_points).unwrap();
    std::fs::write("segment_points.json", segment_pointsJson).unwrap();
    println!("Wrote text_data to segment_points.json");
    
    println!("[TEST] n: {} (len {})", n, n.len());
    println!("[TEST] h: {} (len {})", h, h.len());
    println!("[TEST] x_offset: {}", x_offset);

    let  data: EncodedCommandData = DrawUtils::encode_layout_to_command_data(
        &simplified_shapes,
        5.0,
        0,
        Some(0),
    ).unwrap();

    let datapath = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write("encodedcommanddata.json", datapath).unwrap();
    println!("Wrote text_data to encodedcommanddata.json");


    let cmd_text = CommandGenerator::get_xys_cmd(&simplified_shapes);

    // Fine-grained protocol assertions
    // These indices are based on the expected command format and field lengths
    // Adjust indices if protocol changes

    //let char_width_hex = &cmd_text[138..158];
    //let char_point_hex = &cmd_text[168..188]; // 10 segments * 2 hex chars


    //assert_eq!(char_width_hex, "A8646464646464646464", "charWidthHex mismatch");
    //assert_eq!(char_point_hex, "11010101010101010101", "charPointHex mismatch");

    

    let verify_text = "A0A1A2A3001A0A81E9804F01820C804F1181D1004F1181BB004F1181BE00341081E9804F11819B804F0181C600341081C9004F1181B3004F118178804F11819B804F11819D801401819D802E1181F0802E1181F0801411819D801411813C00000080D800000080740000008010000000005400000000B8000000011C000000018000000001E4000000010AA86464646464646464641101010101010101010100000000000000000001010203040506070809090005A4A5A6A7";

    std::fs::write("rust_output.txt", &cmd_text).unwrap();
    std::fs::write("js_output.txt", &verify_text).unwrap();

    assert_eq!(cmd_text, verify_text, "cmd_text does not match verify_text");
    //println!("Wrote text_data to darkelf_coordinates.json");
}