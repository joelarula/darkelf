
use crate::model::{DrawData, DrawItem, DrawMode, DrawPoint, MirroredPolylines, PathCommand, Point, PolyPoint, PolylineData, TextLinesResult};
use ttf_parser::Face;
use std::collections::HashMap;
use ttf_parser::{GlyphId, OutlineBuilder};



const REFERENCE_COORDINATE_SIZE: f64 = 800.0;


pub struct DrawUtils;

impl DrawUtils {
    /// Converts a single letter to a vector of PathCommand using ttf_parser::Face
    pub fn letter_to_path_commands(face: &Face, letter: char) -> Vec<PathCommand> {
        // Get glyph index for the letter
        let glyph_id = match face.glyph_index(letter) {
            Some(id) => id,
            None => return Vec::new(),
        };

        // Custom builder to collect path commands
        struct Builder {
            commands: Vec<PathCommand>,
        }

        impl OutlineBuilder for Builder {
            fn move_to(&mut self, x: f32, y: f32) {
                self.commands.push(PathCommand {
                    cmd_type: 'M',
                    x: x,
                    y: y,
                    x1: None,
                    y1: None,
                });
            }
            fn line_to(&mut self, x: f32, y: f32) {
                self.commands.push(PathCommand {
                    cmd_type: 'L',
                    x: x,
                    y: y,
                    x1: None,
                    y1: None,
                });
            }
            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.commands.push(PathCommand {
                    cmd_type: 'Q',
                    x: x,
                    y: y,
                    x1: Some(x1),
                    y1: Some(y1),
                });
            }
            fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
                // Not used for TrueType fonts, but could be added if needed
            }
            fn close(&mut self) {
                self.commands.push(PathCommand {
                    cmd_type: 'Z',
                    x: 0.0,
                    y: 0.0,
                    x1: None,
                    y1: None,
                });
            }
        }

        let mut builder = Builder { commands: Vec::new() };
        face.outline_glyph(glyph_id, &mut builder);
        builder.commands
    }

    
    pub fn prepare_draw_data(draw_data: &DrawData, width: f64) -> Vec<Point> {
        let mut points = Vec::new();
        

        for draw_object in &draw_data.draw_points {
            let object_points = match draw_object.draw_mode {
                DrawMode::Polylines => {
                    Self::draw_all_transformed_polylines(draw_object, width)
                }
                DrawMode::Text => {
                    Self::draw_transformed_text(draw_object, width)
                }
                _ => {
                    Self::draw_transformed_object(draw_object, width)
                }
            };
            
            // Concatenate results (points = points.concat(currentDrawResult))
            points.extend(object_points);
        }
        
        points
    }
    

    fn draw_transformed_object(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let rotated_points = Self::rotate_points_around_bounding_box_center(&draw_object.get_all_points(), draw_object.ang);
        
        let mut result_points = Vec::new();
        let scaling_factor = REFERENCE_COORDINATE_SIZE / width;  // scalingFactor = REFERENCE_COORDINATE_SIZE / width
        let center_offset_x = width / 2.0;   // centerOffsetX = width / 2
        let position_x = draw_object.x0;     // positionX = drawObject.x0
        let position_y = draw_object.y0;     // positionY = drawObject.y0
        let scale_z = draw_object.z;         // scaleZ = drawObject.z
        
        // Color calculation logic
        let base_line_color = draw_object.line_color as i32;  // baseLineColor = drawObject.lineColor
        let color_segment_index = base_line_color - 9;        // colorSegmentIndex = baseLineColor - 9
        let mut current_color_index = if base_line_color >= 8 { -1 } else { base_line_color }; // currentColorIndex = baseLineColor >= 8 ? -1 : baseLineColor
        
 
        for (point_index, rotated_point) in rotated_points.iter().enumerate() {
 
            if color_segment_index < 0 {
                current_color_index = if base_line_color >= 8 { 
                    current_color_index + 1 
                } else { 
                    current_color_index 
                };
                current_color_index = if current_color_index >= 8 { 1 } else { current_color_index };
            } else {
                current_color_index = 1;
            }
            
            let final_color = if rotated_point.color != 0 {
                if color_segment_index < 0 {
                    current_color_index as u8
                } else if color_segment_index == 0 {
                    rotated_point.color  // Keep original color
                } else {
                    current_color_index as u8
                }
            } else {
                rotated_point.color
            };
            
            let result_x = rotated_point.x * scaling_factor * scale_z + (position_x - center_offset_x) * scaling_factor;
            let result_y = rotated_point.y * scale_z * scaling_factor + (-position_y + center_offset_x) * scaling_factor;
            let result_color = if point_index == 0 { 0 } else { final_color };
            let result_pen_state = rotated_point.pen_state;
            
            result_points.push(Point::new(result_x, result_y, result_color, result_pen_state));
        }
        
        result_points
    }

    fn draw_all_transformed_polylines(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let mut accumulated_results = Vec::new();
        
        if let crate::model::DrawPoints::Polylines(polylines) = &draw_object.ps {
            for (index, _polyline) in polylines.iter().enumerate() {
                let current_polyline_result = Self::draw_transformed_polyline(draw_object, index, width);
                accumulated_results.extend(current_polyline_result);
            }
        }
        
        accumulated_results
    }


    fn draw_transformed_polyline(draw_object: &DrawItem, index: usize, width: f64) -> Vec<Point> {
        // Get the specific polyline at the given index
        if let crate::model::DrawPoints::Polylines(polylines) = &draw_object.ps {
            if let Some(polyline) = polylines.get(index) {
                // Rotate points around bounding box center (passing true for polyline mode)
                let rotated_points = Self::rotate_points_around_bounding_box_center_polyline(polyline, draw_object.ang);
                
                let base_line_color = draw_object.line_color;
                let mut current_color_index = if base_line_color >= 8 { 1 } else { base_line_color };
                let color_segment_index = base_line_color.wrapping_sub(9); // This will wrap around for values < 9
                
                let position_x = draw_object.x0;
                let position_y = draw_object.y0;
                let scale_z = draw_object.z;
                
                let scaling_factor = REFERENCE_COORDINATE_SIZE / width;
                let center_offset = width / 2.0;
                let mut result_points = Vec::new();
                
                for (k, rotated_point) in rotated_points.iter().enumerate() {
                    // Transform the point coordinates
                    let transformed_x = rotated_point.x * scale_z + position_x;
                    let transformed_y = rotated_point.y * scale_z + position_y;
                    
                    // Handle color logic
                    if base_line_color >= 8 {
                        if color_segment_index < 0 { // This handles the wrapping case
                            current_color_index += 1;
                            if current_color_index >= 8 {
                                current_color_index = 1;
                            }
                        }
                        // TODO: Handle color segment array logic when we have color segments
                    }
                    
                    // Create result point with coordinate transformation
                    let final_x = (transformed_x - center_offset) * scaling_factor;
                    let final_y = (center_offset - transformed_y) * scaling_factor;
                    let color = if k == 0 { 0 } else { current_color_index };
                    let pen_state = rotated_point.pen_state;
                    
                    result_points.push(Point::from_js_array(final_x, final_y, color as f64, pen_state as f64));
                }
                
                return result_points;
            }
        }
        
        vec![]
    }

    /// Rotate points around bounding box center for a single polyline
    fn rotate_points_around_bounding_box_center_polyline(points: &[DrawPoint], angle_degrees: f64) -> Vec<DrawPoint> {
        // This is similar to the regular rotation but for a single polyline
        if points.is_empty() || angle_degrees == 0.0 {
            return points.to_vec();
        }

        // Calculate bounding box
        let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        // Rotate each point around the bounding box center
        points.iter()
            .map(|point| {
                let (rotated_x, rotated_y) = Self::rotate_point_around_center(angle_degrees, center_x, center_y, point.x, point.y);
                DrawPoint {
                    x: rotated_x,
                    y: rotated_y,
                    color: point.color,
                    pen_state: point.pen_state,
                }
            })
            .collect()
    }
    
    /// Placeholder for drawTransformedText2 (drawMode == 9999)  
    fn draw_transformed_text(_draw_object: &DrawItem, _width: f64) -> Vec<Point> {
        // TODO: Implement text transformation
        Vec::new()
    }
    
    /// Rust implementation of JavaScript rotatePointsAroundBoundingBoxCenter
    fn rotate_points_around_bounding_box_center(points: &[DrawPoint], angle: f64) -> Vec<DrawPoint> {
        if points.is_empty() {
            return Vec::new();
        }
        
        let mut rotated_points = Vec::new();
        
        // Calculate bounding box
        let mut left = f64::MAX;
        let mut top = f64::MAX; 
        let mut right = f64::MIN;
        let mut bottom = f64::MIN;
        
        for point in points {
            let x = point.x;
            let y = -point.y;  // JavaScript uses -point.y for bounding box calculation
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }
        
        // Calculate center of bounding box
        let center_x = (right - left) / 2.0 + left;   // (a.right - a.left) / 2 + a.left
        let center_y = (bottom - top) / 2.0 + top;    // (a.bottom - a.top) / 2 + a.top
        
        // Rotate each point around the center
        for point in points {
            let x = point.x;
            let y = -point.y;  // JavaScript uses -point.y
            
            let rotated = Self::rotate_point_around_center(angle, center_x, center_y, x, y);
            
            rotated_points.push(DrawPoint::new(
                rotated.0,      // rotated x 
                -rotated.1,     // -rotated y (flip back)
                point.color,
                point.pen_state
            ));
        }
        
        rotated_points
    }
    
    /// Rust implementation of JavaScript rotatePointAroundCenter
    fn rotate_point_around_center(angle: f64, center_x: f64, center_y: f64, point_x: f64, point_y: f64) -> (f64, f64) {
        // JavaScript: function rotatePointAroundCenter(e, t, r, n, h)
        // var a = n - t, i = h - r
        let a = point_x - center_x;
        let i = point_y - center_y;
        
        // c = t + (a * Math.cos(e) - i * Math.sin(e))
        // o = r + (a * Math.sin(e) + i * Math.cos(e))
        let c = center_x + (a * angle.cos() - i * angle.sin());
        let o = center_y + (a * angle.sin() + i * angle.cos());
        
        (c, o)
    }



    ///fn load_font_data(font_name: &str) -> Vec<u8> {
    //    let ttf_bytes = std::fs::read(font_name).unwrap();
    //    ttf_bytes
    //}
