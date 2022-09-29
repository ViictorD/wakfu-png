use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use anyhow::{Result, anyhow};
use bytebuffer::{ByteBuffer, Endian};
use glam::Vec2;
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;
use image::{ImageBuffer, SubImage, GenericImage, RgbaImage, DynamicImage, imageops, Rgba};
use crate::TgamLoader;
use crate::anm::anm::InteractiveAnim;
use crate::anm::processing::anm_instance::{AnmInstance, self};
use crate::anm::sprite_definition::sprite_def;
use crate::assets::build_atlas::build_atlas;
use crate::map::element::{MapElement};
use crate::map::sprite::{AnmSprite, MapSprite, DynamicSprite};

pub struct Gfx {
	gfx_archive: zip::ZipArchive<BufReader<File>>,
	interactive_archive: zip::ZipArchive<BufReader<File>>,
	dynamic_archive: zip::ZipArchive<BufReader<File>>
}

impl Gfx {
	pub fn load(gfx: File, interactive: File, dynamic: File) -> Self {
		let gfx_archive =
			ZipArchive::new(BufReader::new(gfx)).unwrap();
		let interactive_archive =
			ZipArchive::new(BufReader::new(interactive)).unwrap();
		let dynamic_archive =
			ZipArchive::new(BufReader::new(dynamic)).unwrap();
		return Gfx {
			gfx_archive,
			interactive_archive,
			dynamic_archive
		}
	}

