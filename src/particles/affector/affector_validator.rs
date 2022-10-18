use crate::particles::condition::PositionCondition;

#[derive(Clone)]
pub enum AffectorValidatorType {
	ConditionValidator(ConditionValidator),
	TimeValidator(TimeValidator),
	Always
}

#[derive(Clone)]
pub struct ConditionValidator {
	pub start: f32,
	pub end: f32,
	pub condition: Vec<PositionCondition>
}

impl ConditionValidator {
	pub fn new(start: f32, end: f32, condition: Vec<PositionCondition>) -> Self {
		ConditionValidator {
			start,
			end,
			condition
		}
	}
}

#[derive(Clone)]
pub struct TimeValidator {
	pub start: f32,
	pub end: f32
}

impl TimeValidator {
	pub fn new(start: f32, end: f32) -> Self {
		TimeValidator {
			start,
			end
		}
	}
}