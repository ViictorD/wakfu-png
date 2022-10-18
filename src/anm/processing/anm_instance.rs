use crate::{
	anm::{anm::{InteractiveAnim, AnmImport},
	sprite_definition::sprite_definition::SpriteDefinition
}};

use super::anm_transform::AnmTransform;

#[derive(Clone)]
pub struct SpriteCoord {
	pub is_root_atlas: bool,
	pub atlas: [f32; 4],
	pub result: [f32; 8]
}

impl Default for SpriteCoord {
	fn default() -> Self {
		SpriteCoord {
			is_root_atlas: true,
			atlas: [0.; 4],
			result: [0.; 8]
		}
	}
}

impl SpriteCoord {
	pub fn new(is_root_atlas: bool, atlas: [f32; 4], result: [f32; 8]) -> Self {
		SpriteCoord {
			is_root_atlas,
			atlas,
			result
		}
	}
}

pub struct AnmInstance {
	pub definition: InteractiveAnim,
	pub animation: Option<InteractiveAnim>,
	pub root: AnmTransform,
	pub max_sprite_count: i32,
	pub flip_animation: bool,
	pub crc_animation: i32,
	pub transforms: Vec<AnmTransform>,
	pub min_x: f32,
	pub min_y: f32,
	pub max_x: f32,
	pub max_y: f32,
	pub coords: Vec<SpriteCoord>,
	pub colors: Vec<[f32; 4]>
}

impl AnmInstance {
	pub fn new(anm: InteractiveAnim) -> Self {
		let mut transforms = Vec::with_capacity(32);
		for _ in 0..32 {
			transforms.push(AnmTransform::new());
		}
		AnmInstance {
			definition: anm,
			animation: None,
			root: AnmTransform::new(),
			max_sprite_count: 0,
			flip_animation: false,
			crc_animation: 0,
			transforms,
			min_x: f32::MAX,
			min_y: f32::MAX,
			max_x: f32::MIN,
			max_y: f32::MIN,
			coords: Vec::new(),
			colors: Vec::new()
		}
	}

	pub fn get_flipped_anim_name(&self, anim_name: &String) -> String {
		if self.definition.index.use_flip() {
			let direction = anim_name.chars().next().unwrap();
			let anim_name_sliced = &anim_name[1..];
			match direction {
				'4' => return format!("0{anim_name_sliced}"),
				'3' => return format!("1{anim_name_sliced}"),
				'7' => return format!("5{anim_name_sliced}"),
				_ => {}
			}
		}
		anim_name.clone()
	}

