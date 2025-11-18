//! Builder for constructing ILDA models programmatically

use uuid::Uuid;

use super::model::{IldaFile, IldaSection, IldaHeader, IldaPoint, IldaPaletteColor, IldaFormatCode,ILDA_COORD_RANGE,status::BLANKED,status::NORMAL};

 const DARK_ELF: &str = "darkelf";
/// Builder for IldaFile
pub struct FileBuilder {
    sections: Vec<IldaSection>,
}

impl FileBuilder {
    pub fn new() -> Self {
        Self { sections: Vec::new() }
    }
    pub fn add_section(mut self, section: IldaSection) -> Self {
        self.sections.push(section);
        self
    }
    pub fn add_frame(mut self, header: IldaHeader, points: Vec<IldaPoint>) -> Self {
        self.sections.push(IldaSection::Frame { header, points });
        self
    }
    pub fn add_palette(mut self, header: IldaHeader, colors: Vec<IldaPaletteColor>) -> Self {
        self.sections.push(IldaSection::Palette { header, colors });
        self
    }
    pub fn build(self) -> IldaFile {
        IldaFile { sections: self.sections }
    }
}

/// Builder for IldaSection
pub struct SectionBuilder {
    header: Option<IldaHeader>,
    points: Option<Vec<IldaPoint>>,
    colors: Option<Vec<IldaPaletteColor>>,
    is_palette: bool,
    projection: Projection,
    x_scale_factor: f32,
    y_scale_factor: f32,
    point_axis_ratio: f32,
    point_spacing: f32,
}

/// Projection settings for a section (not yet used)
#[derive(Clone, Debug)]
pub struct Projection {
    pub width: i16,
    pub height: i16,
}

impl Default for Projection {
    fn default() -> Self {
        Projection {
            width: ILDA_COORD_RANGE as i16, 
            height: ILDA_COORD_RANGE as i16,
        }
    }
}

impl SectionBuilder {

    pub fn new_frame_with_projection(projection: Projection,point_axis_ratio: f32) -> Self {
        let default = Projection::default();
        let x_scale_factor = if projection.width != 0 {
            default.width as f32 / projection.width as f32
        } else {
            1.0
        };
        let y_scale_factor = if projection.height != 0 {
            default.height as f32 / projection.height as f32
        } else {
            1.0
        };
        Self {
            header: None,
            points: Some(Vec::new()),
            colors: None,
            is_palette: false,
            projection,
            x_scale_factor,
            y_scale_factor,
            point_axis_ratio: point_axis_ratio,
            point_spacing: Projection::default().width as f32 / point_axis_ratio,
        }
    }
    pub fn new_frame() -> Self {
        Self {
            header: None,
            points: Some(Vec::new()),
            colors: None,
            is_palette: false,
            projection: Projection::default(),
            x_scale_factor: 1.0,
            y_scale_factor: 1.0,
            point_axis_ratio: 20.0,
            point_spacing: Projection::default().width as f32 / 20.0,
        }
    }
    pub fn new_palette() -> Self {
        Self {
            header: None,
            points: None,
            colors: Some(Vec::new()),
            is_palette: true,
            projection: Projection::default(),
            x_scale_factor: 1.0,
            y_scale_factor: 1.0,
            point_axis_ratio: 20.0,
            point_spacing: Projection::default().width as f32 / 20.0,
        }
    }
    pub fn header(mut self, header: IldaHeader) -> Self {
        self.header = Some(header);
        self
    }
    pub fn add_point(mut self, point: IldaPoint) -> Self {
        if let Some(ref mut pts) = self.points {
            pts.push(point);
        }
        self
    }
    pub fn add_points(mut self, points: Vec<IldaPoint>) -> Self {
        if let Some(ref mut pts) = self.points {
            pts.extend(points);
        }
        self
    }

    pub fn move_to_point(mut self,x: i16,y: i16) -> Self {
        if let Some(ref mut pts) = self.points {
            pts.push(IldaPoint::Format4 { x, y, z: 0, status: BLANKED, blue: 0, green: 0, red: 0 });
        }
        self
    }
    pub fn line_to_point(mut self,x: i16,y: i16, color: IldaPaletteColor) -> Self {
        if let Some(ref mut pts) = self.points {
            pts.push(IldaPoint::Format4 { x, y, z: 0, status: NORMAL, blue: color.blue, green: color.green, red: color.red });
        }
        self
    }


