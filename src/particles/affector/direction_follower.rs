use std::{collections::HashMap};

use crate::particles::particle::Particle;

#[derive(Clone)]
pub struct DirectionFollower {
	angle: HashMap<String, f32>
}

impl DirectionFollower {
	pub fn load() -> Self {
		DirectionFollower {
			angle: HashMap::new()
		}
	}

	pub fn affect(&mut self, parent: &*const Particle, particle: &mut Particle) {
		let p = Particle::get_parent(parent);
		if !p.geocentric {
			return ;
		}

		let x = particle.x + p.get_x();
		let y = particle.y + p.get_y();
		let z = particle.z + p.get_z();

		println!("WARNING: DirectionFollower affector is called and result might be wrong");
		
		if particle.last_x.ne(&f32::MAX) {
			let dx = x - particle.last_x;
			let dy = y - particle.last_y;
			let dz = z - particle.last_z;
			if dx.eq(&0.) && dy.eq(&0.) && dz.eq(&0.) {
				return ;
			}
			let rx = (dx - dy) / 4.;
			let ry = (dx + dy) * 0.5 + dz * 0.116279066;
			if rx.abs().gt(&1.0e-5) {
				let angle = (ry / rx).atan();
				let particle_address = format!("{:p}", particle);
				if let Some(last_angle) = self.angle.get(&particle_address) {
					particle.angle += angle - last_angle;
					self.angle.insert(particle_address, angle);
				}
			}
		}
		particle.last_x = x;
		particle.last_y = y;
		particle.last_z = z;
	}
}