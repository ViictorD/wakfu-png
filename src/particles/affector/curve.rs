use bytebuffer::ByteBuffer;
use glam::Vec3;

use crate::{particles::{attributes_reader_writer::AttributesReaderWriter}, particles::particle::Particle};

#[derive(Clone)]
pub struct CubicSplineTrajectory {
	initial_position: Vec3,
	initial_velocity: Vec3,
	final_position: Vec3,
	final_velocity: Vec3,
	a: f32,
	b: f32,
	c: f32,
	d: f32,
	e: f32,
	f: f32,
	g: f32,
	h: f32,
	i: f32,
	j: f32,
	k: f32,
	l: f32,
	vectors_changed: bool,
	initial_time: i64,
	final_time: i64
}

impl CubicSplineTrajectory {
	pub fn new() -> Self {
		CubicSplineTrajectory {
			initial_position: Vec3::default(),
			initial_velocity: Vec3::default(),
			final_position: Vec3::default(),
			final_velocity: Vec3::default(),
			a: 0.,
			b: 0.,
			c: 0.,
			d: 0.,
			e: 0.,
			f: 0.,
			g: 0.,
			h: 0.,
			i: 0.,
			j: 0.,
			k: 0.,
			l: 0.,
			vectors_changed: true,
			initial_time: 0,
			final_time: 0
		}
	}

	pub fn set_initial_time(&mut self, time: i64) {
		self.initial_time = time;
	}

	pub fn set_final_time(&mut self, time: i64) {
		self.final_time = time;
	}

	pub fn set_initial_position(&mut self, position: Vec3) {
		self.initial_position = position;
		self.vectors_changed = true;
	}

	pub fn set_initial_velocity(&mut self, velocity: Vec3) {
		self.initial_velocity = velocity;
		self.vectors_changed = true;
	}

	pub fn set_final_position(&mut self, position: Vec3) {
		self.final_position = position;
		self.vectors_changed = true;
	}

	pub fn set_final_velocity(&mut self, velocity: Vec3) {
		self.final_velocity = velocity;
		self.vectors_changed = true;
	}

	fn compute_factors(&mut self) {
		let x0 = self.initial_position.x;
		let y0 = self.initial_position.y;
		let z0 = self.initial_position.z;
		let x = x0 + self.initial_velocity.x * 1.;
		let y = y0 + self.initial_velocity.y * 1.;
		let z = z0 + self.initial_velocity.z * 1.;
		let x2 = self.final_position.x;
		let y2 = self.final_position.y;
		let z2 = self.final_position.z;
		let x3 = x2 - self.final_velocity.x * 1.;
		let y3 = y2 - self.final_velocity.y * 1.;
		let z3 = z2 - self.final_velocity.z * 1.;
		self.a = x2 - 3. * x3 + 3. * x - x0;
		self.b = 3. * x3 - 6. * x + 3. * x0;
		self.c = 3. * x - 3. * x0;
		self.d = x0;
		self.e = y2 - 3. * y3 + 3. * y - y0;
		self.f = 3. * y3 - 6. * y + 3. * y0;
		self.g = 3. * y - 3. * y0;
		self.h = y0;
		self.i = z2 - 3. * z3 + 3. * z - z0;
		self.j = 3. * z3 - 6. * z + 3. * z0;
		self.k = 3. * z - 3. * z0;
		self.l = z0;
		self.vectors_changed = false;
	}

	pub fn get_position(&mut self, mut time: i64) -> Vec3 {
		if self.vectors_changed {
			self.compute_factors();
		}
		if time < self.initial_time {
			time = self.initial_time;
		}
		else if time > self.final_time {
			time = self.final_time;
		}

		let t = (time - self.initial_time) as f32 / (self.final_time - self.initial_time) as f32;
		let t2 = t * t;
		let t3 = t2 * t;
		Vec3::new(
			self.a * t3 + self.b * t2 + self.c * t + self.d,
			self.e * t3 + self.f * t2 + self.g * t + self.h,
			self.i * t3 + self.j * t2 + self.k * t + self.l
		)
	}
}

#[derive(Clone)]
pub struct Curve {
	trajectory: CubicSplineTrajectory,
}

impl Curve {
	pub fn load(buffer: &mut ByteBuffer, level_percent: &f32) -> Self {
		let start_pos = Self::read_vector_3(buffer, level_percent);
		let start_velocity = Self::read_vector_3(buffer, level_percent);
		let end_pos = Self::read_vector_3(buffer, level_percent);
		let end_velocity = Self::read_vector_3(buffer, level_percent);

		let mut t = CubicSplineTrajectory::new();
		t.set_initial_time(0);
		t.set_final_time(1000000);
		t.set_initial_position(start_pos);
		t.set_initial_velocity(start_velocity);
		t.set_final_position(end_pos);
		t.set_final_velocity(end_velocity);

		Curve {
			trajectory: t
		}
	}

	fn read_vector_3(buffer: &mut ByteBuffer, level_percent: &f32) -> Vec3 {
		let x = AttributesReaderWriter::read_float(buffer, level_percent);
		let y = AttributesReaderWriter::read_float(buffer, level_percent);
		let z = AttributesReaderWriter::read_float(buffer, level_percent);
		Vec3::new(x, y, z)
	}

	pub fn affect(&mut self, parent: &*const Particle, particle: &mut Particle) {
		let t = (1000000. * (particle.life / particle.life_time)) as i64;
		let position = self.trajectory.get_position(t);

		let p = Particle::get_parent(parent);
		if p.geocentric {
			panic!("Not implemented yet");
			// particle.x = position.x + particle_system.get_x();
			// particle.y = position.y + particle_system.get_y();
			// particle.z = position.z + particle_system.get_z();
		}
		else {
			particle.x = position.x + p.x;
			particle.y = position.y + p.y;
			particle.z = position.z + p.z;
		}
	}
}