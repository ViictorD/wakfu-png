use std::fs::{File, self};
use std::path::PathBuf;

use anyhow::Result;
use assets::gfx::Gfx;
use assets::tgam::TgamLoader;
use bdata::interactive_element_model_binary_data::InteractiveElementModelBinaryData;
use bdata::teleporter_binary_data::{TeleporterBinaryData};
use map::environment::EnvironmentChunk;
use map::groups::Groups;
use map::layer_manager::LayerManager;
use map::{element::ElementLibrary};
use map::Map;
use pico_args::Arguments;
use png_creator::{recursive_create_png, create_png};
use tplg::Tplg;

use crate::bdata::binary_document::BinaryDocument;

mod assets;
mod map;
mod bdata;
mod utils;
mod anm;
mod particles;
mod lib;
mod tplg;
mod png_creator;

fn main() -> Result<()> {
	let mut pargs = Arguments::from_env();

	let game_path: PathBuf = pargs.value_from_str("--path")?;
	let map_id: i32 = pargs.value_from_str("--map")?;
	let is_recursive: bool = pargs.contains("--recursive");
	let is_indoor: bool = pargs.contains("--indoor");
	
	let contents_path = game_path.join("contents");
	let maps_path = contents_path.join("maps");
	
	let gfx_path = maps_path.join("gfx.jar");
	let animation_path = contents_path.join("animations");
	let interactive_path = animation_path
		.join("interactives")
		.join("interactives.jar");
	let dynamic_path = animation_path
		.join("dynamics")
		.join("dynamics.jar");
	let particles_path = contents_path
		.join("particles")
		.join("particles.jar");
	let map_path = maps_path.join("gfx").join(format!("{}.jar", map_id));
	let lib_path = maps_path.join("data.jar");
	let env_path = maps_path.join("env").join(format!("{}.jar", map_id));
	let tplg_path = maps_path.join("tplg").join(format!("{}.jar", map_id));
	let iem_path = contents_path.join("bdata").join("34.jar");
	let teleporter_path = contents_path.join("bdata").join("72.jar");
	
	let lib = ElementLibrary::load(File::open(lib_path)?)?;
	let mut gfx = Gfx::load(
		File::open(gfx_path)?,
		File::open(interactive_path)?,
		File::open(dynamic_path)?,
		File::open(particles_path)?
	);
	let mut iem = BinaryDocument::load(File::open(iem_path)?, 34)?;
	let iem_data = InteractiveElementModelBinaryData::read(&mut iem);

	let groups =
		if is_indoor { Some(Groups::load(File::open(map_path.clone())?)?) }
		else { None };

	let tplg =
		if is_indoor { Some(Tplg::load(File::open(tplg_path)?)?) }
		else { None };

	let output_path = PathBuf::from("./output");
	if !output_path.exists() {
		fs::create_dir(output_path.clone())?;
	}
	if is_recursive {
		let mut teleporter = BinaryDocument::load(File::open(teleporter_path)?, 72)?;
		let teleporters_data = TeleporterBinaryData::read(&mut teleporter);
		let mut processed_map = Vec::new();
		let worlds_id = vec![map_id];
		recursive_create_png(
			&mut processed_map,
			worlds_id,
			&teleporters_data,
			&maps_path,
			&lib,
			&mut gfx,
			&iem_data,
			&groups,
			&tplg,
			output_path
		)?;
	}
	else {
		let map = Map::load(File::open(map_path)?)?;
		let visible_layers = 
			if is_indoor { Some(LayerManager::get_outdoor_visible_layers(&map,  &groups.unwrap(), &tplg.unwrap())) }
			else { None };

		let env = EnvironmentChunk::load(File::open(env_path)?)?;
		create_png(
			map_id,
			map,
			&lib,
			&mut gfx,
			env,
			&iem_data,
			visible_layers,
			output_path.clone().into_os_string().into_string().unwrap()
		)?;
	}

	Ok(())
}