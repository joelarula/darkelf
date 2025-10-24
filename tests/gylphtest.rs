
use darkelf::draw::DrawUtils;
use ttf_parser::Face;

struct PathPrinter;


#[test]
fn test_glyphs() {
    
    //let fontName = "Roboto-Bold.ttf";
    let fontName = "laser.regular.ttf";
    let ttf_bytes = std::fs::read(format!("assets/fonts/{}", fontName)).unwrap();
    let face = Face::from_slice(&ttf_bytes, 0).unwrap();


    let text_data = DrawUtils::get_text_lines(&face, "DARKELF", None, None);

    let simplified_shapes = DrawUtils::layout_and_simplify_shapes(&text_data.lines_arr, false, true, true);


    // Serialize text_data to JSON and write to file
    let json = serde_json::to_string_pretty(&simplified_shapes).unwrap();
    std::fs::write("darkelf_coordinates.json", json).unwrap();
    println!("Wrote text_data to darkelf_coordinates.json");


}