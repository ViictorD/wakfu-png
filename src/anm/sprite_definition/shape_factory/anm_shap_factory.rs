use anyhow::{Result, anyhow};
use bytebuffer::ByteBuffer;

use super::{
	anm_shape::{AnmShapeTrait, AnmShape},
	anm_shape_r::AnmShapeR,
	anm_shape_t::AnmShapeT,
	anm_shape_rt::AnmShapeRT,
	anm_shape_a::AnmShapeA,
	anm_shape_ra::AnmShapeRA,
	anm_shape_ta::AnmShapeTA,
	anm_shape_rta::AnmShapeRTA,
	anm_shape_m::AnmShapeM,
	anm_shape_rm::AnmShapeRM,
	anm_shape_tm::AnmShapeTM,
	anm_shape_am::AnmShapeAM,
	anm_shape_rtm::AnmShapeRTM,
	anm_shape_ram::AnmShapeRAM,
	anm_shape_tam::AnmShapeTAM,
	anm_shape_rtam::AnmShapeRTAM,
	anm_shape_cr::AnmShapeCR,
	anm_shape_ct::AnmShapeCT,
	anm_shape_crt::AnmShapeCRT
};

pub struct AnmShapeFactory;

impl AnmShapeFactory {
	pub fn create_shape(buffer: &mut ByteBuffer) -> Result<Box<dyn AnmShapeTrait>> {
		let id: i16 = buffer.read_i16().unwrap();
		let data_descriptor: i8 = buffer.read_i8().unwrap();
		match data_descriptor {
			0 => Ok(Box::new(AnmShape::load(buffer, id)?)),
			1 => Ok(Box::new(AnmShapeR::load(buffer, id)?)),
			2 => Ok(Box::new(AnmShapeT::load(buffer, id)?)),
			3 => Ok(Box::new(AnmShapeRT::load(buffer, id)?)),
			4 => Ok(Box::new(AnmShapeA::load(buffer, id)?)),
			5 => Ok(Box::new(AnmShapeRA::load(buffer, id)?)),
			6 => Ok(Box::new(AnmShapeTA::load(buffer, id)?)),
			7 => Ok(Box::new(AnmShapeRTA::load(buffer, id)?)),
			8 => Ok(Box::new(AnmShapeM::load(buffer, id)?)),
			9 => Ok(Box::new(AnmShapeRM::load(buffer, id)?)),
			10 => Ok(Box::new(AnmShapeTM::load(buffer, id)?)),
			12 => Ok(Box::new(AnmShapeAM::load(buffer, id)?)),
			11 => Ok(Box::new(AnmShapeRTM::load(buffer, id)?)),
			13 => Ok(Box::new(AnmShapeRAM::load(buffer, id)?)),
			14 => Ok(Box::new(AnmShapeTAM::load(buffer, id)?)),
			15 => Ok(Box::new(AnmShapeRTAM::load(buffer, id)?)),
			49 => Ok(Box::new(AnmShapeCR::load(buffer, id)?)),
			82 => Ok(Box::new(AnmShapeCT::load(buffer, id)?)),
			-77 => Ok(Box::new(AnmShapeCRT::load(buffer, id)?)),
			_ => {
				Err(anyhow!("Index not found"))
			}
		}
	}
}