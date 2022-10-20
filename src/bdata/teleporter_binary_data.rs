use std::collections::HashMap;

use super::binary_document::BinaryDocument;

pub struct TeleporterBinaryData;

impl TeleporterBinaryData {
	pub fn read(document: &mut BinaryDocument) -> HashMap<i32, Vec<Destination>> {
		let mut teleporters = HashMap::new();

		for i in 0..document.entries.len() {
			let entry = document.entries.get(i).unwrap();
			document.buffer.position(entry.position, entry.seed);

			let teleporter_id = document.buffer.get_int();
			let destination_count = document.buffer.get_int();
			let mut destinations = Vec::with_capacity(destination_count as usize);
			for _ in 0..destination_count {
				destinations.push(Destination::read(document));
			}

			teleporters.insert(teleporter_id, destinations);
		}
		teleporters
	}
}

pub struct Destination {
	_destination_id: i32,
	_x: i32,
	_y: i32,
	_z: i32,
	pub world_id: i32,
	_direction: i8,
	_criteria: String,
	_visual_id: i32,
	_aps_id: i32,
	_delay: i16,
	_item_consumed: i32,
	_item_quantity: i16,
	_kama_cost: i16,
	_do_consume_item: bool,
	_is_invisible: bool,
	_unknown: i32,
	_loading_animation_name: String,
	_loading_min_duration: i32,
	_loading_fade_in_duration: i32,
	_loading_fade_out_duration: i32
}

impl Destination {
	pub fn read(document: &mut BinaryDocument) -> Self {
		let _destination_id = document.buffer.get_int();
		let _x = document.buffer.get_int();
		let _y = document.buffer.get_int();
		let _z = document.buffer.get_int();
		let world_id = document.buffer.get_int();
		let _direction = document.buffer.get_byte();
		let _criteria = document.buffer.read_utf8();
		let _visual_id = document.buffer.get_int();
		let _aps_id = document.buffer.get_int();
		let _delay = document.buffer.get_short();
		let _item_consumed = document.buffer.get_int();
		let _item_quantity = document.buffer.get_short();
		let _kama_cost = document.buffer.get_short();
		let _do_consume_item = document.buffer.read_boolean();
		let _is_invisible = document.buffer.read_boolean();
		let _unknown = document.buffer.get_int();
		let _loading_animation_name = document.buffer.read_utf8();
		let _loading_min_duration = document.buffer.get_int();
		let _loading_fade_in_duration = document.buffer.get_int();
		let _loading_fade_out_duration = document.buffer.get_int();

		Destination {
			_destination_id,
			_x,
			_y,
			_z,
			world_id,
			_direction,
			_criteria,
			_visual_id,
			_aps_id,
			_delay,
			_item_consumed,
			_item_quantity,
			_kama_cost,
			_do_consume_item,
			_is_invisible,
			_unknown,
			_loading_animation_name,
			_loading_min_duration,
			_loading_fade_in_duration,
			_loading_fade_out_duration
		}
	}
}