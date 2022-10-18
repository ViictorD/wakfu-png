use std::borrow::Cow;
use anyhow::{anyhow, Result};
use byte::ctx::Bytes;
use byte::{TryRead, BytesExt};

use crate::utils::math_helper::MathHelper;

pub struct Tga<'a> {
	header: TgaHeader,
	_palette: Option<Palette>,
	bytes: Cow<'a, [u8]>
}

impl<'a> Tga<'a> {
	pub fn bytes(&'a self) -> &'a [u8] {
		&self.bytes
	}

	pub fn width(&self) -> u16 {
		self.header.width()
	}

	pub fn height(&self) -> u16 {
		self.header.height()
	}
}

impl<'a> TryRead<'a> for Tga<'a> {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let header: TgaHeader = bytes.read(offset)?;
		let mut opt_palette = None;
		if header.color_map_length() != 0 {
			if header.bit_depth() != 4 && header.bit_depth() != 8 {
				let color_size = header.color_map_bit_depth() as i16;
				let palette_size = header.color_map_length() * color_size;
				*offset += palette_size as usize;
			}
			else {
				let mut palette = Palette::new();
				for _ in 0..header.color_map_length() {
					let b: i8 = bytes.read(offset)?;
					let g: i8 = bytes.read(offset)?;
					let r: i8 = bytes.read(offset)?;
					palette.add_color(Color::new(-1, r as i32, g as i32, b as i32));
				}
				opt_palette = Some(palette);
			}
		}

		let line_size = (header.width() as f32 * (header.bit_depth() as f32 / 8.)) as usize;
		let tga_bytes = bytes.read_with(offset, Bytes::Len(header.height() as usize * line_size))?;

		let result = Tga {
			header,
			_palette: opt_palette,
			bytes: Cow::Borrowed(tga_bytes)
		};

		Ok((result, *offset))
	}
}

pub struct TgaHeader {
	_id_length: i8,
	_color_map_type: i8,
	_image_type: i8,
	_color_map_start: i16,
	color_map_length: i16,
	color_map_bit_depth: i8,
	_start_x: i16,
	_start_y: i16,
	width: u16,
	height: u16,
	bit_depth: i8,
	_desc: i8
}

impl TgaHeader {
	pub fn color_map_length(&self) -> i16 {
		self.color_map_length
	}

	pub fn color_map_bit_depth(&self) -> i8 {
		self.color_map_bit_depth
	}

	pub fn width(&self) -> u16 {
		self.width
	}

	pub fn height(&self) -> u16 {
		self.height
	}

	pub fn bit_depth(&self) -> i8 {
		self.bit_depth
	}
}

impl<'a> TryRead<'a> for TgaHeader {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let _id_length: i8 = bytes.read(offset)?;
		let _color_map_type: i8 = bytes.read(offset)?;
		let _image_type: i8 = bytes.read(offset)?;
		let _color_map_start: i16 = bytes.read(offset)?;
		let color_map_length: i16 = bytes.read(offset)?;
		let color_map_bit_depth: i8 = bytes.read(offset)?;
		let _start_x: i16 = bytes.read(offset)?;
		let _start_y: i16 = bytes.read(offset)?;
		let width: u16 = bytes.read(offset)?;
		let height: u16 = bytes.read(offset)?;
		let bit_depth: i8 = bytes.read(offset)?;
		let _desc: i8 = bytes.read(offset)?;

		let result = TgaHeader {
			_id_length,
			_color_map_type,
			_image_type,
			_color_map_start,
			color_map_length,
			color_map_bit_depth,
			_start_x,
			_start_y,
			width,
			height,
			bit_depth,
			_desc
		};

		Ok((result, *offset))
	}
}

pub struct Palette {
	colors: Vec<Color>
}

impl Palette {
	pub fn new() -> Self {
		Palette {
			colors: Vec::new()
		}
	}

	pub fn add_color(&mut self, color: Color) {
		self.colors.push(color);
	}
}

pub struct Color {
	_abgr: i32
}

impl Color {
	pub fn new(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
		let _abgr = MathHelper::clamp(alpha, 0, 255) << 24
			| MathHelper::clamp(red, 0, 255)
			| MathHelper::clamp(green, 0, 255) << 8
			| MathHelper::clamp(blue, 0, 255) << 16;
		Color {
			_abgr
		}
	}
}

pub struct TgaLoader;

impl TgaLoader {
	pub fn load(bytes: &[u8]) -> Result<Tga> {
		let tga: Tga = bytes
			.read(&mut 0)
			.map_err(|err| anyhow!("Failed to read TGA: {:?}", err))?;
		Ok(tga)
	}
}