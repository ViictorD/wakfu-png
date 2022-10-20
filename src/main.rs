use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use assets::gfx::Gfx;
use assets::tgam::TgamLoader;
use bdata::interactive_element_model_binary_data::InteractiveElementModelBinaryData;
// use bdata::teleporter_binary_data::TeleporterBinaryData;
use glam::{const_vec2, Vec2, Vec3};
use image::{RgbaImage};
use itertools::Itertools;
use map::color::Color;
use map::environment::EnvironmentChunk;
use map::{element::ElementLibrary};
use map::Map;
use map::sprite::{MapSprite, AnmSprite, DynamicSprite, ParticleSprite};
use pico_args::Arguments;
use std::collections::HashMap;
use map::{CELL_WIDTH, CELL_HEIGHT};

use crate::assets::{build_particle, build_atlas};
use crate::bdata::binary_document::BinaryDocument;
use crate::map::binar_serial_part::BinarSerialPartsEnum;
use crate::particles::particle_system::ParticleSystem;
use crate::lib::custom_imageops;
use crate::lib::custom_imageops::color::BlendModes;

mod assets;
mod map;
mod bdata;
mod utils;
mod anm;
mod particles;
mod lib;

const FLIP_Y: Vec2 = const_vec2!([1., -1.]);

fn convert_interactive_as_sprite(env: &EnvironmentChunk, iem: &HashMap<i32, InteractiveElementModelBinaryData>) -> Vec<MapSprite>{
	let mut interactive_sprites: Vec<MapSprite> = Vec::new();
	for chunk in env.get_chunks() {
		for interactive in chunk.get_interactive_elements() {
			for interactive_data in interactive.data.get_data() {
				if let BinarSerialPartsEnum::SpecificDataPart(specific_data_part) = interactive_data {
					for view in &interactive.views {
						let bin_data = iem.get(view).unwrap();
						let mut sprite = MapSprite {
							cell_x: specific_data_part.x,
							cell_y: specific_data_part.y,
							cell_z: specific_data_part.z,
							height: 0,
							altitude_order: 100,
							tag: 0,
							element_id: -1,
							group_key: 0,
							group_id: 0,
							layer: 0,
							color: Color::rgb_linear(1.0, 1.0, 1.0),
							anm_sprite: None,
							dyn_sprite: None,
							particle_sprite: None
						};
						if bin_data.particle_id == 0 {
							sprite.anm_sprite = Some(AnmSprite::new(
								bin_data.gfx,
								specific_data_part.direction,
								specific_data_part.activation_pattern,
								specific_data_part.state
							));
						}
						else {
							sprite.altitude_order = 150;
							sprite.particle_sprite = Some(ParticleSprite::new(
								bin_data.particle_id,
								1,
								0,
								0,
								bin_data.particle_offset_z as i8
							));
						}
						interactive_sprites.push(sprite);
					}
				}
			}
		}
	}
	interactive_sprites
}

fn convert_dynamic_as_sprite(env: &EnvironmentChunk) -> Vec<MapSprite> {
	let mut dynamic_sprites: Vec<MapSprite> = Vec::new();
	for chunk in env.get_chunks() {
		for dynamic_element in chunk.get_dynamic_elements() {
			if dynamic_element.dynamic_type == 1 {
				let sprite = MapSprite {
					cell_x: dynamic_element.coord.x as i32 + chunk.x as i32 * 18,
					cell_y: dynamic_element.coord.y as i32 + chunk.y as i32 * 18,
					cell_z: dynamic_element.coord.z as i16,
					height: 0,
					altitude_order: 50,
					tag: 0,
					element_id: -1,
					group_key: 0,
					group_id: 0,
					layer: 0,
					color: Color::rgb_linear(1.0, 1.0, 1.0),
					anm_sprite: None,
					dyn_sprite: Some(DynamicSprite::new(
						dynamic_element.gfx_id,
						dynamic_element.direction,
						dynamic_element.base_animation.clone()
					)),
					particle_sprite: None
				};
				dynamic_sprites.push(sprite);
			}
		}
	}
	dynamic_sprites
}

