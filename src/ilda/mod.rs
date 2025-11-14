/// Format 4: 3D Coordinates with True Color (10 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub status_code: u8,
}

pub struct Frame {
    pub points: Vec<Point>,
}   