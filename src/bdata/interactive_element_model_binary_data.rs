use std::collections::HashMap;

use super::binary_document::BinaryDocument;

#[derive(Debug)]
pub struct InteractiveElementModelBinaryData {
	pub view_model_id: i32,
	pub view_type_id: i16,
	pub gfx: i32,
	pub color: i32,
	pub height: i8,
	pub particle_id: i32,
	pub particle_offset_z: i32
}

impl InteractiveElementModelBinaryData {
	pub fn read(document: &mut BinaryDocument) -> HashMap<i32, InteractiveElementModelBinaryData> {
		let mut iem: HashMap<i32, InteractiveElementModelBinaryData> = HashMap::new();
		
		for i in 0..document.entries.len() {
			let entry = document.entries.get(i).unwrap();
			document.buffer.position(entry.position, entry.seed);

			let view_model_id: i32 = document.buffer.get_int();
			let view_type_id: i16 = document.buffer.get_short();
			let gfx: i32 = document.buffer.get_int();
			let color: i32 = document.buffer.get_int();
			let height: i8 = document.buffer.get_byte();
			let particle_id: i32 = document.buffer.get_int();
			let particle_offset_z: i32 = document.buffer.get_int();

			let result = InteractiveElementModelBinaryData {
				view_model_id,
				view_type_id,
				gfx,
				color,
				height,
				particle_id,
				particle_offset_z
			};
			iem.insert(view_model_id, result);
		}
		iem
	}
}