fn convert_particles_as_sprite(env: &EnvironmentChunk) -> Vec<MapSprite> {
	let mut particles_sprites = Vec::new();
	for chunk in env.get_chunks() {
		for particle in &chunk.particle_data {
			let sprite = MapSprite {
				cell_x: particle.coord.x as i32 + chunk.x as i32 * 18,
				cell_y: particle.coord.y as i32 + chunk.y as i32 * 18,
				cell_z: particle.coord.z,
				height: 0,
				altitude_order: 150,
				tag: 0,
				element_id: -1,
				group_key: 0,
				group_id: 0,
				layer: 0,
				color: Color::rgb_linear(1.0, 1.0, 1.0),
				anm_sprite: None,
				dyn_sprite: None,
				particle_sprite: Some(ParticleSprite::new(
					particle.system_id,
					particle.level,
					particle.offset_x,
					particle.offset_y,
					particle.offset_z
				))
			};
			particles_sprites.push(sprite);
		}
	}
	particles_sprites
}

fn get_margin_and_png_size(lib: &ElementLibrary, sorted_sprite: &HashMap<i64, &MapSprite>) -> (Vec2, Vec2) {
	let mut min = Vec2::new(f32::MAX, f32::MAX);
	let mut max = Vec2::new(f32::MIN, f32::MIN);

	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];
		if sprite.element_id == -1 {
			continue ;
		}
		match lib.get(sprite.element_id) {
			Some(element) => {
				let vec2 = sprite.screen_position() * FLIP_Y - element.origin();
		
				if vec2.x < min.x {
					min.x = vec2.x;
				}
				if vec2.y < min.y {
					min.y = vec2.y;
				}
				if vec2.x > max.x {
					max.x = vec2.x;
				}
				if vec2.y > max.y {
					max.y = vec2.y;
				}
			},
			None => println!("Element not found id: {}", sprite.element_id)
		}
	}

	// Calculate the amount to add to remove negative coords
	let margin = Vec2::new(
		if min.x < 0. { min.x } else { 0. },
		if min.y < 0. { min.y } else { 0. }
	);

	(
		margin,
		Vec2::new(
			max.x - margin.x + CELL_WIDTH,
			max.y - margin.y + CELL_HEIGHT
		)
	)

}

