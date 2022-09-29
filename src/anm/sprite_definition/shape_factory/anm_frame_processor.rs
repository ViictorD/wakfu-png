use crate::anm::{
	sprite_definition::anm_frame_data::DataContainer,
	anm::AnmTransformDataTable,
	processing::anm_transform::AnmTransform
};

pub fn read_and_process(
	fp_type: i32,
	frame_data: &mut DataContainer,
	table: &AnmTransformDataTable,
	parent: &AnmTransform,
	result: &mut AnmTransform
) {
	match fp_type {
		3 => {
			no_color(parent, result);
			process_rotation_translation(parent, result, &table.rotations, frame_data.read(), &table.translations, frame_data.read())
		},
		2 => {
			no_color(parent, result);
			no_rotation(parent, result);
			process_translation(parent, result, &table.translations, frame_data.read());
		},
		0 => {
			no_color(parent, result);
			no_rotation(parent, result);
			no_translation(parent, result);
		},
		8 => {
			process_color_add(parent, result, &table.colors, frame_data.read());
			no_rotation(parent, result);
			no_translation(parent, result);
		},
		12 => {
			process_color_mult_add(parent, result, &table.colors, frame_data.read(), frame_data.read());
			no_rotation(parent, result);
			no_translation(parent, result);
		},
		4 => {
			process_color_mult(parent, result, &table.colors, frame_data.read());
			no_rotation(parent, result);
			no_translation(parent, result);
		},
		1 => {
			no_color(parent, result);
			process_rotation(parent, result, &table.rotations, frame_data.read());
			no_translation(parent, result);
		},
		9 => {
			process_color_add(parent, result, &table.colors, frame_data.read());
			process_rotation(parent, result, &table.rotations, frame_data.read());
			no_translation(parent, result);
		},
		13 => {
			process_color_mult_add(parent, result, &table.colors, frame_data.read(), frame_data.read());
			process_rotation(parent, result, &table.rotations, frame_data.read());
			no_translation(parent, result);
		},
		5 => {
			process_color_mult(parent, result, &table.colors, frame_data.read());
			process_rotation(parent, result, &table.rotations, frame_data.read());
			no_translation(parent, result);
		},
		11 => {
			process_color_add(parent, result, &table.colors, frame_data.read());
			process_rotation_translation(parent, result, &table.rotations, frame_data.read(), &table.translations, frame_data.read())
		},
		15 => {
			process_color_mult_add(parent, result, &table.colors, frame_data.read(), frame_data.read());
			process_rotation_translation(parent, result, &table.rotations, frame_data.read(), &table.translations, frame_data.read())
		},
		7 => {
			process_color_mult(parent, result, &table.colors, frame_data.read());
			process_rotation_translation(parent, result, &table.rotations, frame_data.read(), &table.translations, frame_data.read())
		},
		10 => {
			process_color_add(parent, result, &table.colors, frame_data.read());
			no_rotation(parent, result);
			process_translation(parent, result, &table.translations, frame_data.read());
		},
		14 => {
			process_color_mult_add(parent, result, &table.colors, frame_data.read(), frame_data.read());
			no_rotation(parent, result);
			process_translation(parent, result, &table.translations, frame_data.read());
		},
		6 => {
			process_color_mult(parent, result, &table.colors, frame_data.read());
			no_rotation(parent, result);
			process_translation(parent, result, &table.translations, frame_data.read());
		},
		_ => {}
	}
}

fn no_color(parent: &AnmTransform, result: &mut AnmTransform) {
	result.red = parent.red;
	result.green = parent.green;
	result.blue = parent.blue;
	result.alpha = parent.alpha;
}

fn no_translation(parent: &AnmTransform, result: &mut AnmTransform) {
	result.translation_is_identity = parent.translation_is_identity;
	result.translation_x = parent.translation_x;
	result.translation_y = parent.translation_y;
}

fn no_rotation(parent: &AnmTransform, result: &mut AnmTransform) {
	result.rotation_is_identity = parent.rotation_is_identity;
	result.rotation_skew_x0 = parent.rotation_skew_x0;
	result.rotation_skew_x1 = parent.rotation_skew_x1;
	result.rotation_skew_y0 = parent.rotation_skew_y0;
	result.rotation_skew_y1 = parent.rotation_skew_y1;
}

fn process_color_add(parent: &AnmTransform, result: &mut AnmTransform, colors: &Vec<f32>, offset: i32) {
	result.red = parent.red + colors.get(offset as usize).unwrap();
	result.green = parent.green + colors.get(offset as usize + 1).unwrap();
	result.blue = parent.blue + colors.get(offset as usize + 2).unwrap();
	result.alpha = parent.alpha + colors.get(offset as usize + 3).unwrap();
}

