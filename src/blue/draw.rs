use crate::blue::blueprotocol::BlueProtocol;
use crate::blue::model::{
    BeamColor, PathCommand, Point, PolylineData
};
use ttf_parser::Face;
use ttf_parser::{OutlineBuilder};


pub struct DrawUtils;

impl DrawUtils {
   
    pub fn normalize_and_center_lines(lines_container: &PolylineData) -> PolylineData {
        let n = &lines_container.lines;
        let mut h = lines_container.w;
        let  a = lines_container.h;
        let mut left = 99999.0_f32;
        let mut top = 99999.0_f32;
        let mut right = -99999.0_f32;
        let mut bottom = -99999.0_f32;
        let mut width = 0.0_f32;
        let mut height = 0.0_f32;
        let mut x0 = 0.0_f32;
        let mut y0 = 0.0_f32;

        if n.is_empty() {
            left = 0.0;
            top = 0.0;
            right = 0.0;
            bottom = 0.0;
            width = 200.0;
            height = 200.0;
            x0 = 0.0;
            y0 = 0.0;
        } else {
            for o in n {
                for pt in o {
                    left = left.min(pt.x as f32);
                    top = top.min(pt.y as f32);
                    right = right.max(pt.x as f32);
                    bottom = bottom.max(pt.y as f32);
                }
            }
            width = right - left;
            height = bottom - top;
            x0 = left + width / 2.0;
            y0 = top + height / 2.0;
        }

        let mut p = Vec::new();
        for b in n {
            let mut g = Vec::new();
            for pt in b {
                let mut x = Point {
                    x: pt.x,
                    y: pt.y,
                    color: BeamColor::Blank,
                    pen_state: pt.pen_state,
                };
                //if is_horizontal_adjustment {
                 //   if flip_horizontal {
                 //       x.x = -x.x as f64 + 2.0 * x0 as f64 - left as f64 + 20.0;
                 //   } else {
                        x.x = x.x - left as f64 + 20.0;
                 //   }
                //} else {
                //    x.y = x.y - top as f64 + 20.0;
                //}
                g.push(x);
            }
            p.push(g);
        }
        
        h = width + 40.0;
         
        PolylineData {
            lines: p,
            w: h,
            h: a,
        }
    }

