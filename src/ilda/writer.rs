//! Utility to write ILDA model to file bytes
//! Inspired by Volst/laser-dac and provided JS example
use crate::ilda::model::*;

/// Write an IldaFile to a Vec<u8> (byte array)
pub fn write_ilda_to_bytes(ilda: &IldaFile) -> Vec<u8> {
	let mut bytes = Vec::new();
	for (section_index, section) in ilda.sections.iter().enumerate() {
		match section {
			IldaSection::Frame { header, points } => {
				write_header(&mut bytes, header, section_index as u16, header.total_frames_or_0);
				match header.format_code {
					IldaFormatCode::Format0_3DIndexed => {
						for (i, point) in points.iter().enumerate() {
							if let IldaPoint::Format0 { x, y, z, status, color_index } = point {
								bytes.extend(&x.to_be_bytes());
								bytes.extend(&y.to_be_bytes());
								bytes.extend(&z.to_be_bytes());
								bytes.push(*status);
								bytes.push(*color_index);
							}
						}
					}
					IldaFormatCode::Format1_2DIndexed => {
						for (i, point) in points.iter().enumerate() {
							if let IldaPoint::Format1 { x, y, status, color_index } = point {
								bytes.extend(&x.to_be_bytes());
								bytes.extend(&y.to_be_bytes());
								bytes.push(*status);
								bytes.push(*color_index);
							}
						}
					}
					IldaFormatCode::Format4_3DTrueColor => {
						for (i, point) in points.iter().enumerate() {
							if let IldaPoint::Format4 { x, y, z, status, blue, green, red } = point {
								bytes.extend(&x.to_be_bytes());
								bytes.extend(&y.to_be_bytes());
								bytes.extend(&z.to_be_bytes());
								bytes.push(*status);
								bytes.push(*blue);
								bytes.push(*green);
								bytes.push(*red);
							}
						}
					}
					IldaFormatCode::Format5_2DTrueColor => {
						for (i, point) in points.iter().enumerate() {
							if let IldaPoint::Format5 { x, y, status, blue, green, red } = point {
								bytes.extend(&x.to_be_bytes());
								bytes.extend(&y.to_be_bytes());
								bytes.push(*status);
								bytes.push(*blue);
								bytes.push(*green);
								bytes.push(*red);
							}
						}
					}
					_ => {}
				}
			}
			IldaSection::Palette { header, colors } => {
				write_header(&mut bytes, header, section_index as u16, 0);
				for color in colors {
					bytes.push(color.red);
					bytes.push(color.green);
					bytes.push(color.blue);
				}
			}
		}
	}
	bytes
}

fn write_header(bytes: &mut Vec<u8>, header: &IldaHeader, section_index: u16, total: u16) {
	// 4 bytes: 'ILDA'
	bytes.extend(b"ILDA");
	// 3 bytes reserved (0)
	bytes.extend(&[0, 0, 0]);
	// 1 byte: format code
	bytes.push(header.format_code as u8);
	// 8 bytes: frame/palette name (null-padded)
	write_ilda_string(bytes, &header.frame_or_palette_name, 8);
	// 8 bytes: company name (null-padded)
	write_ilda_string(bytes, &header.company_name, 8);
	// 2 bytes: number of records
	bytes.extend(&(header.num_records).to_be_bytes());
	// 2 bytes: frame/palette number (section index)
	bytes.extend(&(section_index).to_be_bytes());
	// 2 bytes: total frames or 0
	bytes.extend(&(total).to_be_bytes());
	// 1 byte: projector number
	bytes.push(header.projector_number);
	// 1 byte: reserved (0)
	bytes.push(0);
}

fn write_ilda_string(bytes: &mut Vec<u8>, s: &str, len: usize) {
	let mut buf = [0u8; 8];
	let s_bytes = s.as_bytes();
	let n = s_bytes.len().min(len);
	buf[..n].copy_from_slice(&s_bytes[..n]);
	bytes.extend(&buf[..len]);
}
