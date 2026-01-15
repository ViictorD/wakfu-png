use std::borrow::Cow;

use anyhow::{anyhow, Result};
use byte::ctx::Bytes;
use byte::{BytesExt, TryRead};

use crate::utils::math_helper::MathHelper;

#[derive(Debug)]
pub struct Tgam<'a> {
	width: u16,
	height: u16,
	bytes: Cow<'a, [u8]>,
	mask: AlphaMask<'a>,
}

impl<'a> Tgam<'a> {
	#[inline]
	pub fn bytes(&'a self) -> &'a [u8] {
		&self.bytes
	}

	#[inline]
	pub fn width(&self) -> u32 {
		MathHelper::nearest_greatest_pow_of_two(self.width as i32) as u32
	}

	#[inline]
	pub fn height(&self) -> u32 {
		MathHelper::nearest_greatest_pow_of_two(self.height as i32) as u32
	}
}

impl<'a> TryRead<'a> for Tgam<'a> {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let resize_mask: u8 = bytes.read(offset)?;
		let header: &'a [u8] = bytes.read_with(offset, Bytes::Len(3))?;
		if header != b"AGT" {
			let err = "Invalid TGAM header";
			return Err(byte::Error::BadInput { err });
		}

		let width: u16 = bytes.read(offset)?;
		let height: u16 = bytes.read(offset)?;
		let tga_size: u32 = bytes.read(offset)?;
		let mask_size: u32 = bytes.read(offset)?;
		let mask_resize: u8 = if resize_mask == 109 {
			bytes.read(offset)?
		} else {
			1
		};
		let tga_bytes: &[u8] = bytes.read_with(offset, Bytes::Len(tga_size as usize))?;
		let mask_bytes: &[u8] = bytes.read_with(offset, Bytes::Len(mask_size as usize))?;

		let mask = AlphaMask {
			bytes: Cow::Borrowed(mask_bytes),
			resize: mask_resize,
		};

		let tgam = Tgam {
			width,
			height,
			bytes: Cow::Borrowed(tga_bytes),
			mask,
		};
		Ok((tgam, *offset))
	}
}

#[derive(Debug)]
pub struct AlphaMask<'a> {
	bytes: Cow<'a, [u8]>,
	resize: u8,
}

#[derive(Default)]
pub struct TgamLoader;

impl TgamLoader {
	pub fn load(bytes: &[u8]) -> Result<Tgam<'_>> {
		let tgam: Tgam = bytes
			.read(&mut 0)
			.map_err(|err| anyhow!("Failed to read TGAM: {:?}", err))?;
		Ok(tgam)
	}
}
