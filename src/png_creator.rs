use std::fs::{File, self};
use std::path::PathBuf;

use anyhow::Result;
use glam::{const_vec2, Vec2, Vec3};
use image::RgbaImage;
use itertools::Itertools;
use std::collections::HashMap;

use crate::assets::gfx::Gfx;
use crate::assets::{build_particle, build_atlas};
use crate::bdata::interactive_element_model_binary_data::InteractiveElementModelBinaryData;
use crate::bdata::teleporter_binary_data::Destination;
use crate::lib::custom_imageops;
use crate::lib::custom_imageops::color::BlendModes;
use crate::map::groups::Groups;
use crate::map::layer_manager::LayerManager;
use crate::map::{CELL_WIDTH, CELL_HEIGHT, Map};
use crate::map::binar_serial_part::BinarSerialPartsEnum;
use crate::map::color::Color;
use crate::map::element::ElementLibrary;
use crate::map::environment::EnvironmentChunk;
use crate::map::sprite::{MapSprite, AnmSprite, ParticleSprite, DynamicSprite, LayerOrder};
use crate::particles::particle_system::ParticleSystem;
use crate::tplg::Tplg;

const FLIP_Y: Vec2 = const_vec2!([1., -1.]);

fn convert_interactive_as_sprite(env: &EnvironmentChunk, iem: &HashMap<i32, InteractiveElementModelBinaryData>) -> Vec<MapSprite>{
	let mut interactive_sprites: Vec<MapSprite> = Vec::new();
	for chunk in env.get_chunks() {
		for interactive in chunk.get_interactive_elements() {
			for interactive_data in interactive.data.get_data() {
				for view in &interactive.views {
					if let BinarSerialPartsEnum::SpecificDataPart(specific_data_part) = interactive_data {
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

fn is_floor_visible_under(map: &Map, layers: &Vec<i32>, x: i32, y: i32, z: i16) -> bool {
	for chunk in map.chunks() {
		for sprite in chunk.sprites() {
			if sprite.cell_x == x && sprite.cell_y == y && sprite.cell_z == z {
				if sprite.group_key < 0 {
					if layers.contains(&sprite.group_key) {
						return true;
					}
					else {
						return false;
					}
				}
				return true;
			}
		}
	}
	true
}

fn get_teleporters_id(
	env: &EnvironmentChunk,
) -> Vec<i32> {
	let mut map_teleporters_id = Vec::new();
	for chunk in env.get_chunks() {
		for interactive in chunk.get_interactive_elements() {
			for interactive_data in interactive.data.get_data() {
				if let BinarSerialPartsEnum::SpecificDataPart(specific_data_part) = interactive_data {
					if interactive.interactive_type == 51 {
						if let Ok(id) = specific_data_part.parameter.parse::<i32>() {
							map_teleporters_id.push(id);
						}
					}
				}
			}
		}
	}
	map_teleporters_id
}

pub fn create_png(
	map_id: i32,
	map: Map,
	lib: &ElementLibrary,
	gfx: &mut Gfx,
	env: EnvironmentChunk,
	iem: &HashMap<i32, InteractiveElementModelBinaryData>,
	visible_layers: Option<Vec<i32>>,
	output_path: String
) -> Result<()> {
	println!("Processing {map_id}...");
	let mut sorted_sprite: HashMap<i64, &MapSprite> = HashMap::new();
	
	// Base map
	for chunk in map.chunks() {
		for sprite in chunk.sprites() {
			sorted_sprite.insert(sprite.hashcode(LayerOrder::Ground.get_index()), sprite);
		}
	}

	// // Interactive elements
	let interactive_sprites = convert_interactive_as_sprite(&env, &iem);
	for sprite in interactive_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(LayerOrder::InteractiveElement.get_index()), sprite);
	}

	// // Dynamic elements
	let dynamic_sprites = convert_dynamic_as_sprite(&env);
	for sprite in dynamic_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(LayerOrder::DynamicElement.get_index()), sprite);
	}

	// Particles elements
	let particles_sprites = convert_particles_as_sprite(&env);
	for sprite in particles_sprites.iter() {
		sorted_sprite.insert(sprite.hashcode(LayerOrder::Particle.get_index()), sprite);
	}

	let (margin, image_size) = get_margin_and_png_size(&lib, &sorted_sprite);

	let mut image = RgbaImage::new(
		image_size.x as u32,
		image_size.y as u32
	);

	// We display the other layers and interactive elements/dynamic/particles
	for hashcode in sorted_sprite.keys().sorted() {
		let sprite = sorted_sprite[hashcode];

		// Handle interactive animated sprite
		if let Some(anm_sprite) = &sprite.anm_sprite {
			if let Some(layers) = &visible_layers {
				if !is_floor_visible_under(&map, layers, sprite.cell_x, sprite.cell_y, sprite.cell_z) {
					continue ;
				}
			}

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
			if let Some(layers) = &visible_layers {
				if !is_floor_visible_under(&map, layers, sprite.cell_x, sprite.cell_y, sprite.cell_z) {
					continue ;
				}
			}
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
			if let Some(layers) = &visible_layers {
				if !is_floor_visible_under(&map, layers, sprite.cell_x, sprite.cell_y, sprite.cell_z) {
					continue ;
				}
			}
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

			if let Some(layers) = &visible_layers {
				if sprite.group_key < 0 && !layers.contains(&sprite.group_key) {
					continue ;
				}
			}
			else if sprite.group_key > 0 {
				continue ;
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

	println!("Saving into {}/{}.png...", output_path, map_id);
	if let Err(err) = image.save_with_format(format!("{}/{}.png", output_path, map_id), image::ImageFormat::Png) {
		println!("Failed to save image: {}", err);
	}
	Ok(())
}

pub fn recursive_create_png(
	processed_map: &mut Vec<i32>,
	worlds_id: Vec<i32>,
	teleporters_data: &HashMap<i32, Vec<Destination>>,
	maps_path: &PathBuf,
	lib: &ElementLibrary,
	gfx: &mut Gfx,
	iem_data: &HashMap<i32, InteractiveElementModelBinaryData>,
	groups: &Option<Groups>,
	tplg: &Option<Tplg>,
	output_path: PathBuf
) -> Result<()> {
	for world in worlds_id {
		if processed_map.contains(&world) {
			continue ;
		}
		let map_path = maps_path.join("gfx").join(format!("{}.jar", world));
		let env_path = maps_path.join("env").join(format!("{}.jar", world));
		let map = Map::load(File::open(map_path)?)?;
		let env = EnvironmentChunk::load(File::open(env_path)?)?;
		let map_teleporters_id = get_teleporters_id(&env);
		
		let mut new_worlds_id = Vec::new();
		for id in map_teleporters_id {
			if let Some(dest) = teleporters_data.get(&id) {
				let ids: Vec<i32> = dest.iter().map(|exit| exit.world_id).collect();
				for i in ids {
					if !processed_map.contains(&i) && !new_worlds_id.contains(&i) {
						new_worlds_id.push(i);
					}
				}
			}
		}
		if !output_path.exists() {
			fs::create_dir(output_path.clone())?;
		}
		let str_output_path = output_path.clone().into_os_string().into_string().unwrap();
		let visible_layers =
			if groups.is_some() { Some(LayerManager::get_outdoor_visible_layers(&map, &groups.as_ref().unwrap(), &tplg.as_ref().unwrap())) }
			else { None };
		create_png(world, map, lib, gfx, env, iem_data, visible_layers, str_output_path)?;
		processed_map.push(world);
		if new_worlds_id.len() > 0 {
			println!("\nNext maps to process: {:?}\n", new_worlds_id);
			let new_output_path = output_path.join(world.to_string());
			recursive_create_png(
				processed_map,
				new_worlds_id,
				teleporters_data,
				maps_path,
				lib,
				gfx,
				iem_data,
				groups,
				tplg,
				new_output_path
			)?;
		}
	}
	Ok(())
}
