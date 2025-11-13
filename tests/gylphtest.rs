
use darkelf::draw::DrawUtils;
use darkelf::blueprotocol::BlueProtocol;
use darkelf::model::EncodedCommandData;
use ttf_parser::Face;

struct PathPrinter;


#[test]
fn test_glyphs() {
    


    //let fontName = "Roboto-Bold.ttf";
    let fontName = "laser.regular.ttf";
    let ttf_bytes = std::fs::read(format!("assets/fonts/{}", fontName)).unwrap();
    let face = Face::from_slice(&ttf_bytes, 0).unwrap();


    let text_data = DrawUtils::get_text_lines(&face, "ABC");

    let simplified_shapes = DrawUtils::layout_and_simplify_shapes(&text_data, false, true, true);


    // Serialize text_data to JSON and write to file
    let json = serde_json::to_string_pretty(&simplified_shapes).unwrap();
    std::fs::write("darkelf_coordinates.json", json).unwrap();
    println!("Wrote text_data to darkelf_coordinates.json");


    let (segment_points, grouped_segments, n, h, x_offset, group_point_counts, extra_floats) = DrawUtils::generate_segmented_layout_data(&simplified_shapes, 0.5, 0);
    let segment_pointsJson = serde_json::to_string_pretty(&segment_points).unwrap();
    std::fs::write("segment_points.json", segment_pointsJson).unwrap();
    println!("Wrote text_data to segment_points.json");

    let  data: EncodedCommandData = BlueProtocol::encode_layout_to_command_data(
        &simplified_shapes,
        5.0,
    ).unwrap();

    let datapath = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write("encodedcommanddata.json", datapath).unwrap();
    println!("Wrote text_data to encodedcommanddata.json");


   let se1 = "000000000000000000010103";
   let se2 = "010203040506070809090a09";
   assert_eq!(data.se1,se1);
   assert_eq!(data.se2,se2);

   // let verify_cmd = "81e9804f01820c804f1181d1004f1181bb004f1181be00341081e9804f11819b804f0181c600341081c9004f1181b3004f118178804f11819b804f11819d801401819d802e1181f0802e1181f0801411819d801411813c00000080d800000080740000008010000000005400000000b8000000011c000000018000000001e4000000";
   let verify_cmd_b = "822d004f018243004f118287804f11826e804f11825c80241182158024118202804f1181e9804f11822d004f11821e800d018252800d118238003011821e800d11816000490181d5004f2181d5804f21816b804f218160004921817600170181be00172181be0038218175003821817600172181be000001816a00002181be80382181be00002180fd8020018117802030814c803831814500383180fd002031814c004f318145804f3180fd80203180c1000000805d0000000007000000006b00000000cf0000000133000000019700000001fb000000025f000000";



    assert_eq!(verify_cmd_b, data.cmd);


    let cmd_text = BlueProtocol::pack_xys_cmd(&simplified_shapes, 5.0);

    // Fine-grained protocol assertions
    // These indices are based on the expected command format and field lengths
    // Adjust indices if protocol changes

    //let char_width_hex = &cmd_text[138..158];
    //let char_point_hex = &cmd_text[168..188]; // 10 segments * 2 hex chars


    //assert_eq!(char_width_hex, "A8646464646464646464", "charWidthHex mismatch");
    //assert_eq!(char_point_hex, "11010101010101010101", "charPointHex mismatch");

    let verify_cmd_b = "A0A1A2A3002C0C822D004F018243004F118287804F11826E804F11825C80241182158024118202804F1181E9804F11822D004F11821E800D018252800D118238003011821E800D11816000490181D5004F2181D5804F21816B804F218160004921817600170181BE00172181BE0038218175003821817600172181BE000001816A00002181BE80382181BE00002180FD8020018117802030814C803831814500383180FD002031814C004F318145804F3180FD80203180C1000000805D0000000007000000006B00000000CF0000000133000000019700000001FB000000025F000000010CB28A636464646464646464640D0E08010101010101010101000000000000000000010103010203040506070809090A090005A4A5A6A7";
   

  //  let verify_text = "A0A1A2A3001A0A81E9804F01820C804F1181D1004F1181BB004F1181BE00341081E9804F11819B804F0181C600341081C9004F1181B3004F118178804F11819B804F11819D801401819D802E1181F0802E1181F0801411819D801411813C00000080D800000080740000008010000000005400000000B8000000011C000000018000000001E4000000010AA86464646464646464641101010101010101010100000000000000000001010203040506070809090005A4A5A6A7";

    //std::fs::write("rust_output.txt", &cmd_text).unwrap();
    //std::fs::write("js_output.txt", &verify_text).unwrap();

    assert_eq!(cmd_text, verify_cmd_b, "cmd_text does not match verify_text");
    //println!("Wrote text_data to darkelf_coordinates.json");
}