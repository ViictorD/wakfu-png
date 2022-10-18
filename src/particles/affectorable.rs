use super::affector::affector::Affector;

#[derive(Clone)]
pub struct Affectorable {
	pub affectors: Vec<Affector>,
	pub key_framed_affectors: Vec<Affector>
}

impl Affectorable {
	pub fn new() -> Self {
		Affectorable {
			affectors: Vec::new(),
			key_framed_affectors: Vec::new()
		}
	}

	pub fn add_affector(&mut self, affector: Affector) {
		self.affectors.push(affector);
	}

	pub fn add_key_framed_affector(&mut self, affector: Affector) {
		self.key_framed_affectors.push(affector);
	}

	pub fn has_key_framed_affector(&self) -> bool {
		self.key_framed_affectors.len() > 0
	}
}