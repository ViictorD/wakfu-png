use std::{collections::{HashMap}};

use anyhow::{Result};
use bytebuffer::ByteBuffer;
use crate::utils::utils::{read_string_without_len, java_string_hashcode};

use super::{
	anm_action::anm_action::{AnmAction},
	sprite_definition::sprite_definition::{SpriteDefinition}
};

pub struct AnimHeader {
	pub version_number: i8,
	pub frame_rate: i8
}

impl Default for AnimHeader {
	fn default() -> Self {
		AnimHeader {
			version_number: 0,
			frame_rate: 0
		}
	}
}

impl AnimHeader {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let version_number = buffer.read_i8()?;
		buffer.read_i16()?;
		let frame_rate = buffer.read_i8()?;

		let result = AnimHeader {
			version_number,
			frame_rate
		};
		Ok(result)
	}
}

pub struct HiddingPart {
	_crc_key: i32,
	_crc_to_hide: i32
}

pub struct CanHidePart {
	_item_name: String,
	_crc_key: i32
}

pub struct AnmIndexExtend {
	_heights: HashMap<i32, i8>,
	_high_light_color: Vec<f32>
}

impl Default for AnmIndexExtend {
	fn default() -> Self {
		AnmIndexExtend {
			_heights: HashMap::<i32, i8>::new(),
			_high_light_color: Vec::<f32>::new()
		}
	}
}

impl AnmIndexExtend {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let mut _heights: HashMap<i32, i8> = HashMap::new();

		let flags = buffer.read_i32()?;
		if (flags & 0x1) != 0x0 {
			let count = buffer.read_i16()?;
			for _ in 0..count {
				_heights.insert(
					buffer.read_i32()?,
					(buffer.read_i8()?).overflowing_add(1).0
				);
			}
		}
		
		let mut _high_light_color = Vec::new();
		if (flags & 0x2) == 0x2 {
			_high_light_color.push(buffer.read_f32()?);
			_high_light_color.push(buffer.read_f32()?);
			_high_light_color.push(buffer.read_f32()?);
		}

		let result = AnmIndexExtend {
			_heights,
			_high_light_color
		};

		Ok(result)
	}
}

#[derive(Clone)]
pub struct AnmAnimationFileRecord {
	pub name: String,
	pub crc: i32,
	pub file_index: i16
}

impl AnmAnimationFileRecord {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let name = read_string_without_len(buffer)?;
		buffer.read_i32()?;
		let crc = java_string_hashcode(&name);
		let file_index = buffer.read_i16()?;
		let result = AnmAnimationFileRecord {
			name,
			crc,
			file_index
		};
		Ok(result)
	}
}

pub struct AnimIndex {
	pub flags: i8,
	pub scale: f32,
	pub render_radius: f32,
	pub file_names: Vec<String>,
	pub part_hidden_by_item: Vec<HiddingPart>,
	pub can_hide_part_item: Vec<CanHidePart>,
	pub extend: AnmIndexExtend,
	pub animation_file_records: Vec<AnmAnimationFileRecord>,
	pub animation_file_records_by_name: HashMap<String, AnmAnimationFileRecord>
}

impl Default for AnimIndex {
	fn default() -> Self {
		AnimIndex {
			flags: 0,
			scale: 0.,
			render_radius: 0.,
			file_names: Vec::new(),
			part_hidden_by_item: Vec::new(),
			can_hide_part_item: Vec::new(),
			extend: AnmIndexExtend::default(),
			animation_file_records: Vec::new(),
			animation_file_records_by_name: HashMap::new()
		}
	}
}

impl AnimIndex {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let flags = buffer.read_i8()?;
		let scale = 
			if (flags & 0x1) != 0x0 { buffer.read_f32()? }
			else { 0. };
		let render_radius =
			if (flags & 0x8) != 0x0 { buffer.read_f32()? }
			else { 0. };

		let mut file_names: Vec<String> = Vec::new();
		if (flags & 0x2) != 0x0 {
			let num_files: i16 = buffer.read_i16()?;
			for _ in 0..num_files {
				let str = read_string_without_len(buffer)?;
				file_names.push(str);
			}
		}

		let mut part_hidden_by_item = Vec::new();
		if (flags & 0x4) != 0x0 {
			let num_part: i8 = buffer.read_i8()?;
			for _ in 0..num_part {
				let _crc_key: i32 = buffer.read_i32()?;
				let _crc_to_hide = buffer.read_i32()?;
				part_hidden_by_item.push(HiddingPart {
					_crc_key,
					_crc_to_hide
				});
			}
		}

		let mut can_hide_part_item = Vec::new();
		if (flags & 0x40) != 0x0 {
			let num_part: i8 = buffer.read_i8()?;
			for _ in 0..num_part {
				let _item_name = read_string_without_len(buffer)?;
				let crc_key2 = buffer.read_i32()?;
				can_hide_part_item.push(CanHidePart {
					_item_name,
					_crc_key: crc_key2
				});
			}
		}

