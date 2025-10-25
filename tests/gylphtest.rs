
use darkelf::draw::DrawUtils;
use darkelf::command::CommandGenerator;
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


    let cmd_text = CommandGenerator::get_xys_cmd(&simplified_shapes);


    let verify_text = "A0A1A2A3001A0A81E9804F01820C804F1181D1004F1181BB004F1181BE00341081E9804F11819B804F0181C600341081C9004F1181B3004F118178804F11819B804F11819D801401819D802E1181F0802E1181F0801411819D801411813C00000080D800000080740000008010000000005400000000B8000000011C000000018000000001E4000000010AA86464646464646464641101010101010101010100000000000000000001010203040506070809090005A4A5A6A7";

    std::fs::write("rust_output.txt", &cmd_text).unwrap();
    std::fs::write("js_output.txt", &verify_text).unwrap();

    assert_eq!(cmd_text, verify_text, "cmd_text does not match verify_text");
    println!("Wrote text_data to darkelf_coordinates.json");
}