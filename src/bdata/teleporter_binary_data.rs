use super::binary_document::BinaryDocument;

pub struct TeleporterBinaryData {
	pub teleporter_id: i32,
	pub destinations: Vec<Destination>
}

impl TeleporterBinaryData {
	pub fn read(document: &mut BinaryDocument) -> Vec<TeleporterBinaryData> {
		let mut teleporters = Vec::with_capacity(document.entries.len());

		for i in 0..document.entries.len() {
			let entry = document.entries.get(i).unwrap();
			document.buffer.position(entry.position, entry.seed);

			let teleporter_id = document.buffer.get_int();
			let destination_count = document.buffer.get_int();
			let mut destinations = Vec::with_capacity(destination_count as usize);
			for _ in 0..destination_count {
				destinations.push(Destination::read(document));
			}

			teleporters.push(
				TeleporterBinaryData {
					teleporter_id,
					destinations
				}
			);
		}
		teleporters
	}
}

pub struct Destination {
	destination_id: i32,
	x: i32,
	y: i32,
	z: i32,
	world_id: i32,
	direction: i8,
	criteria: String,
	visual_id: i32,
	aps_id: i32,
	delay: i16,
	item_consumed: i32,
	item_quantity: i16,
	kama_cost: i16,
	do_consume_item: bool,
	is_invisible: bool,
	unknown: i32,
	loading_animation_name: String,
	loading_min_duration: i32,
	loading_fade_in_duration: i32,
	loading_fade_out_duration: i32
}

impl Destination {
	pub fn read(document: &mut BinaryDocument) -> Self {
		let destination_id = document.buffer.get_int();
		let x = document.buffer.get_int();
		let y = document.buffer.get_int();
		let z = document.buffer.get_int();
		let world_id = document.buffer.get_int();
		let direction = document.buffer.get_byte();
		let criteria = document.buffer.read_utf8();
		let visual_id = document.buffer.get_int();
		let aps_id = document.buffer.get_int();
		let delay = document.buffer.get_short();
		let item_consumed = document.buffer.get_int();
		let item_quantity = document.buffer.get_short();
		let kama_cost = document.buffer.get_short();
		let do_consume_item = document.buffer.read_boolean();
		let is_invisible = document.buffer.read_boolean();
		let unknown = document.buffer.get_int();
		let loading_animation_name = document.buffer.read_utf8();
		let loading_min_duration = document.buffer.get_int();
		let loading_fade_in_duration = document.buffer.get_int();
		let loading_fade_out_duration = document.buffer.get_int();

		Destination {
			destination_id,
			x,
			y,
			z,
			world_id,
			direction,
			criteria,
			visual_id,
			aps_id,
			delay,
			item_consumed,
			item_quantity,
			kama_cost,
			do_consume_item,
			is_invisible,
			unknown,
			loading_animation_name,
			loading_min_duration,
			loading_fade_in_duration,
			loading_fade_out_duration
		}
	}
}