		let extend =
			if (flags as u8 & 0x80) != 0x0 { AnmIndexExtend::read(buffer)? }
			else { AnmIndexExtend::default() };

		
		let num_animation_file_records = buffer.read_i16()?;
		let mut animation_file_records = Vec::with_capacity(num_animation_file_records as usize);
		let mut animation_file_records_by_name = HashMap::with_capacity(num_animation_file_records as usize);
		for _ in 0..num_animation_file_records {
			let file_record = AnmAnimationFileRecord::read(buffer)?;
			animation_file_records_by_name.insert(file_record.name.clone(), file_record.clone());
			animation_file_records.push(file_record);
		}

		let result = AnimIndex {
			flags,
			scale,
			render_radius,
			file_names,
			part_hidden_by_item,
			can_hide_part_item,
			extend,
			animation_file_records,
			animation_file_records_by_name
		};
		Ok(result)
	}

	pub fn use_flip(&self) -> bool {
		(self.flags & 0x10) == 0x0
	}
	
	pub fn get_animation_file_record(&self, anim_name: &String) -> &AnmAnimationFileRecord {
		if let Some(file_record) = self.animation_file_records_by_name.get(anim_name) {
			return file_record;
		}

		let splited_anm_name = anim_name.split("_").collect::<Vec<&str>>();
		if splited_anm_name.len() > 1 {
			let name_without_state = format!("{}_{}", splited_anm_name.get(0).unwrap(), splited_anm_name.get(1).unwrap());
			if let Some(file_record) = self.animation_file_records_by_name.get(&name_without_state) {
				return file_record;
			}
		}

		for name in self.animation_file_records_by_name.keys() {
			if name.contains("AnimStatique") {
				if let Some(file_record) = self.animation_file_records_by_name.get(name) {
					return file_record;
				}
			}
		}
		panic!("Anmimation file record not found");
	}
}

pub struct AnmShapeDefinition {
	pub id: i16,
	pub texture_index: i16,
	pub top: f32,
	pub left: f32,
	pub bottom: f32,
	pub right: f32,
	pub width: i16,
	pub height: i16,
	pub offset_x: f32,
	pub offset_y: f32
}

impl AnmShapeDefinition {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let id = buffer.read_i16()?;
		let texture_index = buffer.read_i16()?;
		let top = (buffer.read_i16()? as f32) / 65535.;
		let left = (buffer.read_i16()? as f32) / 65535.;
		let bottom = (buffer.read_i16()? as f32) / 65535.;
		let right = (buffer.read_i16()? as f32) / 65535.;
		let width = buffer.read_i16()?;
		let height = buffer.read_i16()?;
		let offset_x = buffer.read_f32()?;
		let offset_y = buffer.read_f32()?;
		
		let result = AnmShapeDefinition {
			id,
			texture_index,
			top,
			left,
			bottom,
			right,
			width,
			height,
			offset_x,
			offset_y
		};

		Ok(result)
	}
}

#[derive(Clone)]
pub struct AnmTransformDataTable {
	pub colors: Vec<f32>,
	pub rotations: Vec<f32>,
	pub translations: Vec<f32>,
	pub actions: Vec<AnmAction>
}

impl Default for AnmTransformDataTable {
	fn default() -> Self {
		AnmTransformDataTable {
			colors: Vec::new(),
			rotations: Vec::new(),
			translations: Vec::new(),
			actions: Vec::new()
		}
	}
}

impl AnmTransformDataTable {
	pub fn read(buffer: &mut ByteBuffer) -> Result<Self> {
		let colors = AnmTransformDataTable::read_floats(buffer)?;
		let rotations = AnmTransformDataTable::read_floats(buffer)?;
		let translations = AnmTransformDataTable::read_floats(buffer)?;
		let actions = AnmTransformDataTable::read_actions(buffer)?;

		let result = AnmTransformDataTable {
			colors,
			rotations,
			translations,
			actions
		};

		Ok(result)
	}

	pub fn read_floats(buffer: &mut ByteBuffer) -> Result<Vec<f32>> {
		let count: i32 = buffer.read_i32()?;
		let mut res = Vec::with_capacity(count as usize);
		for _ in 0..count {
			res.push(buffer.read_f32()?);
		}
		Ok(res)
	}

	pub fn read_actions(buffer: &mut ByteBuffer) -> Result<Vec<AnmAction>>{
		let count: i32 = buffer.read_i32().unwrap();
		let mut actions = Vec::with_capacity(count as usize);
		for _ in 0..count {
			let action_id = buffer.read_i8().unwrap();
			let parameters_count = buffer.read_i8().unwrap();
			let action: AnmAction = AnmAction::get(action_id as u8, parameters_count, buffer)?;
			actions.push(action);
		}

		Ok(actions)
	}

	pub fn is_empty(&self) -> bool {
		self.colors.len() == 0
			&& self.rotations.len() == 0
			&& self.translations.len() == 0
			&& self.actions.len() == 0
	}
}

#[derive(Clone)]
pub struct AnmImport {
	pub id: i16,
	pub name: String,
	pub crc: i32
}

