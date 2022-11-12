use std::fs::{File};
use std::path::PathBuf;

use anyhow::Result;
use assets::gfx::Gfx;
use assets::tgam::TgamLoader;
use bdata::interactive_element_model_binary_data::InteractiveElementModelBinaryData;
use bdata::teleporter_binary_data::TeleporterBinaryData;
use map::environment::EnvironmentChunk;
use map::groups::Groups;
use map::layer_manager::LayerManager;
use map::element::ElementLibrary;
use map::Map;
use paper::Paper;
use paper::render_paper::render_papers;
use pico_args::Arguments;
use png_creator::{recursive_create_png, create_png};
use tplg::Tplg;

use crate::bdata::binary_document::BinaryDocument;
use crate::map::light::MapLight;

mod assets;
mod map;
mod bdata;
mod utils;
mod anm;
mod particles;
mod custom_lib;
mod tplg;
mod png_creator;
mod paper;

fn main() -> Result<()> {
	let mut pargs = Arguments::from_env();

	let game_path: PathBuf = pargs.value_from_str("--path")?;
	let map_id_flag: String = pargs.value_from_str("--map")?;
	let is_recursive: bool = pargs.contains("--recursive");
	let is_indoor: bool = pargs.contains("--indoor");

	let is_paper = if map_id_flag.eq("paper") { true } else { false };
	let map_id = if is_paper { 999 } else { map_id_flag.parse::<i32>().unwrap() };
	let contents_path = game_path.join("contents");
	let maps_path = contents_path.join("maps");
	let animation_path = contents_path.join("animations");
	
	let gfx_path = maps_path.join("gfx.jar");
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
	let light_path = maps_path.join("light").join(format!("{}.jar", map_id));
	let iem_path = contents_path.join("bdata").join("34.jar");
	let teleporter_path = contents_path.join("bdata").join("72.jar");
	let paper_anm_path = animation_path.join("gui").join("gui.jar");

	let mut gfx = Gfx::load(
		File::open(gfx_path)?,
		File::open(interactive_path)?,
		File::open(dynamic_path)?,
		File::open(particles_path)?,
		File::open(paper_anm_path)?
	);

	if is_paper {
		let paper_path = maps_path.join("paper").join("full.jar");
		let paper = Paper::load(File::open(paper_path)?)?;

		println!("Loading papers...");
		let data = gfx.load_paper_as_rgba_images(&paper)?;
		println!("Processing papers...");
		render_papers(paper, data);
		println!("Done");
		return Ok(());
	}

	let lib = ElementLibrary::load(File::open(lib_path)?)?;
	let mut iem = BinaryDocument::load(File::open(iem_path)?, 34)?;
	let iem_data = InteractiveElementModelBinaryData::read(&mut iem);

	let groups =
		if is_indoor { Some(Groups::load(File::open(map_path.clone())?)?) }
		else { None };

	let tplg =
		if is_indoor { Some(Tplg::load(File::open(tplg_path)?)?) }
		else { None };

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
		)?;
	}
	else {
		let map = Map::load(File::open(map_path)?)?;
		let light_map = MapLight::load(File::open(light_path)?)?;
		let visible_layers = 
			if is_indoor { Some(LayerManager::get_outdoor_visible_layers(&map, &groups.unwrap(), &tplg.unwrap())) }
			else { None };

		if visible_layers.is_some() && visible_layers.as_ref().unwrap().len() == 0 {
			println!("No indoor to render, exiting.");
			return Ok(());
		}
		let output_path = PathBuf::from("./output").join(map_id.to_string());

		let env = EnvironmentChunk::load(File::open(env_path)?)?;
		create_png(
			map_id,
			map,
			&light_map,
			&lib,
			&mut gfx,
			env,
			&iem_data,
			visible_layers,
			output_path
		)?;
	}
	println!("Done");
	Ok(())
}