    pub fn layout_and_simplify_shapes( shapes: &[PolylineData],  mark_corners: bool, simplify: bool ) -> Vec<(usize, Vec<Point>, f32, f32)> {

        let normalized_shapes: Vec<PolylineData> = shapes
            .iter()
            .map(|shape| Self::normalize_and_center_lines(shape))
            .collect();


        let (mut total_width, mut total_height) = (0.0, 0.0);
        for shape in &normalized_shapes {
            //if is_horizontal_layout {
                total_width += shape.w;
                total_height = shape.h;
           // } else {
           //     total_width = shape.w;
           //     total_height += shape.h;
          //  }
        }

        let mut result = Vec::new();
        let  offset_x = -total_width / 2.0;
        let  offset_y = total_height / 2.0;
        let  mut layout_x = 0.0;
        let  layout_y = 0.0;

        for (shape_iter, shape) in normalized_shapes.iter().enumerate() {
            let lines = &shape.lines;
           // if !is_horizontal_layout {
           //     layout_x = -shape.w / 2.0;
           //     offset_x = 0.0;
          //  }
            for line in lines {
                let mut line = line.clone();
                let mut simplified_line = Vec::new();
                let mut first_point = Point {
                    x: offset_x as f64 + line[0].x as f64 + layout_x as f64,
                    y: offset_y as f64 - line[0].y as f64 + layout_y as f64,
                    color: BeamColor::Blank,
                    pen_state: 1,
                };
                if simplify {
                    if mark_corners {
                        line = Self::mark_corner_points(&mut line, 135.0, false);
                    } else {
                        let mut point_idx = 1;
                        while point_idx < line.len() {
                            let current_point = Point {
                                x: offset_x as f64 + line[point_idx].x as f64 + layout_x as f64,
                                y: offset_y as f64 - line[point_idx].y as f64 + layout_y as f64,
                                color: line[point_idx].color,
                                pen_state: line[point_idx].pen_state,
                            };
                            if Self::distance_between_points(&first_point, &current_point) < 2.0 {
                                line.remove(point_idx);
                            } else {
                                point_idx += 1;
                                first_point = current_point;
                            }
                        }
                        line = Self::mark_corner_points(&mut line, 145.0, true);
                    }
                }
                first_point = Point {
                    x: offset_x as f64 + line[0].x as f64 + layout_x as f64,
                    y: offset_y as f64 - line[0].y as f64 + layout_y as f64,
                    color: BeamColor::Blank,
                    pen_state: 1,
                };
                simplified_line.push(first_point.clone());
                let mut mid_idx = 1;
                while mid_idx < line.len() - 1 {
                    let mid_point = Point {
                        x: offset_x as f64 + line[mid_idx].x as f64 + layout_x as f64,
                        y: offset_y as f64 - line[mid_idx].y as f64 + layout_y as f64,
                        color: line[mid_idx].color,
                        pen_state: line[mid_idx].pen_state,
                    };
                    let next_point = Point {
                        x: offset_x as f64 + line[mid_idx + 1].x as f64 + layout_x as f64,
                        y: offset_y as f64 - line[mid_idx + 1].y as f64 + layout_y as f64,
                        color: line[mid_idx + 1].color,
                        pen_state: line[mid_idx + 1].pen_state,
                    };
                    if simplify {
                        let angle = Self::calculate_angle_between_points_b(
                            &first_point,
                            &mid_point,
                            &next_point,
                        );
                        if (angle == 0.0 || angle > 174.0) && mid_point.pen_state == 0 {
                            line.remove(mid_idx);
                            if mid_idx > 1 {
                                mid_idx -= 1;
                                simplified_line.pop();
                                first_point = simplified_line[simplified_line.len() - 1].clone();
                            }
                            continue;
                        }
                        if mid_point.pen_state == 0
                            && Self::distance_between_points(
                                &simplified_line[simplified_line.len() - 1],
                                &mid_point,
                            ) < 20.0
                        {
                            line.remove(mid_idx);
                            if mid_idx > 1 {
                                mid_idx -= 1;
                                simplified_line.pop();
                                first_point = simplified_line[simplified_line.len() - 1].clone();
                            }
                            continue;
                        }
                    }
                    simplified_line.push(mid_point.clone());
                    first_point = mid_point;
                    mid_idx += 1;
                }
                let last_point = Point {
                    x: offset_x as f64 + line[line.len() - 1].x as f64 + layout_x as f64,
                    y: offset_y as f64 - line[line.len() - 1].y as f64 + layout_y as f64,
                    color: line[line.len() - 1].color,
                    pen_state: line[line.len() - 1].pen_state,
                };
                simplified_line.push(last_point);
                result.push((shape_iter, simplified_line, shape.w, shape.h));
            }
            if lines.is_empty() {
                let placeholder = Point {
                    x: offset_x as f64 + shape.w as f64 / 2.0 + layout_x as f64,
                    y: 0.0,
                    color: BeamColor::Blank,
                    pen_state: 0,
                };
                result.push((shape_iter, vec![placeholder], shape.w, shape.h));
            }
           // if is_horizontal_layout {
                layout_x += shape.w;
           // } else {
           //     layout_y -= shape.h;
           // }
        }

        // Final simplification pass (optional, as in JS)
        if simplify && !mark_corners {
            for arr in &mut result {
                let line_arr = &mut arr.1;
                if line_arr.len() >= 4 {
                    let start_angle = Self::calculate_angle_between_points_b(
                        &line_arr[line_arr.len() - 2],
                        &line_arr[0],
                        &line_arr[1],
                    );
                    if start_angle > 145.0 || start_angle == 0.0 {
                        for corner_idx in 1..line_arr.len() - 1 {
                            if line_arr[corner_idx].pen_state == 1 {
                                let mut new_arr = Vec::new();
                                for i in corner_idx..line_arr.len() - 1 {
                                    new_arr.push(line_arr[i].clone());
                                }
                                for c in 0..=corner_idx {
                                    if c == 0 {
                                        line_arr[c].pen_state = 0;
                                    }
                                    new_arr.push(line_arr[c].clone());
                                }
                                if !new_arr.is_empty() {
                                    *line_arr = new_arr;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        result
    }

  
    fn distance_between_points(a: &Point, b: &Point) -> f32 {
        (((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()) as f32
    }


    fn mark_corner_points(
        points: &mut Vec<Point>,
        angle_threshold: f32,
        set_z: bool,
    ) -> Vec<Point> {
        if points.len() < 3 {
            return points.clone();
        }
        let mut point1 = Point {
            x: points[0].x,
            y: points[0].y,
            color: points[0].color,
            pen_state: points[0].pen_state,
        };
        for n in 1..points.len() - 1 {
            let h = Point {
                x: points[n].x,
                y: points[n].y,
                color: points[n].color,
                pen_state: points[n].pen_state,
            };
            let a = Point {
                x: points[n + 1].x,
                y: points[n + 1].y,
                color: points[n + 1].color,
                pen_state: points[n + 1].pen_state,
            };
            let i = Self::calculate_angle_between_points_b(&point1, &h, &a);
            if set_z || points[n].pen_state == 1 {
                points[n].pen_state = if i <= angle_threshold && i > 0.0 {
                    1
                } else {
                    0
                };
            }
            point1 = h;
        }
        points.clone()
    }


    fn calculate_angle_between_points_b(a: &Point, b: &Point, c: &Point) -> f32 {
        let n = [a.x - b.x, a.y - b.y];
        let h = [c.x - b.x, c.y - b.y];
        let dot_product = n[0] * h[0] + n[1] * h[1];
        let i = (n[0].powi(2) + n[1].powi(2)).sqrt();
        let c_len = (h[0].powi(2) + h[1].powi(2)).sqrt();
        if i == 0.0 || c_len == 0.0 {
            return 0.0;
        }
        let o = (dot_product / (i * c_len)).acos();
        let s = 180.0_f32 * o as f32 / std::f64::consts::PI as f32;
        s
    }
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

        let mut builder = Builder {
            commands: Vec::new(),
        };
        face.outline_glyph(glyph_id, &mut builder);
        builder.commands
    }

   
    fn append_to_array_or(arr: &mut Vec<Point>, pt: Point) -> bool {
        arr.push(pt);
        true
    }

    
    fn sample_quadratic_bezier(
        start: &Point,
        control: &Point,
        end: &Point,
        n: usize,
    ) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let t = t as f64;
            let x =
                (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control.x + t.powi(2) * end.x;
            let y =
                (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control.y + t.powi(2) * end.y;
            points.push(Point { x, y, color: BeamColor::Blank, pen_state: 0 });
        }
        points
    }

    fn parse_path_commands(
        path_commands: &[PathCommand],
        num_segments: usize,
    ) -> Vec<Vec<Point>> {
        let mut result = Vec::new();
        let mut current_poly = Vec::new();
        let mut h = 0;
        for cmd in path_commands {
            match cmd.cmd_type {
                'M' => {
                    let pt = Point {
                        x: cmd.x as f64,
                        y: cmd.y as f64,
                        color: BeamColor::Blank,
                        pen_state: 0,
                    };
                    if DrawUtils::append_to_array_or(&mut current_poly, pt) {
                        h += 1;
                    }
                }
                'L' => {
                    let pt = Point {
                        x: cmd.x as f64,
                        y: cmd.y as f64,
                        color: BeamColor::Blank,
                        pen_state: 0,
                    };
                    if DrawUtils::append_to_array_or(&mut current_poly, pt) {
                        h += 1;
                    }
                }
                'Q' => {
                    if let (Some(x1), Some(y1)) = (cmd.x1, cmd.y1) {
                        if let Some(last) = current_poly.last() {
                            let start = last.clone();
                            let control = Point { x: x1 as f64, y: y1 as f64, color: BeamColor::Blank, pen_state: 0 };
                            let end = Point {   
                                x: cmd.x as f64,
                                y: cmd.y as f64,
                                color: BeamColor::Blank,
                                pen_state: 0,
                            };
                            let bezier_points = DrawUtils::sample_quadratic_bezier(
                                &start,
                                &control,
                                &end,
                                num_segments,
                            );
                            for pt in bezier_points {
                                if DrawUtils::append_to_array_or(&mut current_poly, pt) {
                                    h += 1;
                                }
                            }
                        }
                    }
                }
                'Z' => {
                    if !current_poly.is_empty() {
                        let first = current_poly[0].clone();
                        // Always repeat the start point at the end for closure, with z=0
                        let mut first_closed = first.clone();
                        first_closed.pen_state = 0;
                        current_poly.push(first_closed);
                        result.push(current_poly.clone());
                        current_poly.clear();
                        h = 0;
                    }
                }
                _ => {}
            }
        }
        // If there is a polyline left open at the end, close it as well
        if !current_poly.is_empty() {
            let first = current_poly[0].clone();
            let mut first_closed = first.clone();
            first_closed.pen_state = 0;
            current_poly.push(first_closed);
            result.push(current_poly.clone());
        }
        result
    }

    pub fn get_text_lines(
        loaded_font: &Face, 
        text: &str
    ) -> Vec<PolylineData> {
        let num_segments = 5;
        let input_text = text.to_string();
        let mut lines = Vec::new();

        let reference_height = 316.66667_f32;
        for letter in input_text.chars() {
            // Get glyph id for the letter
            let glyph_id = match loaded_font.glyph_index(letter) {
                Some(id) => id,
                None => continue,
            };

            // For bounding box, use Face::glyph_bounding_box
            let bounding_box =
                loaded_font
                    .glyph_bounding_box(glyph_id)
                    .unwrap_or(ttf_parser::Rect {
                        x_min: 0,
                        y_min: 0,
                        x_max: 0,
                        y_max: 0,
                    });
            let glyph_width = (bounding_box.x_max - bounding_box.x_min) as f32;
            let glyph_height = (bounding_box.y_max - bounding_box.y_min) as f32;
            let y_max = bounding_box.y_max as f32;
            let x_min = bounding_box.x_min as f32;
            // Avoid division by zero
            let scale = if glyph_height != 0.0 {
                reference_height / glyph_height
            } else {
                1.0
            };

            // Convert outline to PathCommand vector
            let path_commands = DrawUtils::letter_to_path_commands(loaded_font, letter);
            let mut polyline = Vec::new();
            if letter != ' ' && !path_commands.is_empty() {
                let mut raw_polylines =
                    DrawUtils::parse_path_commands(&path_commands, num_segments);
                // Normalize/scaling and flip Y for each point in each polyline
                for poly in &mut raw_polylines {
                    let n = poly.len();
                    for (i, pt) in poly.iter_mut().enumerate() {
                        // Normalize X and Y to reference height, flip Y, and align baseline to y_min = 0
                        pt.x = ((pt.x as f32 - x_min) * scale) as f64;
                        pt.y = ((y_max as f64 - pt.y) * scale as f64); // flip Y, baseline at bottom
                        // Set pen state: z=0 for first and last, z=1 for others
                        if i == 0 || i == n - 1 {
                            pt.pen_state = 0;
                        } else {
                            pt.pen_state = 1;
                        }
                    }
                }
                polyline = raw_polylines;
            }

            lines.push(PolylineData {
                lines: polyline,
                w: glyph_width,
                h: reference_height,
            });
        }

        lines
    }

    pub fn generate_segmented_layout_data(
        segments: &Vec<(usize, Vec<Point>, f32, f32)>,
        scaling_factor: f32,
        mode: i32,
    ) -> (Vec<(usize, Vec<Point>, f32, f32)>, Vec<(usize, Vec<Point>, f32, f32)>, String, String, f32, Vec<usize>, Vec<f32>) {
        let mut n = -1_i32;
        let mut segment_widths: Vec<f32> = Vec::new();
        let mut segment_heights: Vec<f32> = Vec::new();
        let segment_default_size: f32 = 200.0;
        let mut total_segment_width: f32 = 0.0;
        let mut total_segment_height: f32 = 0.0;

        for seg in segments.iter() {
            let seg_id = seg.0 as i32;
            if n != seg_id {
                n = seg_id;
                segment_widths.push(seg.2 * scaling_factor);
                total_segment_width += seg.2;
                segment_heights.push(seg.3 * scaling_factor);
                total_segment_height += seg.3;
           }
        }

        let mut out = segments.clone();
        let mut grouped_segments: Vec<(usize, Vec<Point>, f32, f32)> = Vec::new();

        let mut protocol_segment_widths: Vec<f64> = segment_widths.iter().map(|&x| x as f64).collect();
        while protocol_segment_widths.len() < 12 {
            protocol_segment_widths.push(100.0);
        }

        let split_horizontal_segments = Self::split_into_segments_by_sum_limit(&protocol_segment_widths, 800.0);
        let mut segment_start_hex = String::new();
        let mut segment_count_hex = String::new();

        for (start, count) in split_horizontal_segments.iter() {
            segment_start_hex += &BlueProtocol::to_fixed_width_hex(
                BlueProtocol::clamp_value(*start as i32, 0, 255, 0), 2);
            segment_count_hex += &BlueProtocol::to_fixed_width_hex(
                BlueProtocol::clamp_value(*count as i32, 0, 255, 0), 2);
        }

    if mode == 127 {
            // ...existing code for vertical mode...
            let mut d = 0.0;
            let mut b: Vec<(usize, Vec<Point>, f32, f32)> = Vec::new();
            for i in 0..9 {
                n += 1;
                let pt = Point {
                    x: 0.0,
                    y: total_segment_height as f64 / 2.0 + segment_default_size as f64 / 2.0 + d as f64,
                    color: BeamColor::Blank,
                    pen_state: 0,
                };
                b.push((n as usize, vec![pt], segment_default_size, segment_default_size));
                d += segment_default_size;
                segment_heights.push(segment_default_size * scaling_factor);
            }
            out.extend(b);

            // ...existing code for grouping...
            let segment_heights_f64: Vec<f64> = segment_heights.iter().map(|&x| x as f64).collect();
            let splited_segments = Self::split_into_segments_by_sum_limit(&segment_heights_f64, 800.0);
            let mut v = String::new();
            let mut f = String::new();
            for (start, count) in splited_segments.iter() {
                v += &BlueProtocol::to_fixed_width_hex(
                    BlueProtocol::clamp_value(*start as i32, 0, 255, 0), 2);
                f += &BlueProtocol::to_fixed_width_hex(
                    BlueProtocol::clamp_value(*count as i32, 0, 255, 0), 2);
                let group = out[*start..(*start + *count)].to_vec();
                let mut merged_points = Vec::new();
                let mut total_width = 0.0;
                let mut total_height = 0.0;
                let mut group_idx = group.first().map(|seg| seg.0).unwrap_or(0);
                for seg in &group {
                    merged_points.extend_from_slice(&seg.1);
                    total_width += seg.2;
                    total_height += seg.3;
                }
                grouped_segments.push((group_idx, merged_points, total_width, total_height));
            }
            let x_offset = -d * scaling_factor / 2.0;
            let mut group_point_counts: Vec<usize> = Vec::new();
            for group in &grouped_segments {
                let count = group.1.len();
                group_point_counts.push(count);
            }

            (out, grouped_segments, segment_start_hex, segment_count_hex, x_offset, group_point_counts, segment_heights)
        } else {
   
            let mut k = 0.0;
            let mut m: Vec<(usize, Vec<Point>, f32, f32)> = Vec::new();
            for p in 0..9 {
                n += 1;
                let pt = Point {
                    x: total_segment_width as f64 / 2.0 + segment_default_size as f64 / 2.0 + k as f64,
                    y: 0.0,
                    color: BeamColor::Blank,
                    pen_state: 0,
                };
                m.push((n as usize, vec![pt], segment_default_size, segment_default_size));
                k += segment_default_size;
                segment_widths.push(segment_default_size * scaling_factor);
            }
            out.extend(m);

            let segment_widths_f64: Vec<f64> = segment_widths.iter().map(|&x| x as f64).collect();
            let x = Self::split_into_segments_by_sum_limit(&segment_widths_f64, 800.0);
            let mut n_str = String::new();
            let mut h = String::new();
            for (start, count) in x.iter() {
                n_str += &BlueProtocol::to_fixed_width_hex(
                    BlueProtocol::clamp_value(*start as i32, 0, 255, 0), 2);
                h += &BlueProtocol::to_fixed_width_hex(
                    BlueProtocol::clamp_value(*count as i32, 0, 255, 0), 2);
                let group = out[*start..(*start + *count)].to_vec();
                let mut merged_points = Vec::new();
                let mut total_width = 0.0;
                let mut total_height = 0.0;
                let mut group_idx = group.first().map(|seg| seg.0).unwrap_or(0);
                for seg in &group {
                    merged_points.extend_from_slice(&seg.1);
                    total_width += seg.2;
                    total_height += seg.3;
                }
                grouped_segments.push((group_idx, merged_points, total_width, total_height));
            }
            let x_offset = -k * scaling_factor / 2.0;
            let mut group_point_counts: Vec<usize> = Vec::new();
            for group in &grouped_segments {
                let count = group.1.len();
                group_point_counts.push(count);
            }
            // Pad n_str and h to 24 hex chars (12 segments) by prepending zeros if needed
            while n_str.len() < 24 {
                n_str = format!("00{}", n_str);
            }
            while h.len() < 24 {
                h = format!("00{}", h);
            }

            (out, grouped_segments, n_str, h, x_offset, group_point_counts, segment_widths)
        }
    }



pub fn split_into_segments_by_sum_limit(numbers: &[f64], limit: f64) -> Vec<(usize, usize)> {

    let mut result = Vec::new();
    let mut current_sum = 0.0;
    let mut segment_start_idx = 0;
    let mut segment_size = 0;
    
    // Match JS algorithm: push a range on EVERY iteration, creating cumulative overlapping groups
    for (number_idx, &num) in numbers.iter().enumerate() {
     
        if current_sum + num <= limit {
            segment_size += 1;
            result.push((segment_start_idx, segment_size));
            current_sum += num;
        } else {
            // Need to adjust the window by removing segments from the start
            let mut temp_sum = current_sum;
            loop {
                if temp_sum <= limit {
                    segment_size += 1;
                    result.push((segment_start_idx, segment_size));
                    current_sum = temp_sum + num;
                    break;
                }
                if temp_sum > limit && temp_sum - numbers[segment_start_idx] < limit {
                    segment_size += 1;
                    result.push((segment_start_idx, segment_size));
                    current_sum += num;
                    break;
                }
                temp_sum -= numbers[segment_start_idx];
                current_sum -= numbers[segment_start_idx];
                segment_start_idx += 1;
                segment_size -= 1;
           }
        }
    }
    
    result
}







}
