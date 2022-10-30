pub struct ByteArrayBitSet {
	bits: Vec<u8>
}

impl ByteArrayBitSet {
	pub fn new(bits: Vec<u8>) -> Self {
		ByteArrayBitSet {
			bits
		}
	}

	pub fn new_empty(size: usize) -> Self {
		ByteArrayBitSet {
			bits: vec![0; size]
		}
	}

	fn bit(index: u32) -> u8 {
		1 << index
	}

	pub fn insert(&mut self, index: u32, value: bool) {
		let unit_position = index >> 3;
		let bit_position = 7 - (index - (unit_position << 3));
		let unit = self.bits.get_mut(unit_position as usize).unwrap();
		if value {
			*unit |= Self::bit(bit_position);
		}
		else {
			*unit &= !Self::bit(bit_position);
		}
	}

	pub fn get(&self, index: u32) -> bool {
		let unit_position = index >> 3;
		let bit_position = 7 - (index - (unit_position << 3));
		(self.bits.get(unit_position as usize).unwrap() & Self::bit(bit_position)) != 0
	}

	pub fn get_data_length(len: usize) -> usize {
		(len + 7) >> 3
	}
}