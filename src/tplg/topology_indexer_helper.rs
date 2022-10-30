use crate::utils::math_helper::MathHelper;

pub struct TopologyIndexerHelper;

impl TopologyIndexerHelper {
	fn data_by_long(nb_bits_by_data: usize) -> usize {
		64 / nb_bits_by_data
	}

	fn data_by_int(nb_bits_by_data: usize) -> usize {
		32 / nb_bits_by_data
	}

	fn get_mask(nb_bits: usize) -> i32 {
		(1 << nb_bits) - 1
	}

	pub fn get_index_i64(indexes: &Vec<i64>, index: usize, table_size: usize) -> usize {
		let nb_bits = MathHelper::log2i(table_size);
		let data_count = Self::data_by_long(nb_bits);
		let mask = Self::get_mask(nb_bits) as i64;
		let mut i = *indexes.get(index / data_count).unwrap();
		i >>= nb_bits * (index % data_count);
		(i & mask) as usize
	}

	pub fn get_index_i32(indexes: &Vec<i32>, index: usize, table_size: usize) -> usize {
		let nb_bits = MathHelper::log2i(table_size);
		let data_count = Self::data_by_int(nb_bits);
		let mask = Self::get_mask(nb_bits);
		let mut i = *indexes.get(index / data_count).unwrap();
		i >>= nb_bits * (index % data_count);
		(i & mask) as usize
	}
}