
use darkelf::draw::DrawUtils;
use ttf_parser::Face;

struct PathPrinter;


#[test]
fn test_glyphs() {
    
    //let fontName = "Roboto-Bold.ttf";
       let fontName = "Fredoka dingbats.ttf";
    let ttf_bytes = std::fs::read(format!("assets/fonts/{}", fontName)).unwrap();
    let face = Face::from_slice(&ttf_bytes, 0).unwrap();


    let text_data = DrawUtils::get_text_lines(&face, "A", None, None);

    // Serialize text_data to JSON and write to file
    let json = serde_json::to_string_pretty(&text_data).unwrap();
    std::fs::write("test_glyphs_output.json", json).unwrap();
    println!("Wrote text_data to test_glyphs_output.json");


}