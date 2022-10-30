use std::collections::HashMap;

use crate::tplg::Tplg;

use super::{Map, groups::Groups};

pub struct LayerManager;

impl LayerManager {
	fn get_map_groups(map:&Map) -> Vec<i32> {
		let mut group_ids = Vec::new();
		// Collect all group id from the map
		for chunk in map.chunks() {
			for sprite in chunk.sprites() {
				if sprite.group_id != 0 && !group_ids.contains(&sprite.group_id) {
					group_ids.push(sprite.group_id);
				}
			}
		}
		group_ids
	}

	pub fn get_outdoor_visible_layers(map:&Map, groups: &Groups, tplg: &Tplg) -> Vec<i32> {
		let group_ids = Self::get_map_groups(map);
		
		// Collect visible outside layer in inside render
		let mut visible_layers: Vec<i32> = Vec::new();
		for group_id in group_ids {
			let mut positive_group_keys = Vec::new();
			let mut negative_group_keys = Vec::new();
		
			for chunk in map.chunks() {
				for sprite in chunk.sprites() {
					if sprite.group_id == group_id {
						if sprite.group_key < 0 && !negative_group_keys.contains(&sprite.group_key) {
							negative_group_keys.push(sprite.group_key);
						}
						else if sprite.group_key > 0 && !positive_group_keys.contains(&sprite.group_key) {
							positive_group_keys.push(sprite.group_key);
						}
					}
				}
			}
			// We check for every layers in the group, the ones that show the less outside layers,
			// and that are walkable. The resut layer(s) should be the floor of the inside.
			// From this result, we can find out wich outside layers to display
			let mut exterior_layers_counter: HashMap<i32, u16> = HashMap::new();
			for chunk in map.chunks() {
				for sprite in chunk.sprites() {
					if sprite.group_id == group_id {
						if sprite.group_key != 0 {
							if let Some(layer) = exterior_layers_counter.get(&sprite.group_key) {
								if *layer > 0 {
									continue ;
								}
							}
							for negative_key in &negative_group_keys {
								let tmp =
									if groups.is_layer_visible(sprite.group_key, *negative_key)
										&& !tplg.is_blocked(sprite.cell_x, sprite.cell_y, sprite.cell_z as i32, sprite.height) { true }
									else { false };
								if !exterior_layers_counter.contains_key(&sprite.group_key) {
									exterior_layers_counter.insert(sprite.group_key, if tmp { 1 } else { 0 });
								}
								else {
									*exterior_layers_counter.get_mut(&sprite.group_key).unwrap() += if tmp { 1 } else { 0 };
								}
							}
						}
					}
				}
			}
			let mut min = (0, u16::MAX);
			for (key, value) in &exterior_layers_counter {
				if *value == 0 {
					continue ;
				}
				if *value < min.1 {
					min = (*key, *value);
				}
			}

			// This should get one of the 2 entry floor where we see inside and outside
			let mut min_plus_one = (0, u16::MAX);
			for (key, value) in exterior_layers_counter {
				if value > min.1 && value < min_plus_one.1 {
					min_plus_one = (key, value);
				}
			}
			if min_plus_one.1 < negative_group_keys.len() as u16 {
				min = min_plus_one;
			}
			for negative_key in &negative_group_keys {
				if visible_layers.contains(&negative_key) {
					continue ;
				}
				if groups.is_layer_visible(min.0, *negative_key) {
					visible_layers.push(*negative_key);
				}
			}

		}
		visible_layers
	}
}