	pub fn load_texture_as_rgba_image(&mut self, element: &MapElement, sprite: &MapSprite) -> Result<RgbaImage> {
		let texture_id = element.texture_id;

		match self.gfx_archive.by_name(&format!("gfx/{texture_id}.tgam")) {
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
								let mut result = image.sub_image(
									0,
									0,
									element.img_width as u32,
									element.img_height as u32
								)
								.to_image();
								if element.flags.is_flip() {
									imageops::flip_horizontal_in_place(&mut result);
								}

								for Rgba([r, g, b, a]) in result.pixels_mut() {
									// Apply premultiply alpha
									if *a > 0 && *a < 255 {
										*r = (*r as f32 * (1. / *a as f32) * 255.) as u8;
										*g = (*g as f32 * (1. / *a as f32) * 255.) as u8;
										*b = (*b as f32 * (1. / *a as f32) * 255.) as u8;
									}
									// Apply color tint
									if *a > 0 {
										*r = (sprite.color.r() * *r as f32) as u8;
										*g = (sprite.color.g() * *g as f32) as u8;
										*b = (sprite.color.b() * *b as f32) as u8;
										*a = (sprite.color.a() * *a as f32) as u8;
									}
								}

								Ok(result)
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

	pub fn load_interactive_texture_as_rgba_image(&mut self, anm_sprite: &AnmSprite) -> Result<(RgbaImage, Vec2)> {
		let gfx_id = anm_sprite.gfx_id;
		let anm = match self.interactive_archive.by_name(&format!("{gfx_id}.anm")) {
			Ok(mut entry) => {
				Self::read_anm(&mut entry)?
			},
			Err(_) => return Err(anyhow!("ANM file not found: {}", gfx_id))
		};
		
		let mut animation_name = format!("{}_AnimStatique_{}", anm_sprite.direction, anm_sprite.state);

		self.get_anm_image(anm, &animation_name, true)
	}

	pub fn load_dynamic_texture_as_rgba_image(&mut self, dyn_sprite: &DynamicSprite) -> Result<(RgbaImage, Vec2)> {
		let gfx_id = dyn_sprite.gfx_id;
	
		let anm = self.read_dynamic_anm(format!("{gfx_id}.anm"))?;

		let mut animation_name = String::default();
		
		for anm_name in anm.index.animation_file_records_by_name.keys() {
			if anm_name.contains(&dyn_sprite.base_anm) {
				animation_name = format!("{}_{}", dyn_sprite.direction, dyn_sprite.base_anm);
			}
		}
		if animation_name.is_empty() {
			animation_name = String::from_str("1_AnimStatique")?;
			if anm.index.animation_file_records_by_name.get(&animation_name).is_none() {
				return Err(anyhow!("Dynamic animation not found: {gfx_id}"));
			}
		}

		self.get_anm_image(anm, &animation_name, false)
	}

	fn read_dynamic_anm(&mut self, anm_file: String) -> Result<InteractiveAnim> {
		match self.dynamic_archive.by_name(anm_file.as_str()) {
			Ok(mut entry) => {
				return Self::read_anm(&mut entry);
			},
			Err(_) => return Err(anyhow!("ANM file not found: {anm_file}",))
		};
	}

	fn read_anm(entry: &mut ZipFile) -> Result<InteractiveAnim> {
		let mut bytes = Vec::with_capacity(entry.size() as usize);
		entry.read_to_end(&mut bytes)?;
		let mut buffer = ByteBuffer::from_vec(bytes);
		buffer.set_endian(Endian::LittleEndian);
		let mut anm = InteractiveAnim::new();
		anm.read(buffer);
		Ok(anm)
	}

	fn get_anm_image(&mut self, anm: InteractiveAnim, animation_name: &String, is_interactive: bool) -> Result<(RgbaImage, Vec2)>{
		let mut anm_instance = AnmInstance::new(anm);
		let flipped_anim_name = anm_instance.get_flipped_anim_name(animation_name);
		if !animation_name.eq(&flipped_anim_name) {
			anm_instance.flip_animation = true;
		}
		let file_record = anm_instance.definition.index.get_animation_file_record(&flipped_anim_name);
		if file_record.file_index == -1 {
			anm_instance.max_sprite_count = anm_instance.definition.max_sprite_count;
			anm_instance.crc_animation = file_record.crc;

			let mut sprite_def = anm_instance.definition.get_sprite_definition_by_crc(anm_instance.crc_animation).unwrap().clone();
			anm_instance.process_frame(0, &mut sprite_def, &anm_instance.root.clone(), true, 0);
		}
		else {
			let anm_file = anm_instance.definition.index.file_names.get(file_record.file_index as usize).unwrap();
			let anm = self.read_dynamic_anm(anm_file.clone()).unwrap();

			anm_instance.max_sprite_count = anm.max_sprite_count;
			anm_instance.crc_animation = anm.index.get_animation_file_record(&flipped_anim_name).crc;
			
			let mut sprite_def = anm.get_sprite_definition_by_crc(anm_instance.crc_animation).unwrap().clone();
			anm_instance.animation = Some(anm);
			anm_instance.process_frame(0, &mut sprite_def, &anm_instance.root.clone(), false, 0);
		}
		
		if anm_instance.coords.is_empty() {
			return Err(anyhow!("Empty coords, can not create animated sprite"));
		}

		let atlas: RgbaImage;
		{
			let texture_name = &anm_instance.definition.texture_name;
			let atlas_entry_opt =
				if is_interactive { self.interactive_archive.by_name(&format!("Atlas/{texture_name}.png")) }
				else { self.dynamic_archive.by_name(&format!("Atlas/{texture_name}.png")) };
			atlas = Self::get_atlas(atlas_entry_opt, texture_name)?;
		}

		let mut atlas_2 = None;
		if anm_instance.animation.is_some() {
			let texture_name = &anm_instance.animation.as_ref().unwrap().texture_name;
			let atlas_entry_opt_2 =
				if is_interactive { Some(self.interactive_archive.by_name(&format!("Atlas/{}.png", texture_name))) }
				else { Some(self.dynamic_archive.by_name(&format!("Atlas/{}.png", texture_name))) };
			atlas_2 = Some(Self::get_atlas(atlas_entry_opt_2.unwrap(), texture_name)?);
		}

		let (mut anm_sprite, origin) = build_atlas(atlas, atlas_2, anm_instance)?;
		Ok((anm_sprite, origin))
	}

	fn get_atlas(opt: Result<ZipFile, ZipError>, texture_name: &String) -> Result<RgbaImage> {
		match opt {
			Ok(mut atlas_entry) => {
				let mut bytes = Vec::with_capacity(atlas_entry.size() as usize);
				atlas_entry.read_to_end(&mut bytes);

				let atlas = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;

				if let DynamicImage::ImageRgba8(atlas_png) = atlas {
					return Ok(atlas_png);
				}
				return Err(anyhow!("Unable to load atlas file: {}", texture_name))
			},
			Err(_) => Err(anyhow!("Atlas file not found: {}", texture_name))
		}
	}

}