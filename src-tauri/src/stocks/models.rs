use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStock {
    pub stock_id: i32,
    pub stock_name: String,
    pub uprice: f32,
    pub quantity: i32,
    pub dispensers_name: String,
    pub date_expiry: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct AddStock {
    pub stock_name: String,
    pub uprice: f32,
    pub quantity: i32,
    pub date_expiry: NaiveDate,
}
