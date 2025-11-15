//! Utility to read ILDA files into the Rust model
//! Inspired by https://github.com/Volst/laser-dac/blob/master/packages/ilda-reader/src/index.ts
use std::io::{self, Read, Seek, SeekFrom};
use crate::ilda::model::*;

/// Reads an ILDA file from a byte slice and returns an IldaFile model
pub fn read_ilda_from_bytes(bytes: &[u8]) -> io::Result<IldaFile> {
	let mut offset = 0;
	let mut sections = Vec::new();
	while offset + 32 <= bytes.len() {
		// Check header
		if &bytes[offset..offset+4] != b"ILDA" {
			break;
		}
		// Parse header fields
		let format_code = IldaFormatCode::from_u8(bytes[offset+7])
			.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown ILDA format code"))?;
		let frame_or_palette_name = read_ilda_string(&bytes[offset+8..offset+16]);
		let company_name = read_ilda_string(&bytes[offset+16..offset+24]);
		let num_records = u16::from_be_bytes([bytes[offset+24], bytes[offset+25]]);
		let frame_or_palette_number = u16::from_be_bytes([bytes[offset+26], bytes[offset+27]]);
		let total_frames_or_0 = u16::from_be_bytes([bytes[offset+28], bytes[offset+29]]);
		let projector_number = bytes[offset+30];
		let header = IldaHeader {
			format_code,
			frame_or_palette_name,
			company_name,
			num_records,
			frame_or_palette_number,
			total_frames_or_0,
			projector_number,
		};
		offset += 32;
		// End of file marker
		if num_records == 0 {
			break;
		}
		match format_code {
			IldaFormatCode::Format0_3DIndexed => {
				let mut points = Vec::with_capacity(num_records as usize);
				for _ in 0..num_records {
					if offset + 8 > bytes.len() { break; }
					let x = i16::from_be_bytes([bytes[offset], bytes[offset+1]]);
					let y = i16::from_be_bytes([bytes[offset+2], bytes[offset+3]]);
					let z = i16::from_be_bytes([bytes[offset+4], bytes[offset+5]]);
					let status = bytes[offset+6];
					let color_index = bytes[offset+7];
					points.push(IldaPoint::Format0 { x, y, z, status, color_index });
					offset += 8;
				}
				sections.push(IldaSection::Frame { header, points });
			}
			IldaFormatCode::Format1_2DIndexed => {
				let mut points = Vec::with_capacity(num_records as usize);
				for _ in 0..num_records {
					if offset + 6 > bytes.len() { break; }
					let x = i16::from_be_bytes([bytes[offset], bytes[offset+1]]);
					let y = i16::from_be_bytes([bytes[offset+2], bytes[offset+3]]);
					let status = bytes[offset+4];
					let color_index = bytes[offset+5];
					points.push(IldaPoint::Format1 { x, y, status, color_index });
					offset += 6;
				}
				sections.push(IldaSection::Frame { header, points });
			}
			IldaFormatCode::Format2_Palette => {
				let mut colors = Vec::with_capacity(num_records as usize);
				for _ in 0..num_records {
					if offset + 3 > bytes.len() { break; }
					let red = bytes[offset];
					let green = bytes[offset+1];
					let blue = bytes[offset+2];
					colors.push(IldaPaletteColor { red, green, blue });
					offset += 3;
				}
				sections.push(IldaSection::Palette { header, colors });
			}
			IldaFormatCode::Format4_3DTrueColor => {
				let mut points = Vec::with_capacity(num_records as usize);
				for _ in 0..num_records {
					if offset + 10 > bytes.len() { break; }
					let x = i16::from_be_bytes([bytes[offset], bytes[offset+1]]);
					let y = i16::from_be_bytes([bytes[offset+2], bytes[offset+3]]);
					let z = i16::from_be_bytes([bytes[offset+4], bytes[offset+5]]);
					let status = bytes[offset+6];
					let blue = bytes[offset+7];
					let green = bytes[offset+8];
					let red = bytes[offset+9];
					points.push(IldaPoint::Format4 { x, y, z, status, blue, green, red });
					offset += 10;
				}
				sections.push(IldaSection::Frame { header, points });
			}
			IldaFormatCode::Format5_2DTrueColor => {
				let mut points = Vec::with_capacity(num_records as usize);
				for _ in 0..num_records {
					if offset + 8 > bytes.len() { break; }
					let x = i16::from_be_bytes([bytes[offset], bytes[offset+1]]);
					let y = i16::from_be_bytes([bytes[offset+2], bytes[offset+3]]);
					let status = bytes[offset+4];
					let blue = bytes[offset+5];
					let green = bytes[offset+6];
					let red = bytes[offset+7];
					points.push(IldaPoint::Format5 { x, y, status, blue, green, red });
					offset += 8;
				}
				sections.push(IldaSection::Frame { header, points });
			}
		}
	}
	Ok(IldaFile { sections })
}

fn read_ilda_string(slice: &[u8]) -> String {
	let nul = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
	String::from_utf8_lossy(&slice[..nul]).trim_end().to_string()
}
