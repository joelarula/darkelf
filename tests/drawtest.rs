
use darkelf::ilda::builder::*;
use darkelf::ilda::model::{IldaFormatCode, IldaSection, palette, status};
use darkelf::ilda::builder::HeaderBuilder;


use darkelf::model::DisplayColor;
//use lyon::math::{point};
//use lyon::path::Path;
//use lyon::tessellation::{StrokeTessellator, StrokeOptions, StrokeVertex, VertexBuffers};
use tiny_skia::{Pixmap, Paint, Stroke, PathBuilder, Color};



#[test]
fn test_line_shape() {



    

    // 1. Create a lyon path for a line
//    let mut builder = Path::builder();
 //   builder.begin(point(-400.0, 0.0));
 //   builder.line_to(point(400.0, 0.0));
 //   builder.end(false);
 //   let path = builder.build();



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

#[test]
fn build_single_triangle_ilda_frame() {



    // Build triangle points (closed triangle)
    let p1 = IldaPointBuilder::new_format4_with_color(-400, -400, 0, status::NORMAL, palette::GREEN).build();
    let p2 = IldaPointBuilder::new_format4_with_color(0, 400, 0, status::NORMAL, palette::GREEN).build();
    let p3 = IldaPointBuilder::new_format4_with_color(400, -400, 0, status::NORMAL, palette::GREEN).build();
    
    // Build section
    let section = SectionBuilder::new_frame_with_projection(Projection { width: 800, height: 800}, 20.0)
        .add_point(p1)
        .add_point(p2)
        .add_point(p3)
        .build();
    
    match &section {
        IldaSection::Frame { points, .. } => {
            for point in points {
                println!("{:?}", point);


            }
        }
        _ => {
            println!("Section is not a Frame variant");
        }
    }

}