    pub fn add_color(mut self, color: IldaPaletteColor) -> Self {
        if let Some(ref mut cols) = self.colors {
            cols.push(color);
        }
        self
    }
    pub fn add_colors(mut self, colors: Vec<IldaPaletteColor>) -> Self {
        if let Some(ref mut cols) = self.colors {
            cols.extend(colors);
        }
        self
    }
    pub fn build(self) -> IldaSection {
       
        let header = self.header.unwrap_or_else(|| {
            if self.is_palette {
                // Default palette header
                HeaderBuilder::new()
                    .format_code(IldaFormatCode::Format2_Palette)
                    .frame_or_palette_name(uuid::Uuid::new_v4().to_string())
                    .company_name(DARK_ELF)
                    .num_records(self.colors.as_ref().map(|c| c.len() as u16).unwrap_or(0))
                    .frame_or_palette_number(1)
                    .total_frames_or_0(1)
                    .projector_number(0)
                    .build()
            } else {
                // Default frame header
                HeaderBuilder::new()
                    .format_code(IldaFormatCode::Format0_3DIndexed)
                    .frame_or_palette_name(uuid::Uuid::new_v4().to_string())
                    .company_name(DARK_ELF)
                    .num_records(self.points.as_ref().map(|p| p.len() as u16).unwrap_or(0))
                    .frame_or_palette_number(0)
                    .total_frames_or_0(0)
                    .projector_number(0)
                    .build()
            }
        });
        if self.is_palette {
            IldaSection::Palette {
                header,
                colors: self.colors.unwrap_or_default(),
            }
        } else {
            IldaSection::Frame {
                header,
                points: self.points.unwrap_or_default(),
            }
        }
    }
}

/// Builder for IldaHeader
pub struct HeaderBuilder {
    format_code: Option<IldaFormatCode>,
    frame_or_palette_name: Option<String>,
    company_name: Option<String>,
    num_records: Option<u16>,
    frame_or_palette_number: Option<u16>,
    total_frames_or_0: Option<u16>,
    projector_number: Option<u8>,
}

impl HeaderBuilder {
    pub fn new() -> Self {
        Self {
            format_code: None,
            frame_or_palette_name: None,
            company_name: None,
            num_records: None,
            frame_or_palette_number: None,
            total_frames_or_0: None,
            projector_number: None,
        }
    }
    pub fn format_code(mut self, code: IldaFormatCode) -> Self {
        self.format_code = Some(code);
        self
    }
    pub fn frame_or_palette_name<S: Into<String>>(mut self, name: S) -> Self {
        self.frame_or_palette_name = Some(name.into());
        self
    }
    pub fn company_name<S: Into<String>>(mut self, name: S) -> Self {
        self.company_name = Some(name.into());
        self
    }
    pub fn num_records(mut self, n: u16) -> Self {
        self.num_records = Some(n);
        self
    }
    pub fn frame_or_palette_number(mut self, n: u16) -> Self {
        self.frame_or_palette_number = Some(n);
        self
    }
    pub fn total_frames_or_0(mut self, n: u16) -> Self {
        self.total_frames_or_0 = Some(n);
        self
    }
    pub fn projector_number(mut self, n: u8) -> Self {
        self.projector_number = Some(n);
        self
    }
    pub fn build(self) -> IldaHeader {
        IldaHeader {
            format_code: self.format_code.expect("format_code required"),
            frame_or_palette_name: self.frame_or_palette_name.unwrap_or_default(),
            company_name: self.company_name.unwrap_or_default(),
            num_records: self.num_records.unwrap_or(0),
            frame_or_palette_number: self.frame_or_palette_number.unwrap_or(0),
            total_frames_or_0: self.total_frames_or_0.unwrap_or(0),
            projector_number: self.projector_number.unwrap_or(0),
        }
    }
}

/// Builder for IldaPoint
pub struct IldaPointBuilder {
    kind: Option<IldaPointKind>,
}

enum IldaPointKind {
    Format0 { x: i16, y: i16, z: i16, status: u8, color_index: u8 },
    Format1 { x: i16, y: i16, status: u8, color_index: u8 },
    Format4 { x: i16, y: i16, z: i16, status: u8, blue: u8, green: u8, red: u8 },
    Format5 { x: i16, y: i16, status: u8, blue: u8, green: u8, red: u8 },
}