/
    

fn sample_quadratic_bezier(start: &PolyPoint, control: &PolyPoint, end: &PolyPoint, n: usize) -> Vec<PolyPoint> {
    let mut points = Vec::new();
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control.x + t.powi(2) * end.x;
        let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control.y + t.powi(2) * end.y;
        points.push(PolyPoint { x, y, z: 0 });
    }
    points
}

fn append_to_array_or(arr: &mut Vec<PolyPoint>, pt: PolyPoint) -> bool {
    arr.push(pt);
    true
}

 fn parse_path_commands(path_commands: &[PathCommand], num_segments: usize) -> Vec<Vec<PolyPoint>> {
    let mut result = Vec::new();
    let mut current_poly = Vec::new();
    let mut h = 0;
    for cmd in path_commands {
        match cmd.cmd_type {
            'M' => {
                let pt = PolyPoint { x: cmd.x, y: cmd.y, z: 1 };
                if DrawUtils::append_to_array_or(&mut current_poly, pt) { h += 1; }
            }
            'L' => {
                let pt = PolyPoint { x: cmd.x, y: cmd.y, z: 1 };
                if DrawUtils::append_to_array_or(&mut current_poly, pt) { h += 1; }
            }
            'Q' => {
                if let (Some(x1), Some(y1)) = (cmd.x1, cmd.y1) {
                    if let Some(last) = current_poly.last() {
                        let start = last.clone();
                        let control = PolyPoint { x: x1, y: y1, z: 0 };
                        let end = PolyPoint { x: cmd.x, y: cmd.y, z: 0 };
                        let bezier_points = DrawUtils::sample_quadratic_bezier(&start, &control, &end, num_segments);
                        for pt in bezier_points {
                            if DrawUtils::append_to_array_or(&mut current_poly, pt) { h += 1; }
                        }
                    }
                }
            }
            'Z' => {
                if !current_poly.is_empty() {
                    let first = current_poly[0].clone();
                    let last = current_poly.last().unwrap().clone();
                    let mut first_closed = first.clone();
                    first_closed.z = 0;
                    if last.z == 999 { current_poly.pop(); }
                    if current_poly.len() - h > 2 { current_poly.push(first_closed); }
                    result.push(current_poly.clone());
                    current_poly.clear();
                    h = 0;
                }
            }
            _ => {}
        }
    }
    result
}

