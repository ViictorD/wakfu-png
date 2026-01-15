use std::{str::FromStr, mem::size_of};

use anyhow::{anyhow, Result};
use bytebuffer::ByteBuffer;
use glam::Vec3;

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinarSerialPartsEnum {
	GlobalDataPart(GlobalDataPart),
	SpecificDataPart(SpecificDataPart),
	SynchronizationPart(SynchronizationPart),
	SynchronizationSpecificPart(SynchronizationSpecificPart),
	PersistancePart(PersistancePart),
	AdditionalPersistancePart(AdditionalPersistancePart),
}

pub trait BinarSerialPartsTrait {
	fn unserialize(buffer: ByteBuffer, version: i32) -> Self;
}

#[derive(Debug)]
pub struct Action {
	_action_id: i16,
	_script_id: i32
}

#[derive(Debug)]
pub struct GlobalDataPart {
	_actions: Vec<Action>
}

impl BinarSerialPartsTrait for GlobalDataPart {
	fn unserialize(mut buffer: ByteBuffer, _version: i32) -> Self {
		let n_actions: i8 = buffer.read_i8().unwrap();
		let mut _actions = Vec::with_capacity(n_actions as usize);
		for _ in 0..n_actions {
			let _action_id: i16 = buffer.read_i16().unwrap();
			let _script_id: i32 = buffer.read_i32().unwrap();
			_actions.push(Action {
				_action_id,
				_script_id
			});
		}
		GlobalDataPart {
			_actions
		}
	}
}

#[derive(Debug)]
pub struct SpecificDataPart {
	pub _world: i16,
	pub x: i32,
	pub y: i32,
	pub z: i16,
	pub state: i16,
	pub visible: bool,
	pub _usable: bool,
	pub direction: i8,
	pub activation_pattern: i16,
	pub _positions_trigger: Vec<Vec3>,
	pub parameter: String,
	pub _properties: Vec<String>
}

impl BinarSerialPartsTrait for SpecificDataPart {
	fn unserialize(mut buffer: ByteBuffer, _version: i32) -> Self {
		let _world: i16 = buffer.read_i16().unwrap();
		let x = buffer.read_i32().unwrap();
		let y = buffer.read_i32().unwrap();
		let z = buffer.read_i16().unwrap();
		let state = buffer.read_i16().unwrap();

		let visible = buffer.read_i8().unwrap() != 0;
		let _usable = buffer.read_i8().unwrap() != 0;
		let direction = buffer.read_i8().unwrap();

		let activation_pattern = buffer.read_i16().unwrap();
		
		let number_of_position_trigger = buffer.read_i16().unwrap();
		let mut _positions_trigger = Vec::with_capacity(number_of_position_trigger as usize * size_of::<Vec3>());
		for _ in 0..number_of_position_trigger {
			let pos = Vec3::new(
				buffer.read_i32().unwrap() as f32, 
				buffer.read_i32().unwrap() as f32,
				buffer.read_i16().unwrap() as f32
			);
			_positions_trigger.push(pos);
		}

		let size = buffer.read_i16().unwrap() as usize;
		let parameters = buffer.read_bytes(size).unwrap();
		let parameter = String::from_utf8(parameters).unwrap();

		let properties_count: i8 = buffer.read_i8().unwrap();
		let mut _properties: Vec<String>= Vec::with_capacity(properties_count as usize);
		if properties_count > 0 {
			for _ in 0..properties_count {
				let prop_id = buffer.read_i8().unwrap();
				let prop = 
					if prop_id == 0 { String::from_str("Element de Challenge").unwrap() }
					else { String::from_str("Element d'almanach ").unwrap() };
				_properties.push(prop);
			}
		}
		buffer.read_i32().unwrap();
		SpecificDataPart {
			_world,
			x,
			y,
			z,
			state,
			visible,
			_usable,
			direction,
			activation_pattern,
			_positions_trigger,
			parameter,
			_properties
		}
	}
}

#[derive(Debug)]
pub struct SynchronizationPart;

impl BinarSerialPartsTrait for SynchronizationPart {
	fn unserialize(mut buffer: ByteBuffer, _version: i32) -> Self {
		let _state = buffer.read_i16().unwrap();
		let _visible = buffer.read_i8().unwrap();
		let _usable = buffer.read_i8().unwrap();
		let _block_movements = buffer.read_i8().unwrap();
		let _blocking_line_of_sight = buffer.read_i8().unwrap();
		let _visible_content = buffer.read_i8().unwrap();
		let property_size = buffer.read_i32().unwrap();

		if property_size > 0 {
			let mut properties: Vec<String>= Vec::with_capacity(property_size as usize);
			for _ in 0..property_size {
				let prop_id = buffer.read_i8().unwrap();
				let prop = 
					if prop_id == 0 { String::from_str("Element de Challenge").unwrap() }
					else { String::from_str("Element d'almanach ").unwrap() };
				properties.push(prop);
			}
		}
		SynchronizationPart
	}
}

#[derive(Debug)]
pub struct SynchronizationSpecificPart;

impl BinarSerialPartsTrait for SynchronizationSpecificPart {
	fn unserialize(_buffer: ByteBuffer, _version: i32) -> Self{
		SynchronizationSpecificPart
	}
}

#[derive(Debug)]
pub struct PersistancePart;

impl BinarSerialPartsTrait for PersistancePart {
	fn unserialize(_buffer: ByteBuffer, _version: i32) -> Self {
		PersistancePart
	}
}

#[derive(Debug)]
pub struct AdditionalPersistancePart;

impl BinarSerialPartsTrait for AdditionalPersistancePart {
	fn unserialize(_buffer: ByteBuffer, _version: i32) -> Self {
		AdditionalPersistancePart
	}
}

pub struct BinarSerialParts;

impl BinarSerialParts {
	pub fn unserialize(index: u8, buffer: ByteBuffer, version: i32) -> Result<BinarSerialPartsEnum> {
		match index {
			0 => Ok(BinarSerialPartsEnum::GlobalDataPart(GlobalDataPart::unserialize(buffer, version))),
			1 => Ok(BinarSerialPartsEnum::SpecificDataPart(SpecificDataPart::unserialize(buffer, version))),
			2 => Ok(BinarSerialPartsEnum::SynchronizationPart(SynchronizationPart::unserialize(buffer, version))),
			3 => Ok(BinarSerialPartsEnum::SynchronizationSpecificPart(SynchronizationSpecificPart::unserialize(buffer, version))),
			4 => Ok(BinarSerialPartsEnum::PersistancePart(PersistancePart::unserialize(buffer, version))),
			5 => Ok(BinarSerialPartsEnum::AdditionalPersistancePart(AdditionalPersistancePart::unserialize(buffer, version))),
			_ => Err(anyhow!("Index not found in BinarSerialParts"))
		}
	}
}