fn process_color_mult(parent: &AnmTransform, result: &mut AnmTransform, colors: &Vec<f32>, offset: i32) {
	result.red = parent.red * colors.get(offset as usize).unwrap();
	result.green = parent.green * colors.get(offset as usize + 1).unwrap();
	result.blue = parent.blue * colors.get(offset as usize + 2).unwrap();
	result.alpha = parent.alpha * colors.get(offset as usize + 3).unwrap();
}

fn process_color_mult_add(parent: &AnmTransform, result: &mut AnmTransform, colors: &Vec<f32>, offset_a: i32, offset_m: i32) {
	result.red = parent.red * colors.get(offset_m as usize).unwrap() + colors.get(offset_a as usize).unwrap();
	result.green = parent.green * colors.get(offset_m as usize + 1).unwrap() + colors.get(offset_a as usize + 1).unwrap();
	result.blue = parent.blue * colors.get(offset_m as usize + 2).unwrap() + colors.get(offset_a as usize + 2).unwrap();
	result.alpha = parent.alpha * colors.get(offset_m as usize + 3).unwrap() + colors.get(offset_a as usize + 3).unwrap();
}

fn process_rotation(parent: &AnmTransform, result: &mut AnmTransform, rotations: &Vec<f32>, offset: i32) {
	result.rotation_is_identity = false;

	let rx0 = rotations.get(offset as usize).unwrap();
	let ry0 = rotations.get(offset as usize + 1).unwrap();
	let rx = rotations.get(offset as usize + 2).unwrap();
	let ry = rotations.get(offset as usize + 3).unwrap();
	if parent.rotation_is_identity {
		result.rotation_skew_x0 = *rx0;
		result.rotation_skew_y0 = *ry0;
		result.rotation_skew_x1 = *rx;
		result.rotation_skew_y1 = *ry;
	}
	else {
		result.rotation_skew_x0 = rx0 * parent.rotation_skew_x0 + ry0 * parent.rotation_skew_x1;
		result.rotation_skew_y0 = rx0 * parent.rotation_skew_y0 + ry0 * parent.rotation_skew_y1;
		result.rotation_skew_x1 = rx * parent.rotation_skew_x0 + ry * parent.rotation_skew_x1;
		result.rotation_skew_y1 = rx * parent.rotation_skew_y0 + ry * parent.rotation_skew_y1;
	}
}

fn process_translation(parent: &AnmTransform, result: &mut AnmTransform, translations: &Vec<f32>, offset: i32) {
	result.translation_is_identity = false;
	let tx = translations.get(offset as usize).unwrap();
	let ty = translations.get(offset as usize + 1).unwrap();
	if parent.rotation_is_identity {
		result.translation_x = tx + parent.translation_x;
		result.translation_y = ty + parent.translation_y;
	}
	else {
		result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
		result.translation_y = tx * parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
	}
}

fn process_rotation_translation(
	parent: &AnmTransform,
	result: &mut AnmTransform,
	rotations: &Vec<f32>,
	offset_r: i32,
	translations: &Vec<f32>,
	offset_t: i32
) {
	result.rotation_is_identity = false;
	result.translation_is_identity = false;


	let rx0 = rotations.get(offset_r as usize).unwrap();
	let ry0 = rotations.get(offset_r as usize + 1).unwrap();
	let rx = rotations.get(offset_r as usize + 2).unwrap();
	let ry = rotations.get(offset_r as usize + 3).unwrap();
	let tx = translations.get(offset_t as usize).unwrap();
	let ty = translations.get(offset_t as usize + 1).unwrap();
	if parent.rotation_is_identity {

		result.rotation_skew_x0 = *rx0;
		result.rotation_skew_y0 = *ry0;
		result.rotation_skew_x1 = *rx;
		result.rotation_skew_y1 = *ry;
		result.translation_x = tx + parent.translation_x;
		result.translation_y = ty + parent.translation_y;
	}
	else {
		result.rotation_skew_x0 = rx0 * parent.rotation_skew_x0 + ry0 * parent.rotation_skew_x1;
		result.rotation_skew_y0 = rx0 * parent.rotation_skew_y0 + ry0 * parent.rotation_skew_y1;
		result.rotation_skew_x1 = rx * parent.rotation_skew_x0 + ry * parent.rotation_skew_x1;
		result.rotation_skew_y1 = rx * parent.rotation_skew_y0 + ry * parent.rotation_skew_y1;
		result.translation_x = tx * parent.rotation_skew_x0 + ty * parent.rotation_skew_x1 + parent.translation_x;
		result.translation_y = tx * parent.rotation_skew_y0 + ty * parent.rotation_skew_y1 + parent.translation_y;
	}
}