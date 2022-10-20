use anyhow::{anyhow, Result};
use bytebuffer::ByteBuffer;

use crate::particles::{condition::ConditionType, particle::Particle};

use super::{
	attraction_force::AttractionForce,
	boost_force::BoostForce,
	circle_path::CirclePath,
	color_fader::ColorFader,
	deformer::Deformer,
	direction_follower::DirectionFollower,
	frictional_force::FrictionalForce,
	linear_force::LinearForce,
	rebound::Rebound,
	rotor_force::RotorForce,
	light_radius_deformer::LightRadiusDeformer,
	curve::Curve,
	rotation::Rotation,
	rotation_interpolation::RotationInterpolation, affector_validator::{AffectorValidatorType, ConditionValidator, TimeValidator},
};

#[derive(Clone)]
pub enum AffectorType {
	AttractionForce(AttractionForce),
	BoostForce(BoostForce),
	CirclePath(CirclePath),
	ColorFader(ColorFader),
	Deformer(Deformer),
	DirectionFollower(DirectionFollower),
	FrictionalForce(FrictionalForce),
	LinearForce(LinearForce),
	Rebound(Rebound),
	RotorForce(RotorForce),
	LightRadiusDeformer(LightRadiusDeformer),
	Curve(Curve),
	Rotation(Rotation),
	RotationInterpolation(RotationInterpolation),
	None
}

impl AffectorType {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Result<Self> {
		let affector_type = buffer.read_i8().unwrap();
		match affector_type {
			1 => Ok(AffectorType::AttractionForce(AttractionForce::load(buffer, level_percent))),
			2 => Ok(AffectorType::BoostForce(BoostForce::load(buffer, level_percent))),
			3 => Ok(AffectorType::CirclePath(CirclePath::load(buffer, level_percent))),
			4 => Ok(AffectorType::ColorFader(ColorFader::load(buffer, level_percent))),
			5 => Ok(AffectorType::Deformer(Deformer::load(buffer, level_percent))),
			6 => Ok(AffectorType::DirectionFollower(DirectionFollower::load())),
			7 => Ok(AffectorType::FrictionalForce(FrictionalForce::load(buffer, level_percent))),
			8 => Ok(AffectorType::LinearForce(LinearForce::load(buffer, level_percent))),
			9 => Ok(AffectorType::Rebound(Rebound::load(buffer, level_percent))),
			10 => Ok(AffectorType::RotorForce(RotorForce::load(buffer, level_percent))),
			11 => Ok(AffectorType::LightRadiusDeformer(LightRadiusDeformer::load(buffer, level_percent))),
			12 => Ok(AffectorType::Curve(Curve::load(buffer, level_percent))),
			13 => Ok(AffectorType::Rotation(Rotation::load(buffer, level_percent))),
			14 => Ok(AffectorType::RotationInterpolation(RotationInterpolation::load(buffer, level_percent))),
			_ => Err(anyhow!("Affector type not found: {affector_type}"))
		}
	}
	
	pub fn update(&mut self, time_increment: f32, time_progress_ratio: f32, parent: &*const Particle, particle: &mut Particle) {
		match self {
			AffectorType::AttractionForce(attraction_force) => attraction_force.affect(time_increment, parent, particle),
			AffectorType::BoostForce(boost_force) => boost_force.affect(particle),
			AffectorType::CirclePath(circle_path) => circle_path.affect(time_increment, parent, particle),
			AffectorType::ColorFader(color_fader) => color_fader.affect(time_increment, particle),
			AffectorType::Deformer(deformer) => deformer.affect(time_increment, particle),
			AffectorType::DirectionFollower(direction_follower) => direction_follower.affect(parent, particle),
			AffectorType::FrictionalForce(frictional_force) => frictional_force.affect(time_increment, particle),
			AffectorType::LinearForce(linear_force) => linear_force.affect(time_increment, particle),
			AffectorType::Rebound(rebound) => rebound.affect(parent, particle),
			AffectorType::RotorForce(rotor_force) => rotor_force.affect(time_increment, parent, particle),
			AffectorType::LightRadiusDeformer(light_radius_deformer) => light_radius_deformer.affect(time_increment, particle),
			AffectorType::Curve(curve) => curve.affect(parent, particle),
			AffectorType::Rotation(rotation) => rotation.affect(time_increment, particle),
			AffectorType::RotationInterpolation(rotation_interpolation) => rotation_interpolation.affect(time_progress_ratio, particle),
			AffectorType::None => {}
		}
	}

