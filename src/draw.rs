use crate::model::{
    DrawData, DrawItem, DrawMode, DrawPoint, MirroredPolylines, PathCommand, Point, PolyPoint,
    PolylineData,
};
use std::collections::HashMap;
use ttf_parser::Face;
use ttf_parser::{GlyphId, OutlineBuilder};

const REFERENCE_COORDINATE_SIZE: f64 = 800.0;

pub struct DrawUtils;

impl DrawUtils {
    /// Normalizes and centers lines, similar to JS normalizeAndCenterLines.
    pub fn normalize_and_center_lines(
        lines_container: &PolylineData,
        is_horizontal_adjustment: bool,
        flip_horizontal: bool,
    ) -> PolylineData {
        let n = &lines_container.lines;
        let mut h = lines_container.w;
        let mut a = lines_container.h;
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
                    left = left.min(pt.x);
                    top = top.min(pt.y);
                    right = right.max(pt.x);
                    bottom = bottom.max(pt.y);
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
                let mut x = PolyPoint {
                    x: pt.x,
                    y: pt.y,
                    z: pt.z,
                };
                if is_horizontal_adjustment {
                    if flip_horizontal {
                        x.x = -x.x + 2.0 * x0 - left + 20.0;
                    } else {
                        x.x = x.x - left + 20.0;
                    }
                } else {
                    x.y = x.y - top + 20.0;
                }
                g.push(x);
            }
            p.push(g);
        }
        if is_horizontal_adjustment {
            h = width + 40.0;
        } else {
            a = height + 40.0;
        }
        PolylineData {
            lines: p,
            w: h,
            h: a,
        }
    }

    pub fn layout_and_simplify_shapes(
        shapes: &[PolylineData],
        mark_corners: bool,
        is_horizontal_layout: bool,
        simplify: bool,
    ) -> Vec<(usize, Vec<PolyPoint>, f32, f32)> {
        // 1. Normalize and center each shape (match JS logic)
        let normalized_shapes: Vec<PolylineData> = shapes
            .iter()
            .map(|shape| Self::normalize_and_center_lines(shape, is_horizontal_layout, false))
            .collect();

        // 2. Calculate total width/height
        let (mut total_width, mut total_height) = (0.0, 0.0);
        for shape in &normalized_shapes {
            if is_horizontal_layout {
                total_width += shape.w;
                total_height = shape.h;
            } else {
                total_width = shape.w;
                total_height += shape.h;
            }
        }

        let mut result = Vec::new();
        let mut offset_x = -total_width / 2.0;
        let mut offset_y = total_height / 2.0;
        let mut layout_x = 0.0;
        let mut layout_y = 0.0;

        for (shape_iter, shape) in normalized_shapes.iter().enumerate() {
            let lines = &shape.lines;
            if !is_horizontal_layout {
                layout_x = -shape.w / 2.0;
                offset_x = 0.0;
            }
            for line in lines {
                let mut line = line.clone();
                let mut simplified_line = Vec::new();
                let mut first_point = PolyPoint {
                    x: offset_x + line[0].x + layout_x,
                    y: offset_y - line[0].y + layout_y,
                    z: 1,
                };
                if simplify {
                    if mark_corners {
                        line = Self::mark_corner_points(&mut line, 135.0, false);
                    } else {
                        let mut point_idx = 1;
                        while point_idx < line.len() {
                            let current_point = PolyPoint {
                                x: offset_x + line[point_idx].x + layout_x,
                                y: offset_y - line[point_idx].y + layout_y,
                                z: line[point_idx].z,
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
                first_point = PolyPoint {
                    x: offset_x + line[0].x + layout_x,
                    y: offset_y - line[0].y + layout_y,
                    z: 1,
                };
                simplified_line.push(first_point.clone());
                let mut mid_idx = 1;
                while mid_idx < line.len() - 1 {
                    let mid_point = PolyPoint {
                        x: offset_x + line[mid_idx].x + layout_x,
                        y: offset_y - line[mid_idx].y + layout_y,
                        z: line[mid_idx].z,
                    };
                    let next_point = PolyPoint {
                        x: offset_x + line[mid_idx + 1].x + layout_x,
                        y: offset_y - line[mid_idx + 1].y + layout_y,
                        z: line[mid_idx + 1].z,
                    };
                    if simplify {
                        let angle = Self::calculate_angle_between_points_b(
                            &first_point,
                            &mid_point,
                            &next_point,
                        );
                        if (angle == 0.0 || angle > 174.0) && mid_point.z == 0 {
                            line.remove(mid_idx);
                            if mid_idx > 1 {
                                mid_idx -= 1;
                                simplified_line.pop();
                                first_point = simplified_line[simplified_line.len() - 1].clone();
                            }
                            continue;
                        }
                        if mid_point.z == 0
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
                let last_point = PolyPoint {
                    x: offset_x + line[line.len() - 1].x + layout_x,
                    y: offset_y - line[line.len() - 1].y + layout_y,
                    z: 1,
                };
                simplified_line.push(last_point);
                result.push((shape_iter, simplified_line, shape.w, shape.h));
            }
            if lines.is_empty() {
                let placeholder = PolyPoint {
                    x: offset_x + shape.w / 2.0 + layout_x,
                    y: 0.0,
                    z: 0,
                };
                result.push((shape_iter, vec![placeholder], shape.w, shape.h));
            }
            if is_horizontal_layout {
                layout_x += shape.w;
            } else {
                layout_y -= shape.h;
            }
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
                            if line_arr[corner_idx].z == 1 {
                                let mut new_arr = Vec::new();
                                for i in corner_idx..line_arr.len() - 1 {
                                    new_arr.push(line_arr[i].clone());
                                }
                                for c in 0..=corner_idx {
                                    if c == 0 {
                                        line_arr[c].z = 0;
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

    // Dummy stub for distance_between_points
    fn distance_between_points(a: &PolyPoint, b: &PolyPoint) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }

    /// Marks corner points in a polyline based on angle threshold, similar to JS markCornerPoints.
    fn mark_corner_points(
        points: &mut Vec<PolyPoint>,
        angle_threshold: f32,
        set_z: bool,
    ) -> Vec<PolyPoint> {
        if points.len() < 3 {
            return points.clone();
        }
        let mut point1 = PolyPoint {
            x: points[0].x,
            y: points[0].y,
            z: 1,
        };
        for n in 1..points.len() - 1 {
            let h = PolyPoint {
                x: points[n].x,
                y: points[n].y,
                z: points[n].z,
            };
            let a = PolyPoint {
                x: points[n + 1].x,
                y: points[n + 1].y,
                z: points[n + 1].z,
            };
            let i = Self::calculate_angle_between_points_b(&point1, &h, &a);
            if set_z || points[n].z == 1 {
                points[n].z = if i <= angle_threshold && i > 0.0 {
                    1
                } else {
                    0
                };
            }
            point1 = h;
        }
        points.clone()
    }

    /// Calculates the angle (in degrees) between three points: a, b, c.
    /// Equivalent to JS calculateAngleBetweenPoints_B.
    fn calculate_angle_between_points_b(a: &PolyPoint, b: &PolyPoint, c: &PolyPoint) -> f32 {
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

    pub fn prepare_draw_data(draw_data: &DrawData, width: f64) -> Vec<Point> {
        let mut points = Vec::new();

        for draw_object in &draw_data.draw_points {
            let object_points = match draw_object.draw_mode {
                DrawMode::Polylines => Self::draw_all_transformed_polylines(draw_object, width),
                DrawMode::Text => Self::draw_transformed_text(draw_object, width),
                _ => Self::draw_transformed_object(draw_object, width),
            };

            // Concatenate results (points = points.concat(currentDrawResult))
            points.extend(object_points);
        }

        points
    }

    fn draw_transformed_object(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let rotated_points = Self::rotate_points_around_bounding_box_center(
            &draw_object.get_all_points(),
            draw_object.ang,
        );

        let mut result_points = Vec::new();
        let scaling_factor = REFERENCE_COORDINATE_SIZE / width; // scalingFactor = REFERENCE_COORDINATE_SIZE / width
        let center_offset_x = width / 2.0; // centerOffsetX = width / 2
        let position_x = draw_object.x0; // positionX = drawObject.x0
        let position_y = draw_object.y0; // positionY = drawObject.y0
        let scale_z = draw_object.z; // scaleZ = drawObject.z

        // Color calculation logic
        let base_line_color = draw_object.line_color as i32; // baseLineColor = drawObject.lineColor
        let color_segment_index = base_line_color - 9; // colorSegmentIndex = baseLineColor - 9
        let mut current_color_index = if base_line_color >= 8 {
            -1
        } else {
            base_line_color
        }; // currentColorIndex = baseLineColor >= 8 ? -1 : baseLineColor

        for (point_index, rotated_point) in rotated_points.iter().enumerate() {
            if color_segment_index < 0 {
                current_color_index = if base_line_color >= 8 {
                    current_color_index + 1
                } else {
                    current_color_index
                };
                current_color_index = if current_color_index >= 8 {
                    1
                } else {
                    current_color_index
                };
            } else {
                current_color_index = 1;
            }

            let final_color = if rotated_point.color != 0 {
                if color_segment_index < 0 {
                    current_color_index as u8
                } else if color_segment_index == 0 {
                    rotated_point.color // Keep original color
                } else {
                    current_color_index as u8
                }
            } else {
                rotated_point.color
            };

            let result_x = rotated_point.x * scaling_factor * scale_z
                + (position_x - center_offset_x) * scaling_factor;
            let result_y = rotated_point.y * scale_z * scaling_factor
                + (-position_y + center_offset_x) * scaling_factor;
            let result_color = if point_index == 0 { 0 } else { final_color };
            let result_pen_state = rotated_point.pen_state;

            result_points.push(Point::new(
                result_x,
                result_y,
                result_color,
                result_pen_state,
            ));
        }

        result_points
    }

    fn draw_all_transformed_polylines(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let mut accumulated_results = Vec::new();

        if let crate::model::DrawPoints::Polylines(polylines) = &draw_object.ps {
            for (index, _polyline) in polylines.iter().enumerate() {
                let current_polyline_result =
                    Self::draw_transformed_polyline(draw_object, index, width);
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
                let rotated_points = Self::rotate_points_around_bounding_box_center_polyline(
                    polyline,
                    draw_object.ang,
                );

                let base_line_color = draw_object.line_color;
                let mut current_color_index = if base_line_color >= 8 {
                    1
                } else {
                    base_line_color
                };
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
                        if color_segment_index < 0 {
                            // This handles the wrapping case
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

                    result_points.push(Point::from_js_array(
                        final_x,
                        final_y,
                        color as f64,
                        pen_state as f64,
                    ));
                }

                return result_points;
            }
        }

        vec![]
    }

    /// Rotate points around bounding box center for a single polyline
    fn rotate_points_around_bounding_box_center_polyline(
        points: &[DrawPoint],
        angle_degrees: f64,
    ) -> Vec<DrawPoint> {
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
        points
            .iter()
            .map(|point| {
                let (rotated_x, rotated_y) = Self::rotate_point_around_center(
                    angle_degrees,
                    center_x,
                    center_y,
                    point.x,
                    point.y,
                );
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
    fn rotate_points_around_bounding_box_center(
        points: &[DrawPoint],
        angle: f64,
    ) -> Vec<DrawPoint> {
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
            let y = -point.y; // JavaScript uses -point.y for bounding box calculation
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }

        // Calculate center of bounding box
        let center_x = (right - left) / 2.0 + left; // (a.right - a.left) / 2 + a.left
        let center_y = (bottom - top) / 2.0 + top; // (a.bottom - a.top) / 2 + a.top

        // Rotate each point around the center
        for point in points {
            let x = point.x;
            let y = -point.y; // JavaScript uses -point.y

            let rotated = Self::rotate_point_around_center(angle, center_x, center_y, x, y);

            rotated_points.push(DrawPoint::new(
                rotated.0,  // rotated x
                -rotated.1, // -rotated y (flip back)
                point.color,
                point.pen_state,
            ));
        }

        rotated_points
    }

    /// Rust implementation of JavaScript rotatePointAroundCenter
    fn rotate_point_around_center(
        angle: f64,
        center_x: f64,
        center_y: f64,
        point_x: f64,
        point_y: f64,
    ) -> (f64, f64) {
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

    fn sample_quadratic_bezier(
        start: &PolyPoint,
        control: &PolyPoint,
        end: &PolyPoint,
        n: usize,
    ) -> Vec<PolyPoint> {
        let mut points = Vec::new();
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let x =
                (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control.x + t.powi(2) * end.x;
            let y =
                (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control.y + t.powi(2) * end.y;
            points.push(PolyPoint { x, y, z: 0 });
        }
        points
    }

    fn append_to_array_or(arr: &mut Vec<PolyPoint>, pt: PolyPoint) -> bool {
        arr.push(pt);
        true
    }

    fn parse_path_commands(
        path_commands: &[PathCommand],
        num_segments: usize,
    ) -> Vec<Vec<PolyPoint>> {
        let mut result = Vec::new();
        let mut current_poly = Vec::new();
        let mut h = 0;
        for cmd in path_commands {
            match cmd.cmd_type {
                'M' => {
                    let pt = PolyPoint {
                        x: cmd.x,
                        y: cmd.y,
                        z: 1,
                    };
                    if DrawUtils::append_to_array_or(&mut current_poly, pt) {
                        h += 1;
                    }
                }
                'L' => {
                    let pt = PolyPoint {
                        x: cmd.x,
                        y: cmd.y,
                        z: 1,
                    };
                    if DrawUtils::append_to_array_or(&mut current_poly, pt) {
                        h += 1;
                    }
                }
                'Q' => {
                    if let (Some(x1), Some(y1)) = (cmd.x1, cmd.y1) {
                        if let Some(last) = current_poly.last() {
                            let start = last.clone();
                            let control = PolyPoint { x: x1, y: y1, z: 0 };
                            let end = PolyPoint {
                                x: cmd.x,
                                y: cmd.y,
                                z: 0,
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
                        first_closed.z = 0;
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
            first_closed.z = 0;
            current_poly.push(first_closed);
            result.push(current_poly.clone());
        }
        result
    }

    pub fn get_text_lines(
        loaded_font: &Face, // your font type
        text: &str,
        number_of_segments: Option<usize>,
        generate_mirror_lines: Option<bool>,
    ) -> Vec<PolylineData> {
        let num_segments = number_of_segments.unwrap_or(5);
        let mirror_lines = generate_mirror_lines.unwrap_or(false);
        let font_size = 400.0;
        let input_text = text.to_string();
        let mut lines = Vec::new();

        // Reference height to normalize to (from JS output, e.g. 316.666...)
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
                        pt.x = (pt.x - x_min) * scale;
                        pt.y = (y_max - pt.y) * scale; // flip Y, baseline at bottom
                        // Set pen state: z=0 for first and last, z=1 for others
                        if i == 0 || i == n - 1 {
                            pt.z = 0;
                        } else {
                            pt.z = 1;
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

     /// Generate segmented layout data matching the JS generateSegmentedLayoutData behavior
    pub fn generate_segmented_layout_data(
        segments: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
        scaling_factor: f32,
        mode: i32,
    ) -> (Vec<(usize, Vec<PolyPoint>, f32, f32)>, String, String, f32) {
        let mut n = -1_i32;
        let mut segment_widths: Vec<f32> = Vec::new();
        let mut segment_heights: Vec<f32> = Vec::new();
        let segment_default_size: f32 = 200.0;
        let mut total_segment_width: f32 = 0.0;
        let mut total_segment_height: f32 = 0.0;

        // Collect widths/heights for real segments
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

        if mode == 127 {
            // JS: vertical filler segments
            let mut d = 0.0;
            let mut b: Vec<(usize, Vec<PolyPoint>, f32, f32)> = Vec::new();
            for i in 0..9 {
                n += 1;
                let pt = PolyPoint {
                    x: 0.0,
                    y: total_segment_height / 2.0 + segment_default_size / 2.0 + d,
                    z: 0,
                };
                b.push((n as usize, vec![pt], segment_default_size, segment_default_size));
                d += segment_default_size;
                segment_heights.push(segment_default_size * scaling_factor);
            }
            out.extend(b);

            // JS: split heights for vertical mode
            let segment_heights_f64: Vec<f64> = segment_heights.iter().map(|&x| x as f64).collect();
            let splited_segments = Self::split_into_segments_by_sum_limit(&segment_heights_f64, 800.0);
            let mut V = String::new();
            let mut f = String::new();
            for (start, count) in splited_segments.iter() {
                V += &Self::to_fixed_width_hex_b(*start as i32, 2);
                f += &Self::to_fixed_width_hex_b(*count as i32, 2);
            }
            let x_offset = -d * scaling_factor / 2.0;
            // Debug output for vertical mode
            println!("[generate_segmented_layout_data] mode=127 (vertical)");
            println!("  segment_heights: {:?}", segment_heights);
            println!("  V: {} (len {})", V, V.len());
            println!("  f: {} (len {})", f, f.len());
            println!("  x_offset: {}", x_offset);
            return (out, V, f, x_offset);
        }

        // JS: horizontal filler segments
        let mut k = 0.0;
        let mut m: Vec<(usize, Vec<PolyPoint>, f32, f32)> = Vec::new();
        for P in 0..9 {
            n += 1;
            let pt = PolyPoint {
                x: total_segment_width / 2.0 + segment_default_size / 2.0 + k,
                y: 0.0,
                z: 0,
            };
            m.push((n as usize, vec![pt], segment_default_size, segment_default_size));
            k += segment_default_size;
            segment_widths.push(segment_default_size * scaling_factor);
        }
        out.extend(m);

        // JS: split widths for horizontal mode
        let segment_widths_f64: Vec<f64> = segment_widths.iter().map(|&x| x as f64).collect();
        let X = Self::split_into_segments_by_sum_limit(&segment_widths_f64, 800.0);
        let mut N = String::new();
        let mut H = String::new();
        for (start, count) in X.iter() {
            N += &Self::to_fixed_width_hex_b(*start as i32, 2);
            H += &Self::to_fixed_width_hex_b(*count as i32, 2);
        }
        let x_offset = -k * scaling_factor / 2.0;
        // Debug output for horizontal mode
        println!("[generate_segmented_layout_data] mode={} (horizontal)", mode);
        println!("  segment_widths: {:?}", segment_widths);
        println!("  N: {} (len {})", N, N.len());
        println!("  H: {} (len {})", H, H.len());
        println!("  x_offset: {}", x_offset);
        (out, N, H, x_offset)
    }

    /// Helper function to extract and clamp numeric values
    fn clamp_value<T: PartialOrd + Copy>(value: T, min: T, max: T, default: T) -> T {
        if value < min || value > max {
            default
        } else {
            value
        }
    }


pub fn split_into_segments_by_sum_limit(numbers: &[f64], limit: f64) -> Vec<(usize, usize)> {
    let mut r = 0.0;
    let mut n = Vec::new();
    let mut h = 0;
    let mut a = 0;
    let mut i = 0;
    while i < numbers.len() {
        if r + numbers[i] <= limit {
            a += 1;
            n.push((h, a));
            r += numbers[i];
        } else {
            let mut temp_width = r;
            loop {
                if temp_width <= limit {
                    a += 1;
                    n.push((h, a));
                    r = temp_width + numbers[i];
                    break;
                }
                if temp_width > limit && temp_width - numbers[h] < limit {
                    a += 1;
                    n.push((h, a));
                    r += numbers[i];
                    break;
                }
                temp_width -= numbers[h];
                r -= numbers[h];
                h += 1;
                a -= 1;
            }
        }
        i += 1;
    }
    n
}

    pub fn to_fixed_width_hex_b(val: i32, width: usize) -> String {
        let clamped = if width == 2 {
            val.max(0).min(255) as u32
        } else {
            val as u32
        };
        format!("{:0width$X}", clamped, width = width)
    }


}
