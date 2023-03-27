use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Dispenser {
	pub dispenser_id: i32,
	pub dispenser_name: String
}

pub struct NewDispenser {
	pub dispenser_id: i32
}
