use sqlx::PgPool as Pool;
use super::models::{GetStock, AddStock};

pub struct StocksState {}

impl StocksState {
    pub async fn insert(new_stock: AddStock, pool: &Pool) {}
}
