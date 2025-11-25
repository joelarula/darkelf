
use darkelf::blue::model::DrawCommandData;
use serde::{Serialize, Deserialize};
use darkelf::blue::model::BeamColor;
use darkelf::blue::model::Point;


const REFERENCE_COORDINATE_SIZE: f64 = 800.0;


/// Drawing modes that determine how the object is rendered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Polylines = -1,     // Multiple polyline paths
    Shape = 2,          // Generic shape (your example uses this)
    Text = 9999,        // Text rendering
}



impl From<i32> for DrawMode {
    fn from(value: i32) -> Self {
        match value {
            -1 => DrawMode::Polylines,
            9999 => DrawMode::Text,
            _ => DrawMode::Shape, // Default for other values like 2
        }
    }
}

impl From<DrawMode> for i32 {
    fn from(mode: DrawMode) -> Self {
        mode as i32
    }
}

impl Serialize for DrawMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for DrawMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(DrawMode::from(value))
    }
}


/// Flexible point structure that can handle both simple points and nested polylines
#[derive(Debug, Clone)]
pub enum DrawPoints {
    /// Simple array of points for regular shapes/text
    Simple(Vec<Point>),
    /// Nested arrays for polylines mode
    Polylines(Vec<Vec<Point>>),
}

impl DrawPoints {
    /// Get all points as a flattened vector for processing
    pub fn flatten(&self) -> Vec<Point> {
        match self {
            DrawPoints::Simple(points) => points.clone(),
            DrawPoints::Polylines(polylines) => {
                polylines.iter().flat_map(|polyline| polyline.iter()).cloned().collect()
            }
        }
    }
    
