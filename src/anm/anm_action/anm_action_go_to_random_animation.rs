use bytebuffer::ByteBuffer;
use anyhow::{Result, anyhow};
use super::anm_action::AnmActionTrait;
use crate::utils::utils::read_string_without_len;

#[derive(Clone)]
pub struct AnmActionGoToRandomAnimation {
	_animation_names: Vec<String>,
	_percents: Vec<i8>
}

impl AnmActionTrait for AnmActionGoToRandomAnimation {
	fn load(parameters_count: i8, buffer: &mut ByteBuffer) -> Result<Self> {
		if parameters_count == 0 {
			return Ok(AnmActionGoToRandomAnimation::default());
		}
		let first = read_string_without_len(buffer)?;
		if first.eq("#optimized") {
			let count: i32 = ((parameters_count - 1) / 2) as i32;
			let mut _animation_names = Vec::with_capacity(count as usize);
			for _ in 0..count {
				_animation_names.push(read_string_without_len(buffer)?);
			}
			let mut _percents = Vec::with_capacity(count as usize);
			for _ in 0..count {
				_percents.push(buffer.read_i8()?);
			}
			let result = AnmActionGoToRandomAnimation {
				_animation_names,
				_percents
			};
			return Ok(result);
		}
		let mut parameters = Vec::with_capacity(parameters_count as usize);
		parameters.push(first);
		for _ in 1..parameters_count {
			parameters.push(read_string_without_len(buffer)?);
		}

		if let Ok(result) = AnmActionGoToRandomAnimation::fill_animations(&parameters) {
			return Ok(result);
		}

		let result = AnmActionGoToRandomAnimation::default();

		Ok(result)
	}

	fn get_type(&self) -> &'static str {
		"GO_TO_RANDOM_ANIMATION"
	}
}

impl Default for AnmActionGoToRandomAnimation {
	fn default() -> Self {
		AnmActionGoToRandomAnimation {
			_animation_names: Vec::new(),
			_percents: Vec::new()
		}
	}
}

impl AnmActionGoToRandomAnimation {
	pub fn fill_animations(parameters: &Vec<String>) -> Result<AnmActionGoToRandomAnimation> {
		let mut anim_read = false;

		let mut anims = Vec::new();
		let mut values = Vec::with_capacity(parameters.len());

		let mut last: i32 = -1;
		for i in 0..parameters.len() {
			let param = parameters.get(i).unwrap();
			let mut steps = || -> Result<()> {
				let percent = param.trim().parse::<i8>()?;
				if anim_read {
					if values.len() > last as usize {
						values.insert(last as usize, values.get(last as usize).unwrap() + percent)
					}
					else {
						for _ in values.len()..anims.len() {
							values.push(0);
						}
						values.push(percent);
					}
					anim_read = false;
				}
				Ok(())
			};
			if let Err(_) = steps() {
				anim_read = true;
				last = anims.iter().position(|r: &String| r.eq(param)).unwrap() as i32;
				if last == -1 {
					last = anims.len() as i32;
					anims.push(param.clone());
				}
			}
		}

		let mut remain: i32 = 100;
		for i in 0..values.len() {
			remain -= *(values.get(i).unwrap()) as i32;
		}


		let r: i32 = anims.len() as i32 - values.len() as i32;
		if r == 0 {
			let v: i8 = (remain / values.len() as i32) as i8;
			let count: i32 = remain - v as i32 * values.len() as i32;
			for i in 0..count {
				values.insert(i as usize, values.get(i as usize).unwrap() + v + 1);
			}
			for i in count..values.len() as i32 {
				values.insert(i as usize, values.get(i as usize).unwrap() + v);
			}
		}
		else {
			let v: i8 = (remain / r) as i8;
			let count: i32 = remain - v as i32 * r;
			for _ in 0..count {
				values.push(v + 1);
			}
			for _ in count..r {
				values.push(v);
			}
		}

		for i in (0..(values.len() - 1)).rev() {
			if *values.get(i).unwrap() == 0 {
				values.remove(i);
				anims.remove(i);
			}
		}

		let mut total: i32 = 0;
		for i in 0..values.len() {
			total += *values.get(i).unwrap() as i32;
		}
		if total != 100 {
			return Err(anyhow!("Bad total percentage"));
		}

		let result = AnmActionGoToRandomAnimation {
			_animation_names: anims,
			_percents: values
		};
		Ok(result)
	}
}