fn transform_polylines_for_vertical_mirroring(
    input_polylines: &[Vec<PolyPoint>],
    _unused: f32,
    width: f32,
    height: f32,
) -> MirroredPolylines {
    let mut new_lines_up = Vec::new();
    let mut new_lines_down = Vec::new();
    for polyline in input_polylines {
        let mut up = Vec::new();
        let mut down = Vec::new();
        for pt in polyline {
            up.push(PolyPoint {
                x: pt.y,
                y: -pt.x + width / 2.0 + 0.4 * height,
                z: pt.z,
            });
            down.push(PolyPoint {
                x: -pt.y,
                y: -pt.x + width / 2.0 + 0.4 * height,
                z: pt.z,
            });
        }
        new_lines_up.push(up);
        new_lines_down.push(down);
    }
    MirroredPolylines {
        new_lines_up,
        new_lines_down,
    }
}



pub fn get_text_lines(
    loaded_font: &Face, // your font type
    text: &str,
    number_of_segments: Option<usize>,
    generate_mirror_lines: Option<bool>,
) -> TextLinesResult {
    let num_segments = number_of_segments.unwrap_or(5);
    let mirror_lines = generate_mirror_lines.unwrap_or(false);
    let font_size = 400.0;
    let input_text = text.to_string();


    let mut lines_arr = Vec::new();
    let mut lines_arr_up = Vec::new();
    let mut lines_arr_down = Vec::new();

    for letter in input_text.chars() {
        // Get glyph id for the letter
        let glyph_id = match loaded_font.glyph_index(letter) {
            Some(id) => id,
            None => continue,
        };

        // Use your own builder logic as in letter_to_path_commands
        // For bounding box, use Face::glyph_bounding_box
        let bounding_box = loaded_font.glyph_bounding_box(glyph_id).unwrap_or(ttf_parser::Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0 });
        let mut glyph_height = (bounding_box.y_min.abs() + bounding_box.y_max.abs()) as f32;
        let mut glyph_width = (bounding_box.x_min.abs() + bounding_box.x_max.abs()) as f32;
        if glyph_width == 0.0 { glyph_width = font_size as f32 / 2.0; }
        if glyph_height == 0.0 { glyph_height = font_size as f32; } else { glyph_height *= 1.1; }

        // Convert outline to PathCommand vector
        let path_commands = DrawUtils::letter_to_path_commands(loaded_font, letter);
        let mut polyline = Vec::new();
        if letter != ' ' && !path_commands.is_empty() {
            polyline = DrawUtils::parse_path_commands(&path_commands, num_segments);
        }

        if mirror_lines {
            let mirrored = DrawUtils::transform_polylines_for_vertical_mirroring(&polyline, 0.0, glyph_width, font_size as f32);
            lines_arr_up.push(PolylineData {
                lines: mirrored.new_lines_up,
                w: glyph_width,
                h: glyph_height,
            });
            lines_arr_down.push(PolylineData {
                lines: mirrored.new_lines_down,
                w: glyph_width,
                h: glyph_height,
            });
        }
        lines_arr.push(PolylineData {
            lines: polyline,
            w: glyph_width,
            h: glyph_height,
        });

    }

    TextLinesResult {
        lines_arr,
        lines_arr_up,
        lines_arr_down
    }
}


}





