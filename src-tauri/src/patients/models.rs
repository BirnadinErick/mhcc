use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetPatient {
    pub patient_id: i32,
    pub patient_name: String,
    pub dob: NaiveDate,
	pub address: String,
	pub tpno: String,
	pub nic: String
}

#[derive(Serialize, Deserialize)]
pub struct AddPatient {
    pub patient_name: String,
    pub dob: NaiveDate,
	pub address: String,
	pub tpno: String,
	pub nic: String
}
