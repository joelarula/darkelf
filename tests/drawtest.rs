use svg::node::element::Element;

use darkelf::model::DisplayColor;
//use lyon::math::{point};
//use lyon::path::Path;
//use lyon::tessellation::{StrokeTessellator, StrokeOptions, StrokeVertex, VertexBuffers};
use tiny_skia::{Pixmap, Paint, Stroke, PathBuilder, Color};

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;


#[derive(Debug, Clone, Copy)]
pub enum SvgAttr {
    Fill,
    Stroke,
    StrokeWidth,
    D,
}

impl SvgAttr {
    pub fn name(&self) -> &'static str {
        match self {
            SvgAttr::Fill => "fill",
            SvgAttr::Stroke => "stroke",
            SvgAttr::StrokeWidth => "stroke-width",
            SvgAttr::D => "d",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SvgValue {
    None,
}

impl SvgValue {
    pub fn name(&self) -> &'static str {
        match self {
            SvgValue::None => "none",
        }
    }
}


fn traverse(element: &Element, depth: usize) {
    let indent = "  ".repeat(depth);
    print!("{}<{}", indent, element.get_name());

    for (name, value) in element.get_attributes() {
        print!(" {}=\"{}\"", name, value);
    }

    if element.get_children().is_empty() {
        println!(" />");
    } else {
        println!(">");
        for child in element.get_children() {
            // Try to downcast to Element
           // if let Some(child_element) = child.clone().downcast_ref::<Element>() {
           //     traverse(child_element, depth + 1);
           // } else if let Some(text_node) = child.clone().downcast_ref::<svg::node::Text>() {
           //     let t = text_node.text().trim();
           //     if !t.is_empty() {
           //         println!("{}{}", "  ".repeat(depth + 1), t);
           //     }
           // }
        }
        println!("</{}>", element.get_name());
    }
}

#[test]
fn test_line_shape() {


    /*
    ---- Polyline data: [x, y, color_index, flag] ----
    [
        [-400, 0, 0, 1],
        [-360, 0, 1, 0],
        [-320, 0, 1, 0],
        [-280, 0, 1, 0],
        [-240, 0, 4, 0],
        [-200, 0, 4, 0],
        [-160, 0, 4, 0],
        [-120, 0, 2, 0],
        [-80, 0, 2, 0],
        [-40, 0, 2, 0],
        [0, 0, 5, 0],
        [40, 0, 5, 0],
        [80, 0, 5, 0],
        [120, 0, 6, 0],
        [160, 0, 6, 0],
        [200, 0, 6, 0],
        [240, 0, 3, 0],
        [280, 0, 3, 0],
        [320, 0, 3, 0],
        [360, 0, 7, 0],
        [400, 0, 7, 1]
    ]
    ---- End polyline data ----
    


    typedef struct
{
	std::uint16_t x; // Unsigned 12 bit (valid values from 0 to 0xFFF). X position.
	std::uint16_t y; // Unsigned 12 bit (valid values from 0 to 0xFFF). Y position.
	std::uint8_t r;	// Unsigned 8 bit (valid values from 0 to 0xFF). Red.
	std::uint8_t g;	// Unsigned 8 bit (valid values from 0 to 0xFF). Green.
	std::uint8_t b;	// Unsigned 8 bit (valid values from 0 to 0xFF). Blue.
	std::uint8_t i;	// Unsigned 8 bit (valid values from 0 to 0xFF). Intensity. Optional and should be set to max value if not used.
} HeliosPoint;
    
    
    */


    let line = Data::new()
        .move_to((-400, 0))
        .line_by((400, 0))
        .close();

    let path = Path::new()
        .set(SvgAttr::Fill.name(), SvgValue::None.name())
        .set(SvgAttr::Stroke.name(), DisplayColor::Green.name())
        .set(SvgAttr::StrokeWidth.name(), 1)
        .set(SvgAttr::D.name(), line);


    let document = Document::new()
        .set("viewBox", (0, 0, 800, 800))
        .add(path);

    // Process all <path> elements in the SVG document
    //for node in document.iter() {
    //    if let Some(root) = node.clone().downcast_ref::<Element>() {
    //        traverse(root, 0);
    //    }
    //}





    // 1. Create a lyon path for a line
    //let mut builder = Path::builder();
    //builder.begin(point(-400.0, 0.0));
    //builder.line_to(point(400.0, 0.0));


    //builder.end(false);
   // let path = builder.build();



    // 2. Prepare a Pixmap (canvas) to draw on
    let width = 800;
    let height = 800;
    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::BLACK);


    // 3. Draw the first half of the line in one color, second half in another
    let (x0, y0) = (20.0, 20.0);
    let (x1, y1) = (180.0, 180.0);
    let mx = (x0 + x1) / 2.0;
    let my = (y0 + y1) / 2.0;

    // First half: start to midpoint
    let mut pb1 = PathBuilder::new();
    pb1.move_to(x0, y0);
    pb1.line_to(mx, my);
    let path1 = pb1.finish().unwrap();

    // Second half: midpoint to end
    let mut pb2 = PathBuilder::new();
    pb2.move_to(mx, my);
    pb2.line_to(x1, y1);
    let path2 = pb2.finish().unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 2.0;

    // First half: red
    let mut paint1 = Paint::default();
    paint1.set_color(Color::from_rgba8(255, 0, 0, 255));
    pixmap.stroke_path(&path1, &paint1, &stroke, tiny_skia::Transform::identity(), None);

    // Second half: blue
    let mut paint2 = Paint::default();
    paint2.set_color(Color::from_rgba8(0, 0, 255, 255));
    pixmap.stroke_path(&path2, &paint2, &stroke, tiny_skia::Transform::identity(), None);

    // 5. Save as PNG using the `image` crate
    let png_data = pixmap.encode_png().unwrap();
    std::fs::write("line.png", &png_data).unwrap();
    println!("Saved to line.png");
}