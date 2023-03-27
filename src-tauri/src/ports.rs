use std::collections::BTreeMap;
use async_trait::async_trait;
use crate::stocks::models::*;
use crate::patients::models::*;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StockService: {
    async fn add_stock(&self, add_stock: &AddStock) -> u64;
    async fn get_stocks(&self, offset: f64) -> Vec<GetStock>;
    async fn update_stock(&self, updated_stock: GetStock) -> u64;
    async fn search_stock(&self, query: String) -> Vec<GetStock>;
    async fn delete_stock(&self, id: i64) -> bool;
	}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PatientService: {
    async fn add_patient(&self, add_patient: AddPatient) -> u64;
    async fn get_patients(&self, offset: f64) -> Vec<GetPatient>;
    async fn update_patient(&self, updated_patient: GetPatient) -> u64;
    async fn search_patient_by_name(&self, query: String) -> Vec<GetPatient>;
    async fn search_patient_by_nic(&self, query: String) -> Vec<GetPatient>;
    // async fn delete_stock(&self, id: i64) -> bool;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GRNService: {
    async fn save_grn(&self, stock_id:i64, quantity:i64, staff_id:i64) -> bool;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DispenserService: {
    async fn add_dispenser(&self, dispenser_name:String) -> i32;
    async fn make_dispensers_index(&self) -> BTreeMap<String, i32>;
}
