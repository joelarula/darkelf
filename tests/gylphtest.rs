
use ttf_parser::Face;

struct PathPrinter;

impl ttf_parser::OutlineBuilder for PathPrinter {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("Move to ({}, {})", x, y);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        println!("Line to ({}, {})", x, y);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!("Quad to ({}, {}), ({}, {})", x1, y1, x, y);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("Curve to ({}, {}), ({}, {}), ({}, {})", x1, y1, x2, y2, x, y);
    }
    fn close(&mut self) {
        println!("Close path");
    }
}

#[test]
fn test_glyphs() {
    let ttf_bytes = std::fs::read("refactor target/Roboto-Bold.ttf").unwrap();
    let face = Face::from_slice(&ttf_bytes, 0).unwrap();
    let glyph_id = face.glyph_index('A').unwrap();
    println!("Glyph index: {}", glyph_id.0);

    // Print outline path commands
    let mut printer = PathPrinter;
    face.outline_glyph(glyph_id, &mut printer);
}