fn create_png(
	map_id: i32,
	map: Map,
	lib: ElementLibrary,
	mut gfx: Gfx,
	env: EnvironmentChunk,
	iem: HashMap<i32, InteractiveElementModelBinaryData>
) {
	let mut sorted_sprite: HashMap<i64, &MapSprite> = HashMap::new();
	
	// Base map
	for chunk in map.chunks() {
		for sprite in chunk.sprites() {
			sorted_sprite.insert(sprite.hashcode(), sprite);
		}
	}

	// // Interactive elements
	let interactive_sprites = convert_interactive_as_sprite(&env, &iem);
	for sprite in interactive_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(), sprite);
	}

	// // Dynamic elements
	let dynamic_sprites = convert_dynamic_as_sprite(&env);
	for sprite in dynamic_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(), sprite);
	}

	// Particles elements
	let particles_sprites = convert_particles_as_sprite(&env);
	for sprite in particles_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(), sprite);
	}

	let (margin, image_size) = get_margin_and_png_size(&lib, &sorted_sprite);

	let mut image = RgbaImage::new(
		image_size.x as u32,
		image_size.y as u32
	);


	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];

		// Handle interactive animated sprite
		if let Some(anm_sprite) = &sprite.anm_sprite {
			let res = gfx.load_interactive_texture_as_rgba_image(anm_sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let (atlas, atlas_2, anm_instance) = res.unwrap();

			build_atlas::build_atlas(
				&mut image,
				sprite.screen_position() * FLIP_Y - margin,
				atlas,
				atlas_2,
				anm_instance
			);

			continue;
		}

		// Handle dynamic animated sprite
		if let Some(dyn_sprite) = &sprite.dyn_sprite {
			let res = gfx.load_dynamic_texture_as_rgba_image(dyn_sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let (atlas, atlas_2, anm_instance) = res.unwrap();

			build_atlas::build_atlas(
				&mut image,
				sprite.screen_position() * FLIP_Y - margin,
				atlas,
				atlas_2,
				anm_instance
			);

			continue;
		}

		// Handle particle
		if let Some(particle) = &sprite.particle_sprite {
			let res = gfx.load_particle_system_and_tga(
				particle,
				Vec3::new(sprite.cell_x as f32, sprite.cell_y as f32, sprite.cell_z as f32)
			);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}

			let (loaded_particles, tga) = res.unwrap();
			let mut particle_system = ParticleSystem::new();
			particle_system.load(loaded_particles);
			particle_system.register_all_base_emitters();
		
			// We simulate 5s of rendering, to get a well rendered result
			const TIME_INCREMENT: f32 = 0.0066;
			let mut life = 0.;
			while life < 5. {
				life += TIME_INCREMENT;
				particle_system.update(TIME_INCREMENT);
			}

			let (particles_coords, particles_colors) = particle_system.get_particles_coords_and_colors();
			let iso_offsets = ParticleSystem::get_screen_position(particle.offset_x as f32 / 100., particle.offset_y as f32 / 100., particle.offset_z as f32 / 10.);
			build_particle::build_particle(
				&mut image,
				sprite.screen_position() * FLIP_Y - margin + Vec2::new(iso_offsets.0, iso_offsets.1) * FLIP_Y,
				tga,
				particles_coords,
				particles_colors,
				(particle_system.src_blend, particle_system.dst_blend)
			);

			continue;
		}

		// Handle base map sprite
		if let Some(element) = lib.get(sprite.element_id) {
			// Skip the debug animated cell
			if element.texture_id == 19067 {
				continue;
			}
			
			let res = gfx.load_texture_as_rgba_image(&element, &sprite);
			if let Err(err) = res {
				println!("{}", err);
				continue;
			}
			let vec2 = sprite.screen_position() * FLIP_Y - element.origin();
			custom_imageops::custom_overlay(
				&mut image,
				&res.unwrap(),
				(vec2.x - margin.x) as i64,
				(vec2.y - margin.y) as i64,
				&BlendModes::One,
				&BlendModes::InvSrcAlpha
			);
		}
	}

	println!("Saving into {}.png...", map_id);
	if let Err(err) = image.save_with_format(format!("./{}.png", map_id), image::ImageFormat::Png) {
		println!("Failed to save image: {}", err);
	}
}

fn main() -> Result<()> {
	let mut pargs = Arguments::from_env();

	let game_path: PathBuf = pargs.value_from_str("--path")?;
	let map_id: i32 = pargs.value_from_str("--map")?;
	
	let contents_path = game_path.join("contents");
	let maps_path = contents_path.join("maps");
	
	let gfx_path = maps_path.join("gfx.jar");
	let animation_path = contents_path.join("animations");
	let interactive_path = animation_path
		.join("interactives")
		.join("interactives.jar");
	let dynamic_path = animation_path
		.join("dynamics")
		.join("dynamics.jar");
	let particles_path = contents_path
		.join("particles")
		.join("particles.jar");
	let map_path = maps_path.join("gfx").join(format!("{}.jar", map_id));
	let lib_path = maps_path.join("data.jar");
	let env_path = maps_path.join("env").join(format!("{}.jar", map_id));
	let iem_path = contents_path.join("bdata").join("34.jar");
	// let teleporter_path = contents_path.join("bdata").join("72.jar");
	
	let map = Map::load(File::open(map_path)?)?;
	let lib = ElementLibrary::load(File::open(lib_path)?)?;
	let gfx = Gfx::load(
		File::open(gfx_path)?,
		File::open(interactive_path)?,
		File::open(dynamic_path)?,
		File::open(particles_path)?
	);
	let env = EnvironmentChunk::load(File::open(env_path)?)?;
	let mut iem = BinaryDocument::load(File::open(iem_path)?, 34)?;
	let iem_data = InteractiveElementModelBinaryData::read(&mut iem);

	// let mut teleporter = BinaryDocument::load(File::open(teleporter_path)?, 72)?;
	// let teleporter_data = TeleporterBinaryData::read(&mut teleporter);

	create_png(map_id, map, lib, gfx, env, iem_data);

	Ok(())
}