	pub fn is_key_framed_affector(&self) -> bool {
		match self {
			AffectorType::AttractionForce(_) => true,
			_ => false
		}
	}
}

#[derive(Clone)]
pub struct Affector {
	pub affector: AffectorType,
	pub validator: AffectorValidatorType
}

impl Affector {
	pub fn new() -> Self {
		Affector {
			affector: AffectorType::None,
			validator: AffectorValidatorType::Always
		}
	}

	pub fn add_affector(&mut self, affector: AffectorType) {
		self.affector = affector;
	}

	pub fn add_condition(&mut self, conditions_opt: Option<Vec::<ConditionType>>) {
		if conditions_opt.is_none() {
			self.validator = AffectorValidatorType::Always;
			return ;
		}

		let conditions = conditions_opt.unwrap();

		let mut start = 0f32;
		let mut end = f32::MAX;
		let mut position_condition = None;

		for i in 0..conditions.len() {
			match conditions.get(i).unwrap() {
				ConditionType::PositionCondition(position) => {
					position_condition = Some(position.clone());
				},
				ConditionType::LifeCondition(life) => {
					start = life.start;
					end = life.end;
					if end <= start {
						end = f32::MAX;
					}
					
				}
			}
		}
		if position_condition.is_some() {
			self.validator = AffectorValidatorType::ConditionValidator(ConditionValidator::new(
				start,
				end,
				vec![position_condition.unwrap()]
			))
		}
		else if end != f32::MAX {
			self.validator = AffectorValidatorType::TimeValidator(TimeValidator::new(
				start,
				end
			))
		}
		else {
			self.validator = AffectorValidatorType::Always;
		}
		
	}

	pub fn is_key_framed_affector(&self) -> bool {
		self.affector.is_key_framed_affector()
	}

	pub fn update(&mut self, time_increment: f32, parent: &*const Particle, particle: &mut Particle) -> bool {
		match self.validator {
			AffectorValidatorType::Always => {
				let time_progress_ratio = particle.life / particle.life_time;
				self.affector.update(time_increment, time_progress_ratio, parent, particle);
				false
			},
			AffectorValidatorType::TimeValidator(ref time_validator) => {
				let life = particle.life;
				if life < time_validator.start {
					return false;
				}
				let d = time_validator.end - life;
				if d.lt(&0.) {
					return false;
				}
				let elapsed_time_since_start = time_increment - time_validator.start;
				let validator_total_time = time_validator.end - time_validator.start;
				let time_progress_ratio = (elapsed_time_since_start / validator_total_time).max(0.);
				let time = if d.lt(&time_increment) { d } else { time_increment };
				self.affector.update(time, time_progress_ratio, parent, particle);
				false
			},
			AffectorValidatorType::ConditionValidator(ref condition_validator) => {
				let life = particle.life;
				if life < condition_validator.start {
					return false;
				}
				let d = condition_validator.end - life;
				if d.lt(&0.) {
					return true;
				}
				for condition in &condition_validator.condition {
					if !condition.validate(parent, particle) {
						return false;
					}
				}
				let elapsed_time_since_start = time_increment - condition_validator.start;
				let validator_total_time = condition_validator.end - condition_validator.start;
				let time_progress_ratio = (elapsed_time_since_start / validator_total_time).max(0.);
				let time = if d.lt(&time_increment) { d } else { time_increment };
				self.affector.update(time, time_progress_ratio, parent, particle);
				false
			}
		}
	}
}

