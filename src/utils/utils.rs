use anyhow::{Result};
use bytebuffer::ByteBuffer;

pub fn read_string_without_len(buffer: &mut ByteBuffer) -> Result<String> {
	let mut vec_str = Vec::new();
	for _ in 0..buffer.len() - buffer.get_rpos() {
		let byte = buffer.read_u8()?;
		if byte == 0 {
			break ;
		}
		vec_str.push(byte);
	}
	Ok(String::from_utf8(vec_str)?)
}

pub fn java_string_hashcode(str: &String) -> i32 {
	let bytes_str = str.as_bytes();
	let mut result = 0i32;
	for i in 0..bytes_str.len() {
		let pow = 31i32.overflowing_pow((bytes_str.len() - (i + 1)) as u32).0;
		let mul = (bytes_str[i] as i32).overflowing_mul(pow).0;
		result = result.overflowing_add(mul).0;
	}
	result
}