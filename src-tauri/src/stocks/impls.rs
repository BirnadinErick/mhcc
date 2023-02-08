use async_trait::async_trait;
use super::models::*;
use crate::ports::StockService;
use crate::adapters::PgAdapter;

#[async_trait]
impl StockService for PgAdapter {
    async fn add_stock(&self, add_stock: &AddStock) -> u64 {
        let res = sqlx::query(
            "
INSERT INTO stocks(stock_name, uprice, quantity, date_expiry, staff_id, dispenser_id) VALUES (
    $1, $2, $3, $4, 1, 100
 );
        ",
        )
        .bind(add_stock.stock_name.as_str())
        .bind(add_stock.uprice)
        .bind(add_stock.quantity)
        .bind(add_stock.date_expiry)
        .execute(&self.pool)
        .await
        .expect("add_stock failed");

        res.rows_affected()

    }

    async fn get_stocks(&self, offset: f64) -> Vec<GetStock> {
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

    async fn update_stock(&self, updated_stock: GetStock) -> u64 {
        let query = format!(
            r#"
UPDATE stocks
SET
    stock_name = '{}',
    uprice = {},
    date_expiry = '{}',
    quantity = {}
WHERE stock_id = {};
        "#,
            updated_stock.stock_name,
            updated_stock.uprice,
            updated_stock.date_expiry,
            updated_stock.quantity,
            updated_stock.stock_id
        );

        let res = sqlx::query(&query)
			.execute(&self.pool)
			.await
			.expect("couldn't update");

        res.rows_affected()
    }

    // async fn delete_stock(&self, _id: i64) -> bool {
    //     todo!("delete_stocks not implemented for PgAdapter")
    // }

    async fn search_stock(&self, query: String) -> Vec<GetStock> {
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
WHERE stocks.search_tokens @@ plainto_tsquery($1)
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
            query
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        stocks
    }

}
