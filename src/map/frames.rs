use image::math::Rect;
use byte::ctx::Endian;
use byte::{BytesExt, TryRead};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Frames {
	pub _total_time: u32,
	pub _frame_times: Vec<u16>,
	pub _frame_rects: Vec<Rect>,
}

impl Frames {
	pub fn new(_total_time: u32, frame_durations: &[u16], frame_coords: Vec<Rect>) -> Self {
		let mut _frame_times = Vec::with_capacity(frame_durations.len());
		let mut _frame_time = 0;
		for dur in frame_durations {
			_frame_times.push(_frame_time);
			_frame_time += dur;
		}

		Self {
			_total_time,
			_frame_times,
			_frame_rects: frame_coords,
		}
	}
}

impl<'a> TryRead<'a, u8> for Frames {
	fn try_read(bytes: &'a [u8], count: u8) -> byte::Result<(Self, usize)> {
		let offset = &mut 0;

		let total_time: u32 = bytes.read(offset)?;
		let width: u16 = bytes.read(offset)?;
		let height: u16 = bytes.read(offset)?;
		let _width_total: u16 = bytes.read(offset)?;
		let _height_total: u16 = bytes.read(offset)?;
		let frame_durations: Vec<u16> = bytes
			.read_iter(offset, Endian::default())
			.take(count.into())
			.collect();
		let coords = bytes
			.read_iter::<i16>(offset, Endian::default())
			.take(count as usize * 2)
			.tuples()
			.map(|(x, y)| Rect {
				x: x as u32,
				y: y as u32,
				width: (x as f32 + width as f32) as u32,
				height: (y as f32 + height as f32) as u32
			})
			.collect_vec();

		let result = Frames::new(total_time, &frame_durations, coords);
		Ok((result, *offset))
	}
}