impl AnmImport {
	pub fn new() -> Self {
		AnmImport {
			id: 0,
			name: String::default(),
			crc: 0
		}
	}

	pub fn load(&mut self, buffer: &mut ByteBuffer) {
		self.id = buffer.read_i16().unwrap();
		self.name = read_string_without_len(buffer).unwrap();
		buffer.read_i32().unwrap();
		self.crc = java_string_hashcode(&self.name);
	}
}

pub struct InteractiveAnim {
	pub max_sprite_count: i32,
	pub index: AnimIndex,
	header: AnimHeader,
	pub sprite_definitions: Vec<SpriteDefinition>,
	pub shape_definitions_by_id: HashMap<i16, AnmShapeDefinition>,
	pub sprite_definitions_by_id: HashMap<i16, SpriteDefinition>,
	sprite_definitions_by_crc: HashMap<i32, SpriteDefinition>,
	pub imports_by_id: HashMap<i16, AnmImport>,
	_texture_crc: i64,
	pub use_perfect_hit_test: bool,
	pub texture_name: String,
	table: Option<AnmTransformDataTable>
}

impl InteractiveAnim {
	pub fn new() -> Self {
		let max_sprite_count = -1;
		let index = AnimIndex::default();
		let header = AnimHeader::default();
		let sprite_definitions = Vec::new();
		let shape_definitions_by_id = HashMap::new();
		let sprite_definitions_by_id = HashMap::new();
		let sprite_definitions_by_crc = HashMap::new();
		let imports_by_id = HashMap::new();
		let _texture_crc = 0i64;
		let use_perfect_hit_test = false;
		let texture_name = String::default();
		let table = None;

		InteractiveAnim {
			max_sprite_count,
			index,
			header,
			sprite_definitions,
			shape_definitions_by_id,
			sprite_definitions_by_id,
			sprite_definitions_by_crc,
			imports_by_id,
			_texture_crc,
			use_perfect_hit_test,
			texture_name,
			table
		}
	}

	pub fn read(&mut self, mut buffer: ByteBuffer) {
		self.header = AnimHeader::read(&mut buffer).unwrap();

		if (self.header.version_number & 0x2) == 0x0 {
			return ;
		}
		self.index = AnimIndex::read(&mut buffer).unwrap();

		self.use_perfect_hit_test = (self.header.version_number & 0x4) == 0x4;
		let optimized = (self.header.version_number & 0x8) == 0x8;
		let num_texture = buffer.read_i16().unwrap();
		if num_texture == 1 {
			self.texture_name = read_string_without_len(&mut buffer).unwrap(); // .PNG atlas name
			let _base_crc = buffer.read_i32().unwrap();
		}

		// Shapes inside atlas file
		let num_shapes = buffer.read_i16().unwrap();
		for _ in 0..num_shapes {
			let shape_definition = AnmShapeDefinition::read(&mut buffer).unwrap();
			self.shape_definitions_by_id.insert(shape_definition.id, shape_definition);
		}

		// Transform informations
		self.table =
			if (self.header.version_number & 0x10) == 0x10 { Some(AnmTransformDataTable::read(&mut buffer).unwrap()) }
			else { None };
		
		let num_sprites = buffer.read_i16().unwrap();
		let use_flip = (self.index.flags & 0x10) == 0x0;
		for _ in 0..num_sprites {
			let mut def = SpriteDefinition::create_from(&self.table, &mut buffer, optimized).unwrap();
			def.sprite_def.load(&mut buffer);
			
			if def.sprite_def.get_frame_count() != 0 {
				if def.sprite_def.get_sprite_def().max_sprite_count > self.max_sprite_count {
					self.max_sprite_count = def.sprite_def.get_sprite_def().max_sprite_count;
				}
				if use_flip && InteractiveAnim::is_flippable_animation(&def.sprite_def.get_sprite_def().name) {
					def = SpriteDefinition::default();
				}
				else {
					self.sprite_definitions_by_id.insert(def.sprite_def.get_sprite_def().id, def.clone());
					self.sprite_definitions_by_crc.insert(def.sprite_def.get_sprite_def().name_crc, def.clone());
				}
			}
			self.sprite_definitions.push(def);
		}
		let num_imports = buffer.read_i16().unwrap();
		for _ in 0.. num_imports {
			let mut anm_import = AnmImport::new();
			anm_import.load(&mut buffer);
			self.imports_by_id.insert(anm_import.id, anm_import);
		}
		if !optimized {
			let _num_actions = buffer.read_i16().unwrap();
		}
	}

	fn is_flippable_animation(name: &String) -> bool{
		if name.len() == 0 {
			return false;
		}
		if !name.starts_with("_Anim") {
			return false;
		}
		let direction = name.chars().next().unwrap();
		direction == '3' || direction == '4' || direction == '7'
	}

	pub fn get_sprite_definition_by_crc(&self, crc: i32) -> Option<&SpriteDefinition> {
		self.sprite_definitions_by_crc.get(&crc)
	}

	pub fn get_import(&self, id: i16) -> Option<&AnmImport> {
		self.imports_by_id.get(&id)
	}
}