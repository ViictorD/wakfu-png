use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;
use anyhow::{Result, anyhow};
use zip::ZipArchive;
use zip::result::ZipError;
use image::{ImageBuffer, SubImage, GenericImage, RgbaImage};
use crate::TgamLoader;
use crate::map::element::{MapElement};

pub struct Gfx {
	archive: zip::ZipArchive<BufReader<File>>
}

impl Gfx {
	pub fn load(input: File) -> Self {
		let archive =
			ZipArchive::new(BufReader::new(input)).unwrap();
		return Gfx {
			archive
		}
	}

	pub fn load_texture_as_rgba_image(&mut self, element: &MapElement, texture_id: i32) -> Result<RgbaImage> {
		match self.archive.by_name(&format!("gfx/{texture_id}.tgam")) {
			Ok(mut entry) => {
				let mut bytes = Vec::with_capacity(entry.size() as usize);
				entry.read_to_end(&mut bytes)?;
				match TgamLoader::load(&bytes) {
					Ok(tgam) => {
						let opt_image = RgbaImage::from_vec(
							tgam.width(),
							tgam.height(),
							tgam.bytes().to_vec()
						);
						match opt_image {
							Some(mut image) => {
								Ok(
									image.sub_image(
										0,
										0,
										element.img_width as u32,
										element.img_height as u32
									).to_image()
								)
							},
							None => Err(anyhow!("Failed to create image buffer for texture: {}", texture_id))
						}
					}
					Err(err) => Err(err)
				}
			},
			Err(_) => Err(anyhow!("TGAM file not found: {}", texture_id))
		}
	}
}