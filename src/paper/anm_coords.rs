
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use glam::Vec2;

pub struct Coord {
	pub id: i32,
	pub coord: Vec2
}

// I couldn't figure it out how to get the elements placement coords, so i do it in the ugly way for now.
// The startX, startY... infos are not usable for placing the elements. It's just for the mouse over.
pub struct FullMapCoord {
	pub coords: HashMap<i32, Vec<Coord>>
}

impl FullMapCoord {
	pub fn new() -> Self {
		let mut coords = HashMap::new();

		// -1
		let mut tmp = Vec::with_capacity(5);
		// cycle wakfu/stasis
		tmp.push(Coord { id: -1, coord: Vec2::new(50., 50.) });
		// Monde des 12
		tmp.push(Coord { id: -2, coord: Vec2::new(50., 50.) });
		// Ingloriom
		tmp.push(Coord { id: -3, coord: Vec2::new(45., 18.) });
		// Shushu
		tmp.push(Coord { id: -6, coord: Vec2::new(50., 84.) });
		// Incarnam
		tmp.push(Coord { id: 1076, coord: Vec2::new(45., 6.) });
		coords.insert(-1, tmp);

		// -2
		tmp = Vec::with_capacity(24);
		// Amakna
		tmp.push(Coord { id: 1134, coord: Vec2::new(73., 42.) });
		// Bonta
		tmp.push(Coord { id: 1136, coord: Vec2::new(32., 22.) });
		// Brakmar
		tmp.push(Coord { id: 1137, coord: Vec2::new(42., 84.) });
		// Sufokia
		tmp.push(Coord { id: 1135, coord: Vec2::new(72., 72.) });
		// Ereboria
		tmp.push(Coord { id: -7, coord: Vec2::new(82., 53.) });
		// Katrepat
		tmp.push(Coord { id: 327, coord: Vec2::new(47., 61.) });
		// Royaume Sadida
		tmp.push(Coord { id: 1264, coord: Vec2::new(44., 33.) });
		// Ile aux moines
		tmp.push(Coord { id: 323, coord: Vec2::new(57., 80.) });
		// Domaine sauvage
		tmp.push(Coord { id: 711, coord: Vec2::new(68., 53.) });
		// L'ile wabbit
		tmp.push(Coord { id: 184, coord: Vec2::new(86., 39.) });
		// L'ile des chuchoteurs
		tmp.push(Coord { id: 1295, coord: Vec2::new(53., 48.) });
		// Pandalousie
		tmp.push(Coord { id: 1252, coord: Vec2::new(79., 22.) });
		// Saharash
		tmp.push(Coord { id: 792, coord: Vec2::new(90., 28.) });
		// Shushus
		tmp.push(Coord { id: 794, coord: Vec2::new(32., 76.) });
		// Astrub
		tmp.push(Coord { id: 527, coord: Vec2::new(68., 18.) });
		// Kelba
		tmp.push(Coord { id: 943, coord: Vec2::new(69., 27.) });
		// Foir du trool
		tmp.push(Coord { id: 337, coord: Vec2::new(51., 23.) });
		// Frigost
		tmp.push(Coord { id: 504, coord: Vec2::new(14., 12.) });
		// Bilbiza
		tmp.push(Coord { id: 458, coord: Vec2::new(65., 85.) });
		// Zinit
		tmp.push(Coord { id: -5, coord: Vec2::new(22., 49.) });
		// Tour de la Fratrie / Tour Minérale
		tmp.push(Coord { id: 1201, coord: Vec2::new(18., 34.) });
		// Moon
		tmp.push(Coord { id: 999, coord: Vec2::new(89., 48.) });
		// Ecole des Huppermages
		tmp.push(Coord { id: 1017, coord: Vec2::new(40., 14.) });
		// Bbliotemple
		tmp.push(Coord { id: 1089, coord: Vec2::new(55., 34.) });
		coords.insert(-2, tmp);

		// -3
		tmp = Vec::with_capacity(8);
		// Cloud
		tmp.push(Coord { id: -1, coord: Vec2::new(50., 50.) });
		// Dimension Enu
		tmp.push(Coord { id: 853, coord: Vec2::new(40., 69.) });
		// Dimension Sram
		tmp.push(Coord { id: 843, coord: Vec2::new(30., 66.) });
		// Dimension Xelor passe
		tmp.push(Coord { id: 922, coord: Vec2::new(81., 61.) });
		// Dimension Xelor Present
		tmp.push(Coord { id: 923, coord: Vec2::new(74., 54.) });
		// Ingloriom
		tmp.push(Coord { id: 856, coord: Vec2::new(50., 50.) });
		// Dimension Eca
		tmp.push(Coord { id: 967, coord: Vec2::new(34., 16.) });
		// Dimension Osamodas
		tmp.push(Coord { id: 1276, coord: Vec2::new(16., 34.) });
		coords.insert(-3, tmp);

		// -5
		tmp = Vec::with_capacity(8);
		// Mont Zinit
		tmp.push(Coord { id: 0, coord: Vec2::new(30., 30.) });
		// Plage du Zinit
		tmp.push(Coord { id: 986, coord: Vec2::new(60., 78.) });
		// Plage sauvage
		tmp.push(Coord { id: 1118, coord: Vec2::new(77., 56.) });
		// Grotte Slek
		tmp.push(Coord { id: 1119, coord: Vec2::new(62., 46.) });
		// Bas Flancs
		tmp.push(Coord { id: 1120, coord: Vec2::new(40., 47.) });
		// Grotte du Dor'Mor
		tmp.push(Coord { id: 1123, coord: Vec2::new(18., 12.) });
		// Haut-Flancs
		tmp.push(Coord { id: 1122, coord: Vec2::new(27., 21.) });
		// Sommet
		tmp.push(Coord { id: 1124, coord: Vec2::new(3., 6.) });
		coords.insert(-5, tmp);
		
	
		// -6
		tmp = Vec::with_capacity(9);
		// Cloud
		tmp.push(Coord { id: -1, coord: Vec2::new(50., 50.) });
		// Croisée des ames déchues
		tmp.push(Coord { id: 1342, coord: Vec2::new(45., 41.) });
		// Zone 0 - Territoir de sthulhu
		tmp.push(Coord { id: 793, coord: Vec2::new(48., 9.) });
		// Zone 1 - Royaume du désespoir
		tmp.push(Coord { id: 1305, coord: Vec2::new(82., 38.) });
		// Zone 2 - Spirale du vide
		tmp.push(Coord { id: 1309, coord: Vec2::new(47., 74.) });
		// Zone 3 - Citadelle de l'horreur
		tmp.push(Coord { id: 1326, coord: Vec2::new(17., 62.) });
		// Zone 4 - Dementia
		tmp.push(Coord { id: 1333, coord: Vec2::new(63., 33.) });
		// Zone 5 - Route des morts
		tmp.push(Coord { id: 1338, coord: Vec2::new(75., 68.) });
		// Zone 6 - Palais de rushu
		tmp.push(Coord { id: 1332, coord: Vec2::new(18., 15.) });
		coords.insert(-6, tmp);

		tmp = Vec::with_capacity(7);
		// Volcano smoke
		tmp.push(Coord { id: -1, coord: Vec2::new(30., 28.) });
		// Hub - Port d'Ereboria
		tmp.push(Coord { id: 1360, coord: Vec2::new(60., 61.5) });
		// Plage des pirates
		tmp.push(Coord { id: 1358, coord: Vec2::new(34., 67.) });
		// Plage maudite
		tmp.push(Coord { id: 1359, coord: Vec2::new(60., 32.) });
		// Prison maritime
		tmp.push(Coord { id: 1368, coord: Vec2::new(82., 82.) });
		// Mine Néo-Sufkienne
		tmp.push(Coord { id: 1365, coord: Vec2::new(71., 70.2) });
		// Caverne des Marteaux-Aigris
		tmp.push(Coord { id: 1364, coord: Vec2::new(37., 36.) });
		coords.insert(-7, tmp);

		FullMapCoord {
			coords: coords
		}
	}

	pub fn get_coord(&self, id: &i32, id2: &i32) -> Result<&Vec2> {
		if let Some(lst_coord) = self.coords.get(&id) {
			for coord in lst_coord {
				if coord.id == *id2 {
					return Ok(&coord.coord);
				}
			}
		}
		Err(anyhow!("Coord not found"))
	}
}