impl IldaPointBuilder {
    pub fn new_format0(x: i16, y: i16, z: i16, status: u8, color_index: u8) -> Self {
        Self { kind: Some(IldaPointKind::Format0 { x, y, z, status, color_index }) }
    }
    pub fn new_format1(x: i16, y: i16, status: u8, color_index: u8) -> Self {
        Self { kind: Some(IldaPointKind::Format1 { x, y, status, color_index }) }
    }
    pub fn new_format4(x: i16, y: i16, z: i16, status: u8, blue: u8, green: u8, red: u8) -> Self {
        Self { kind: Some(IldaPointKind::Format4 { x, y, z, status, blue, green, red }) }
    }
    /// Create a Format4 point using an IldaPaletteColor
    pub fn new_format4_with_color(x: i16, y: i16, z: i16, status: u8, color: IldaPaletteColor) -> Self {
        Self {
            kind: Some(IldaPointKind::Format4 {
                x,
                y,
                z,
                status,
                blue: color.blue,
                green: color.green,
                red: color.red,
            }),
        }
    }
    pub fn new_format5(x: i16, y: i16, status: u8, blue: u8, green: u8, red: u8) -> Self {
        Self { kind: Some(IldaPointKind::Format5 { x, y, status, blue, green, red }) }
    }
    /// Create a Format5 point using an IldaPaletteColor
    pub fn new_format5_with_color(x: i16, y: i16, status: u8, color: IldaPaletteColor) -> Self {
        Self {
            kind: Some(IldaPointKind::Format5 {
                x,
                y,
                status,
                blue: color.blue,
                green: color.green,
                red: color.red,
            }),
        }
    }
    pub fn build(self) -> IldaPoint {
        match self.kind.expect("Point kind required") {
            IldaPointKind::Format0 { x, y, z, status, color_index } => IldaPoint::Format0 { x, y, z, status, color_index },
            IldaPointKind::Format1 { x, y, status, color_index } => IldaPoint::Format1 { x, y, status, color_index },
            IldaPointKind::Format4 { x, y, z, status, blue, green, red } => IldaPoint::Format4 { x, y, z, status, blue, green, red },
            IldaPointKind::Format5 { x, y, status, blue, green, red } => IldaPoint::Format5 { x, y, status, blue, green, red },
        }
    }
}

/// Builder for IldaPaletteColor
pub struct IldaPaletteColorBuilder {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

impl IldaPaletteColorBuilder {
    pub fn new() -> Self {
        Self { red: None, green: None, blue: None }
    }
    pub fn red(mut self, r: u8) -> Self {
        self.red = Some(r);
        self
    }
    pub fn green(mut self, g: u8) -> Self {
        self.green = Some(g);
        self
    }
    pub fn blue(mut self, b: u8) -> Self {
        self.blue = Some(b);
        self
    }
    pub fn build(self) -> IldaPaletteColor {
        IldaPaletteColor {
            red: self.red.unwrap_or(0),
            green: self.green.unwrap_or(0),
            blue: self.blue.unwrap_or(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complete_builder_chain() {
        let header = HeaderBuilder::new()
            .format_code(IldaFormatCode::Format1_2DIndexed)
            .frame_or_palette_name("frame1")
            .company_name(DARK_ELF)
            .num_records(2)
            .frame_or_palette_number(0)
            .total_frames_or_0(1)
            .projector_number(0)
            .build();
        let point1 = IldaPointBuilder::new_format1(0, 0, 0, 1).build();
        let point2 = IldaPointBuilder::new_format1(100, 100, 0, 2).build();
        let section = SectionBuilder::new_frame()
            .header(header)
            .add_point(point1)
            .add_point(point2)
            .build();
        let file = FileBuilder::new()
            .add_section(section)
            .build();
        assert_eq!(file.sections.len(), 1);
    }
    #[test]
    fn test_palette_builder() {
        let header = HeaderBuilder::new()
            .format_code(IldaFormatCode::Format2_Palette)
            .frame_or_palette_name("palette1")
            .company_name("testco")
            .num_records(1)
            .frame_or_palette_number(0)
            .total_frames_or_0(1)
            .projector_number(0)
            .build();
        let color = IldaPaletteColorBuilder::new().red(10).green(20).blue(30).build();
        let section = SectionBuilder::new_palette()
            .header(header)
            .add_color(color)
            .build();
        let file = FileBuilder::new().add_section(section).build();
        assert_eq!(file.sections.len(), 1);
    }
}