	pub fn process_frame(&mut self, frame_index: i32, sprite_def: &mut SpriteDefinition, parent_transform: &AnmTransform, is_root_anm: bool, level: i32) {
		let index = self.get_real_frame_index(frame_index, sprite_def);
		let sprite_count = sprite_def.sprite_def.begin_process_frame(index);
		let has_import = 
			if is_root_anm { !self.definition.imports_by_id.is_empty() }
			else { !self.animation.as_ref().unwrap().imports_by_id.is_empty() };
		let use_perfect_hit = 
			if is_root_anm { self.definition.use_perfect_hit_test }
			else { self.animation.as_ref().unwrap().use_perfect_hit_test };

		for _ in 0..sprite_count {
			sprite_def.sprite_def.next_sprite();
			let mut transform = self.transforms.get(level as usize).unwrap().clone();
			transform.custom_color_index = parent_transform.custom_color_index;
			let sprite_id = sprite_def.sprite_def.process(parent_transform, &mut transform);
			if transform.alpha.ne(&0f32) || !use_perfect_hit {
				if has_import {
					if let Some(anm_import) = 
						if is_root_anm { self.definition.get_import(sprite_id) }
						else { self.animation.as_ref().unwrap().get_import(sprite_id) }
					{
						self.attach_imported(frame_index, level, &mut transform, &anm_import.clone());
						continue ;
					}
				}
				let sprite_definition = 
					if is_root_anm { self.definition.sprite_definitions_by_id.get(&sprite_id) }
					else { self.animation.as_ref().unwrap().sprite_definitions_by_id.get(&sprite_id) };
				if sprite_definition.is_some() {
					self.process_sprite(&mut sprite_definition.unwrap().clone(), frame_index, &mut transform, is_root_anm, level);
				}
				else {
					let shape_definition_opt = 
						if is_root_anm { self.definition.shape_definitions_by_id.get(&sprite_id) }
						else { self.animation.as_ref().unwrap().shape_definitions_by_id.get(&sprite_id) };
					if let Some(shape_definition) = shape_definition_opt {
						if transform.alpha.gt(&0.004) || !use_perfect_hit {
							let tx = shape_definition.offset_x * transform.rotation_skew_x0 + shape_definition.offset_y * transform.rotation_skew_x1 + transform.translation_x;
							let ty = shape_definition.offset_x * -transform.rotation_skew_y0 + shape_definition.offset_y * -transform.rotation_skew_y1 - transform.translation_y;
							let hx = transform.rotation_skew_x1 * shape_definition.height as f32;
							let hy = -transform.rotation_skew_y1 * shape_definition.height as f32;
							let wx = transform.rotation_skew_x0 * shape_definition.width as f32;
							let wy = -transform.rotation_skew_y0 * shape_definition.width as f32;
		
							let x = hx + tx;
							let y = hy + ty;
							let x2 = wx + hx + tx;
							let y2 = wy + hy + ty;
							let x3 = wx + tx;
							let y3 = wy + ty;
		
							if tx > self.max_x {
								self.max_x = tx;
							}
							else if x < self.min_x {
								self.min_x = tx;
							}
							if x2 > self.max_x {
								self.max_x = x2;
							}
							else if x2 < self.min_x {
								self.min_x = x2;
							}
							if x2 > self.max_x {
								self.max_x = x2;
							}
							else if x2 < self.min_x {
								self.min_x = x2;
							}
							if x3 > self.max_x {
								self.max_x = x3;
							}
							else if x3 < self.min_x {
								self.min_x = x3;
							}
							if ty > self.max_y {
								self.max_y = ty;
							}
							else if ty < self.min_y {
								self.min_y = ty;
							}
							if y2 > self.max_y {
								self.max_y = y2;
							}
							else if y2 < self.min_y {
								self.min_y = y2;
							}
							if y2 > self.max_y {
								self.max_y = y2;
							}
							else if y2 < self.min_y {
								self.min_y = y2;
							}
							if y3 > self.max_y {
								self.max_y = y3;
							}
							else if y3 < self.min_y {
								self.min_y = y3;
							}
							self.coords.push(SpriteCoord {
								is_root_atlas: is_root_anm,
								atlas: [shape_definition.top, shape_definition.left, shape_definition.bottom, shape_definition.right],
								result: [tx, ty, x, y, x2, y2, x3, y3]
							});
							self.colors.push([transform.red, transform.green, transform.blue, transform.alpha]);
						}
					}
				}
				self.transforms[level as usize] = transform;
			}
		}
	}

	fn process_sprite(&mut self, sprite_definition: &mut SpriteDefinition, frame_index: i32, transform: &mut AnmTransform, is_root_anm: bool, level: i32) {
		let custom_color_index: i32 = sprite_definition.sprite_def.get_sprite_def().get_color_index();
		if custom_color_index != 0 {
			panic!("Not implmented");
		}
		if !sprite_definition.sprite_def.get_sprite_def().name.is_empty() {
			match sprite_definition.sprite_def.get_sprite_def().base_name_crc {
				1272524161 => {
					panic!("Not implmented");
				}
				1003439990 => {
					panic!("Not implmented");
				}
				_ => {}
			}
		}
		self.process_frame(frame_index, sprite_definition, transform, is_root_anm, level + 1);
	}

	fn get_real_frame_index(&self, frame_index: i32, animation: &SpriteDefinition) -> i32 {
		let frame_count = animation.sprite_def.get_frame_count();
		if frame_index < frame_count {
			return frame_index;
		}
		if animation.sprite_def.get_sprite_def().is_loop() {
			return frame_index % frame_count;
		}
		return frame_count - 1;
	}

	fn attach_imported(&mut self, frame_index: i32, level: i32, transform: &mut AnmTransform, anm_import: &AnmImport) {
		let sprite_definition_2 = self.definition.get_sprite_definition_by_crc(anm_import.crc);
		if self.sprite_definition_is_visible(&sprite_definition_2) {
			self.process_sprite(&mut sprite_definition_2.unwrap().clone(), frame_index, transform, true, level);
		}
	}

	fn sprite_definition_is_visible(&self, sprite_definition: &Option<&SpriteDefinition>) -> bool {
		sprite_definition.is_some()
	}

}