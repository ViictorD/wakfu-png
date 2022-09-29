use anyhow::{Result, anyhow};
use bytebuffer::ByteBuffer;

use super::{
	anm_action_go_to_animation::AnmActionGoToAnimation,
	anm_action_go_to_static_animation::AnmActionGoToStaticAnimation,
	anm_action_run_script::AnmActionRunScript,
	anm_action_go_to_random_animation::AnmActionGoToRandomAnimation,
	anm_action_hit::AnmActionHit,
	anm_action_delete::AnmActionDelete,
	anm_action_end::AnmActionEnd,
	anm_action_go_to_if_previous_animation::AnmActionGoToIfPreviousAnimation,
	anm_action_add_particle::AnmActionAddParticle,
	anm_action_set_radius::AnmActionSetRadius
};

pub trait AnmActionTrait: AnmActionClone {
	fn load(parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> where Self: Sized;
	fn get_type(&self) -> &'static str;
}

pub trait AnmActionClone {
	fn clone_box(&self) -> Box<dyn AnmActionTrait>;
}

impl<T> AnmActionClone for T
where
	T: 'static + AnmActionTrait + Clone,
{	
	fn clone_box(&self) -> Box<dyn AnmActionTrait> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn AnmActionTrait> {
	fn clone(&self) -> Self {
		self.clone_box()
	}
}


#[derive(Clone)]
pub struct AnmAction {
	pub action: Box<dyn AnmActionTrait>,
	frame_index: i32
}

impl AnmAction {
	pub fn get(index: u8, parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		let action: Result<Box<dyn AnmActionTrait>> = match index {
			1 => Ok(Box::new(AnmActionGoToAnimation::load(parameters_count, buffer)?)),
			2 => Ok(Box::new(AnmActionGoToStaticAnimation::load(parameters_count, buffer)?)),
			3 => Ok(Box::new(AnmActionRunScript::load(parameters_count, buffer)?)),
			4 => Ok(Box::new(AnmActionGoToRandomAnimation::load(parameters_count, buffer)?)),
			5 => Ok(Box::new(AnmActionHit::load(parameters_count, buffer)?)),
			6 => Ok(Box::new(AnmActionDelete::load(parameters_count, buffer)?)),
			7 => Ok(Box::new(AnmActionEnd::load(parameters_count, buffer)?)),
			8 => Ok(Box::new(AnmActionGoToIfPreviousAnimation::load(parameters_count, buffer)?)),
			9 => Ok(Box::new(AnmActionAddParticle::load(parameters_count, buffer)?)),
			10 => Ok(Box::new(AnmActionSetRadius::load(parameters_count, buffer)?)),
			_ => Err(anyhow!("Index not found"))
		};
		if let Err(err) = action {
			return Err(err);
		}
		let result = AnmAction {
			action: action.unwrap(),
			frame_index: 0
		};
		Ok(result)
	}

	pub fn set_frame_index(&mut self, frame_index: i32) {
		self.frame_index = frame_index;
	}
}