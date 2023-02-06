use async_trait::async_trait;
use super::models::*;
use crate::ports::StockService;
use crate::adapters::PgAdapter;

#[async_trait]
impl StockService for PgAdapter {
    async fn add_stock(&self, add_stock: &AddStock) -> bool {
        panic!("delete_stocks not implemented for PgStockService")
    }

    async fn get_stock(&self, offset: f64) -> Vec<GetStock> {
        let stocks: Vec<GetStock> = sqlx::query_as!(
            GetStock,
            r#"
SELECT
    stock_id,
    stock_name,
    uprice,
    quantity,
    date_expiry,
    dispensers.dispenser_name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE
    date_expiry >= CURRENT_DATE + interval '+7 day' * $1
        AND
    date_expiry < CURRENT_DATE + interval '+7 day' + interval '+7 day' * $1
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .expect("Failed to fetch stocks!");

        stocks
    }

    async fn update_stock(&self) -> bool {
        panic!("update_stocks not implemented for PgStockService")
    }

    async fn delete_stock(&self, id: i64) -> bool {
        panic!("delete_stocks not implemented for PgStockService")
    }

    async fn search_stock(&self, query: String) -> Vec<GetStock> {
        panic!("search_stocks not implemented for PgStockService")
    }

}
