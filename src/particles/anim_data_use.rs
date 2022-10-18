use bytebuffer::ByteBuffer;

use crate::utils::math_helper::MathHelper;

#[derive(Clone)]
pub struct AnimDataUse {
	pub total_time: i32,
	animation_times: Vec<i16>,
	animation_texture_coords: Vec<[f32; 4]>
}

impl AnimDataUse {
	pub fn load(buffer: &mut ByteBuffer) -> Option<Self> {
		let anim_count = buffer.read_u8().unwrap();
		if anim_count == 0 {
			return None;
		}

		let total_time = buffer.read_i32().unwrap();
		let img_width = buffer.read_i16().unwrap();
		let img_height =  buffer.read_i16().unwrap();
		let img_width_total =  buffer.read_i16().unwrap();
		let img_height_total =  buffer.read_i16().unwrap();

		let mut animation_times = Vec::with_capacity(anim_count as usize);
		for _ in 0..anim_count {
			animation_times.push(buffer.read_i16().unwrap());
		}
		let mut texture_coords = Vec::with_capacity(anim_count as usize * 2);
		for _ in 0..anim_count * 2 {
			texture_coords.push(buffer.read_i16().unwrap());
		}

		let width_total = MathHelper::nearest_greatest_pow_of_two(img_width_total as i32) as f32;
		let height_total = MathHelper::nearest_greatest_pow_of_two(img_height_total as i32) as f32 - 0.5;
		let right = img_width as f32 / width_total;
		let bottom = img_height as f32 / height_total;

		let animation_texture_coords_count = texture_coords.len() / 2;
		let mut animation_texture_coords = Vec::with_capacity(animation_texture_coords_count);
		for i in 0..animation_texture_coords_count {
			let offset_x = (texture_coords[i * 2] as f32 + 0.5) / width_total;
			let offset_y = (texture_coords[i * 2 + 1] as f32 + 0.5) / height_total;
			animation_texture_coords.push([
				right + offset_x,
				bottom + offset_y,
				offset_x,
				offset_y
			])
		}

		let result = AnimDataUse {
			total_time,
			animation_times,
			animation_texture_coords
		};
		Some(result)
	}

	pub fn get_texture_coodrinates(&self, animation_time: u16) -> [f32; 4] {
		let mut time = animation_time as i32 % self.total_time;
		for i in 0..self.animation_times.len() {
			time -= *self.animation_times.get(i).unwrap() as i32;
			if time < 0 {
				return *self.animation_texture_coords.get(i).unwrap();
			}
		}
		*self.animation_texture_coords.get(0).unwrap()
	}
}