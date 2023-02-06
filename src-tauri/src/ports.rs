use async_trait::async_trait;
use crate::stocks::models::*;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StockService: {
    async fn add_stock(&self, add_stock: &AddStock) -> u64;
    async fn get_stock(&self, offset: f64) -> Vec<GetStock>;
    async fn update_stock(&self, updated_stock: GetStock) -> u64;
    async fn delete_stock(&self, id: i64) -> bool;
    async fn search_stock(&self, query: String) -> Vec<GetStock>;
}