    /// Get the total number of points
    pub fn len(&self) -> usize {
        match self {
            DrawPoints::Simple(points) => points.len(),
            DrawPoints::Polylines(polylines) => polylines.iter().map(|p| p.len()).sum(),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}




mod draw_points_serde {
    use super::{DrawPoints, Point, BeamColor};
    use serde::{Serializer, Deserializer, Deserialize, Serialize};
    use serde::ser::SerializeSeq;
    use serde_json::Value;

    pub fn serialize<S>(points: &DrawPoints, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match points {
            DrawPoints::Simple(points) => points.serialize(serializer),
            DrawPoints::Polylines(polylines) => polylines.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DrawPoints, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        if let Value::Array(arr) = value {
            if arr.is_empty() {
                return Ok(DrawPoints::Simple(vec![]));
            }

            // Check the first element to determine structure
            match &arr[0] {
                // If first element is an array, check if it's nested (polylines) or flat (simple points)
                Value::Array(inner_arr) => {
                    // Check if this is nested arrays (polylines) or flat point arrays (simple)
                    if !inner_arr.is_empty() && matches!(inner_arr[0], Value::Array(_)) {
                        // This is polylines - nested arrays like [[[x,y,c,p], [x,y,c,p]], [[x,y,c,p]]]
                        let mut polylines = Vec::new();
                        for polyline_value in arr {
                            if let Value::Array(polyline_arr) = polyline_value {
                                let mut polyline_points = Vec::new();
                                for point_value in polyline_arr {
                                    if let Value::Array(point_arr) = point_value {
                                        if point_arr.len() == 4 {
                                            let x = point_arr[0].as_f64().unwrap_or(0.0);
                                            let y = point_arr[1].as_f64().unwrap_or(0.0);
                                            let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                            let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                            polyline_points.push(Point { x, y, color: BeamColor::try_from(color).unwrap_or(BeamColor::Blank), pen_state });
                                        }
                                    }
                                }
                                polylines.push(polyline_points);
                            }
                        }
                        Ok(DrawPoints::Polylines(polylines))
                    } else {
                        // This is simple points - flat array like [[x,y,c,p], [x,y,c,p]]
                        let mut points = Vec::new();
                        for point_value in arr {
                            if let Value::Array(point_arr) = point_value {
                                if point_arr.len() == 4 {
                                    let x = point_arr[0].as_f64().unwrap_or(0.0);
                                    let y = point_arr[1].as_f64().unwrap_or(0.0);
                                    let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                    let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                    points.push(Point { x, y, color: BeamColor::try_from(color).unwrap_or(BeamColor::Blank), pen_state });
                                }
                            }
                        }
                        Ok(DrawPoints::Simple(points))
                    }
                }
                // If first element is a number, this is simple points format
                Value::Number(_) => {
                    // This is simple points - flat array of [x, y, color, pen_state] arrays
                    let mut points = Vec::new();
                    for point_value in arr {
                        if let Value::Array(point_arr) = point_value {
                            if point_arr.len() == 4 {
                                let x = point_arr[0].as_f64().unwrap_or(0.0);
                                let y = point_arr[1].as_f64().unwrap_or(0.0);
                                let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                points.push(Point { x, y, color: BeamColor::try_from(color).unwrap_or(BeamColor::Blank), pen_state });
                            }
                        }
                    }
                    Ok(DrawPoints::Simple(points))
                }
                _ => {
                    // Fallback to empty simple points
                    Ok(DrawPoints::Simple(vec![]))
                }
            }
        } else {
            Ok(DrawPoints::Simple(vec![]))
        }
    }
}

/// Represents a single drawable object with geometry and transformation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawItem {
    /// Drawing points - can be either simple points or nested polylines depending on draw_mode
    #[serde(serialize_with = "draw_points_serde::serialize", deserialize_with = "draw_points_serde::deserialize")]
    pub ps: DrawPoints,
    
    /// X-axis translation offset
    pub x0: f64,
    
    /// Y-axis translation offset  
    pub y0: f64,
    
    /// Scale factor (z-axis or zoom)
    pub z: f64,
    
    /// Drawing mode that determines rendering method
    #[serde(rename = "drawMode")]
    pub draw_mode: DrawMode,
    
    /// Rotation angle in degrees
    pub ang: f64,
    
    /// Line color value (0-15 for direct colors, >=8 for special color modes)
    #[serde(rename = "lineColor")]
    pub line_color: u8,
}
impl DrawItem {
    pub fn new() -> Self {
        Self {
            ps: DrawPoints::Simple(Vec::new()),
            x0: 0.0,
            y0: 0.0,
            z: 1.0,
            draw_mode: DrawMode::Shape,
            ang: 0.0,
            line_color: 1,
        }
    }
    
    /// Add a point to the drawing path
    pub fn add_point(&mut self, point: Point) {
        match &mut self.ps {
            DrawPoints::Simple(points) => points.push(point),
            DrawPoints::Polylines(_) => {
                // Convert to simple and add point
                let mut flattened = self.ps.flatten();
                flattened.push(point);
                self.ps = DrawPoints::Simple(flattened);
            }
        }
    }
    
 
    pub fn get_all_points(&self) -> Vec<Point> {
        self.ps.flatten()
    }
}

impl Default for DrawItem {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyDrawData {

    #[serde(rename = "drawPoints")]
    pub draw_points: Vec<DrawItem>,
     
    #[serde(rename = "pisObj")]
    pub pis_obj: DrawCommandData,
    
}


impl LegacyDrawData {
    pub fn new() -> Self {
        Self {
            draw_points: Vec::new(),
            pis_obj: DrawCommandData::default(),
        }
    }
    
    pub fn add_draw_object(&mut self, obj: DrawItem) {
        self.draw_points.push(obj);
    }
}

impl Default for LegacyDrawData {
    fn default() -> Self {
        Self::new()
    }
}


 pub fn prepare_draw_data(draw_data: &LegacyDrawData, width: f64) -> Vec<Point> {
        let mut points = Vec::new();

        for draw_object in &draw_data.draw_points {
            let object_points = match draw_object.draw_mode {
                DrawMode::Polylines => draw_all_transformed_polylines(draw_object, width),
                DrawMode::Text => draw_transformed_text(draw_object, width),
                _ => draw_transformed_object(draw_object, width),
            };

            // Concatenate results (points = points.concat(currentDrawResult))
            points.extend(object_points);
        }

        points
    }

    fn draw_transformed_object(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let rotated_points = rotate_points_around_bounding_box_center(
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

            let final_color = if rotated_point.color != BeamColor::Blank {
                if color_segment_index < 0 {
                    BeamColor::from_u8(current_color_index as u8).unwrap_or(BeamColor::Blank)
                } else if color_segment_index == 0 {
                    rotated_point.color // Keep original color
                } else {
                    BeamColor::from_u8(current_color_index as u8).unwrap_or(BeamColor::Blank)
                }
            } else {
                rotated_point.color
            };

            let result_x = rotated_point.x * scaling_factor * scale_z
                + (position_x - center_offset_x) * scaling_factor;
            let result_y = rotated_point.y * scale_z * scaling_factor
                + (-position_y + center_offset_x) * scaling_factor;
            let result_color = if point_index == 0 { BeamColor::Blank } else { final_color };
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

        if let DrawPoints::Polylines(polylines) = &draw_object.ps {
            for (index, _polyline) in polylines.iter().enumerate() {
                let current_polyline_result =
                    draw_transformed_polyline(draw_object, index, width);
                accumulated_results.extend(current_polyline_result);
            }
        }

        accumulated_results
    }

    fn draw_transformed_polyline(draw_object: &DrawItem, index: usize, width: f64) -> Vec<Point> {
        // Get the specific polyline at the given index
        if let DrawPoints::Polylines(polylines) = &draw_object.ps {
            if let Some(polyline) = polylines.get(index) {
                // Rotate points around bounding box center (passing true for polyline mode)
                let rotated_points = rotate_points_around_bounding_box_center_polyline(
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
        points: &[Point],
        angle_degrees: f64,
    ) -> Vec<Point> {
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
                let (rotated_x, rotated_y) = rotate_point_around_center(
                    angle_degrees,
                    center_x,
                    center_y,
                    point.x,
                    point.y,
                );
                Point {
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


    fn rotate_points_around_bounding_box_center(
        points: &[Point],
        angle: f64,
    ) -> Vec<Point> {
        if points.is_empty() {
            return Vec::new();
        }

        let mut rotated_points = Vec::new();
        let mut left = f64::MAX;
        let mut top = f64::MAX;
        let mut right = f64::MIN;
        let mut bottom = f64::MIN;

        for point in points {
            let x = point.x;
            let y = -point.y; 
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }

        let center_x = (right - left) / 2.0 + left; 
        let center_y = (bottom - top) / 2.0 + top; 


        for point in points {
            let x = point.x;
            let y = -point.y; 

            let rotated = rotate_point_around_center(angle, center_x, center_y, x, y);
            
            rotated_points.push(Point::new(
                rotated.0,  
                -rotated.1, 
                point.color,
                point.pen_state,
            ));
        }

        rotated_points
    }


    fn rotate_point_around_center(
        angle: f64,
        center_x: f64,
        center_y: f64,
        point_x: f64,
        point_y: f64,
    ) -> (f64, f64) {

        let a = point_x - center_x;
        let i = point_y - center_y;


        let c = center_x + (a * angle.cos() - i * angle.sin());
        let o = center_y + (a * angle.sin() + i * angle.cos());

        (c, o)
    }


