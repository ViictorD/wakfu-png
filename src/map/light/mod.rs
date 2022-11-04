use std::{io::{Seek, Read}, collections::HashMap};
use anyhow::{Result, anyhow};
use byte::{BytesExt, TryRead};
use itertools::Itertools;

use crate::utils::math_helper::MathHelper;

use self::cell_light_def::CellLightDef;

mod cell_light_def;

pub struct MapLightChunk {
	x: i16,
	y: i16,
	layer_colors: HashMap<u16, CellLightDef>
}

impl<'a> TryRead<'a> for MapLightChunk {
	fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let x = bytes.read::<i16>(offset)? * 18;
		let y = bytes.read::<i16>(offset)? * 18;
		let layer_colors = Self::load_layers(bytes, offset)?;

		let result = MapLightChunk {
			x,
			y,
			layer_colors
		};

		Ok((result, *offset))
	}
}

impl MapLightChunk {
	fn load_layers(bytes: &[u8], offset: &mut usize) -> byte::Result<HashMap<u16, CellLightDef>>{
		let mut cell_light_def = Self::read_definition(bytes, offset)?;
		Ok(Self::read_layers(&mut cell_light_def, bytes, offset)?)
	}

	fn read_definition(bytes: &[u8], offset: &mut usize) -> byte::Result<Vec<CellLightDef>> {
		let def_count: u16 = bytes.read(offset)?;
		let mut cell_light_def = Vec::with_capacity(def_count as usize);
		for _ in 0..def_count {
			let allow_out_door_lighting = bytes.read::<i8>(offset)? != 0;
			let ambiance: i32 = bytes.read(offset)?;
			let shadow: i32 = bytes.read(offset)?;
			let light: i32 = bytes.read(offset)?;
			let def = CellLightDef::new(ambiance, shadow, light, allow_out_door_lighting);
			cell_light_def.push(def);
		}
		Ok(cell_light_def)
	}

	fn read_layers(cell_light_def: &mut Vec<CellLightDef>, bytes: &[u8], offset: &mut usize) -> byte::Result<HashMap<u16, CellLightDef>> {
		let _layer_count: i16 = bytes.read(offset)?;
		let count: i16 = bytes.read(offset)?;
		let mut layer_colors: HashMap<u16, CellLightDef> = HashMap::new();

		for _ in 0..count {
			let k: u16 = bytes.read(offset)?;
			let idx: u16 = bytes.read(offset)?;
			layer_colors.insert(k, cell_light_def[idx as usize].clone());
		}

		Ok(layer_colors)
	}

	pub fn get_light_info(&self, x: i32, y: i32, layer: i32) -> Result<&CellLightDef> {
		if let Some(res) = self.layer_colors.get(&self.get_hashcode(x, y, layer)) {
			return Ok(res);
		}
		Err(anyhow!("No layer color for x: {}, y: {}, layer: {}", x, y, layer))
	}

	fn get_hashcode(&self, x: i32, y: i32, layer: i32) -> u16 {
		(x - self.x as i32 + (y - self.y as i32 + layer * 18) * 18) as u16
	}
}

pub struct MapLight {
	pub chunks: HashMap<i32, MapLightChunk>
}

impl MapLight {
	pub fn load<R: Seek + Read>(input: R) -> Result<Self> {
		let mut archive = zip::ZipArchive::new(input)?;

		let mut chunks = HashMap::with_capacity(archive.len());
		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			if file
				.name()
				.trim_matches(|c| char::is_numeric(c) || c == '-')
				== "_"
			{
				let splited = file.name().split("_").collect_vec();
				let x = splited.get(0).unwrap().parse::<i32>()?;
				let y = splited.get(1).unwrap().parse::<i32>()?;
				let mut buffer = Vec::with_capacity(file.size() as usize);
				file.read_to_end(&mut buffer)?;
				let chunk = buffer
					.read(&mut 0)
					.map_err(|err| anyhow!("Read error: {:?}", err))?;
				chunks.insert(MathHelper::get_int_from_two_int(x, y) , chunk);
			}
		}
		Ok(MapLight { chunks })
	}

	pub fn get_light_info(&self, x: i32, y: i32, layer: i32) -> Result<&CellLightDef> {
		if let Some(map) = self.get_maps_from_cell(x, y) {
			return map.get_light_info(x, y, layer);
		}
		Err(anyhow!("Light for cell x:{} y:{} layer:{} not found", x, y, layer))
	}

	fn get_maps_from_cell(&self, x: i32, y: i32) -> Option<&MapLightChunk> {
		let map_x = MathHelper::fast_floor(x as f32 / 18.);
		let map_y = MathHelper::fast_floor(y as f32 / 18.);
		self.chunks.get(&MathHelper::get_int_from_two_int(map_x, map_y))
	}

	pub fn apply(&self, x: i32, y: i32, layer: i32, color: &mut [f32; 3]) -> Result<()> {
		let cell_light_def = self.get_light_info(x, y, layer)?;
		let c = cell_light_def.get_color();
		for i in 0..3 {
			color[i] *= c[i];
		}
		Ok(())
	}
}