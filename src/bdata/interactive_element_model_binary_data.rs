use std::collections::HashMap;

use super::binary_document::BinaryDocument;

#[derive(Debug)]
pub struct InteractiveElementModelBinaryData {
	pub _view_model_id: i32,
	pub _view_type_id: i16,
	pub gfx: i32,
	pub _color: i32,
	pub _height: i8,
	pub particle_id: i32,
	pub particle_offset_z: i32
}

impl InteractiveElementModelBinaryData {
	pub fn read(document: &mut BinaryDocument) -> HashMap<i32, InteractiveElementModelBinaryData> {
		let mut iem: HashMap<i32, InteractiveElementModelBinaryData> = HashMap::new();
		
		for i in 0..document.entries.len() {
			let entry = document.entries.get(i).unwrap();
			document.buffer.position(entry.position, entry.seed);

			let _view_model_id: i32 = document.buffer.get_int();
			let _view_type_id: i16 = document.buffer.get_short();
			let gfx: i32 = document.buffer.get_int();
			let _color: i32 = document.buffer.get_int();
			let _height: i8 = document.buffer.get_byte();
			let particle_id: i32 = document.buffer.get_int();
			let particle_offset_z: i32 = document.buffer.get_int();

			let result = InteractiveElementModelBinaryData {
				_view_model_id,
				_view_type_id,
				gfx,
				_color,
				_height,
				particle_id,
				particle_offset_z
			};
			iem.insert(_view_model_id, result);
		}
		iem
	}
}
