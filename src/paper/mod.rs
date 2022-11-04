use xml::reader::{EventReader, XmlEvent};
use anyhow::{Result};
use image::{DynamicImage, codecs::dds::DdsDecoder, RgbaImage};
use std::{io::{Seek, Read, BufReader}, collections::HashMap};

pub mod render_paper;
pub mod anm_coords;

pub struct CoordsXml {
	pub id: i32,
	pub coord_type: String,
	pub start_x: i16,
	pub start_y: i16,
	pub end_x: i16,
	pub end_y: i16,
	pub anim_name_1: String,
	pub anim_name_2: String
}

impl Default for CoordsXml {
	fn default() -> Self {
		CoordsXml {
			id: 0,
			coord_type: String::new(),
			start_x: 0,
			start_y: 0,
			end_x: 0,
			end_y: 0,
			anim_name_1: String::new(),
			anim_name_2: String::new()
		}
	}
}

impl CoordsXml {
	fn load(buffer: Vec<u8>) -> Result<Vec<Self>> {
		let mut result = Vec::new();
		let buf_reader = BufReader::new(buffer.as_slice());
		let parser = EventReader::new(buf_reader);
		for e in parser {
			match e {
				Ok(XmlEvent::StartElement { name, attributes, .. }) => {
					if name.local_name.eq("coord") {
						let mut xml_coord = CoordsXml::default();
						for attribute in &attributes {
							match attribute.name.local_name.as_str() {
								"id" => xml_coord.id = attribute.value.parse::<i32>().unwrap(),
								"type" => xml_coord.coord_type = attribute.value.clone(),
								"startX" => xml_coord.start_x = attribute.value.parse::<i16>().unwrap(),
								"startY" => xml_coord.start_y = attribute.value.parse::<i16>().unwrap(),
								"endX" => xml_coord.end_x = attribute.value.parse::<i16>().unwrap(),
								"endY" => xml_coord.end_y = attribute.value.parse::<i16>().unwrap(),
								"animName1" => xml_coord.anim_name_1 = attribute.value.clone(),
								"animName2" => xml_coord.anim_name_2 = attribute.value.clone(),
								_ => {}
							}
						}
						result.push(xml_coord);
					}
				}
				Err(e) => {
					println!("Error: {}", e);
				}
				_ => {}
			}
		}

		Ok(result)
	}
}

pub struct MapXml {
	pub texture: String,
	pub iso_x: i32,
	pub iso_y: i32,
	pub iso_width: i32,
	pub iso_height: i32,
	pub width: i32,
	pub height: i32,
}

impl Default for MapXml {
	fn default() -> Self {
		MapXml {
			texture: String::new(),
			iso_x: 0,
			iso_y: 0,
			iso_width: 0,
			iso_height: 0,
			width: 0,
			height: 0
		}
	}
}

impl MapXml {
	fn load(buffer: Vec<u8>) -> Result<Self> {
		let mut result =  MapXml::default();
		let buf_reader = BufReader::new(buffer.as_slice());
		let parser = EventReader::new(buf_reader);
		for e in parser {
			match e {
				Ok(XmlEvent::StartElement { name, attributes, .. }) => {
					if name.local_name.eq("mapNavigatorBackgroundPart") {
						for attribute in &attributes {
							match attribute.name.local_name.as_str() {
								"texture" => result.texture = attribute.value.clone(),
								"isoX" => result.iso_x = attribute.value.parse::<i32>().unwrap(),
								"isoY" => result.iso_y = attribute.value.parse::<i32>().unwrap(),
								"isoWidth" => result.iso_width = attribute.value.parse::<i32>().unwrap(),
								"isoHeight" => result.iso_height = attribute.value.parse::<i32>().unwrap(),
								"width" => result.width = attribute.value.parse::<i32>().unwrap(),
								"height" => result.height = attribute.value.parse::<i32>().unwrap(),
								_ => {}
							}
						}
						break ;
					}
				}
				Err(e) => {
					println!("Error: {}", e);
				}
				_ => {}
			}
		}

		Ok(result)
	}
}

pub struct PaperGroup {
	pub coords: Vec<CoordsXml>,
	pub texture: RgbaImage,
	pub map: MapXml
}

impl Default for PaperGroup {
	fn default() -> Self {
		PaperGroup {
			coords: Vec::new(),
			texture: RgbaImage::new(0, 0),
			map: MapXml::default()
		}
	}
}

pub struct Paper {
	pub maps: HashMap<i32, PaperGroup>
}

impl Paper {
	pub fn load<R: Seek + Read>(input: R) -> Result<Self> {
		let mut archive = zip::ZipArchive::new(input)?;

		let mut maps: HashMap<i32, PaperGroup> = HashMap::new();

		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			if file.name().contains("/") {
				let name = file.name().to_string();
				let path: Vec<&str> = name.split("/").collect();
				if path.len() < 2 || path.get(1).unwrap().len() == 0 {
					continue ;
				}
				if let Ok(dir) = path.get(0).unwrap().parse::<i32>() {
					let mut buffer = Vec::with_capacity(file.size() as usize);
					file.read_to_end(&mut buffer)?;
					if !maps.contains_key(&dir) {
						maps.insert(dir, PaperGroup::default());
					}
					if file.name().contains(".dds") {
						let image = DynamicImage::from_decoder(
							DdsDecoder::new(buffer.as_slice())?
						)?
						.into_rgba8();
						maps.get_mut(&dir).unwrap().texture = image;
					}
					if (*path.get(1).unwrap()).eq("coords.xml") {
						if &buffer[..3] == b"\xEF\xBB\xBF" {
							buffer.drain(0..3);
						}
						let coords_xml = CoordsXml::load(buffer)?;
						maps.get_mut(&dir).unwrap().coords = coords_xml;
					}
					else if (*path.get(1).unwrap()).eq("map.xml") {
						let map_xml = MapXml::load(buffer)?;
						maps.get_mut(&dir).unwrap().map = map_xml;
					}
				}
			}
		}

		let result = Paper {
			maps
		};
		Ok(result)
	}
}