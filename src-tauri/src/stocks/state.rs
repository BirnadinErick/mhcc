use sqlx::PgPool as Pool;
use super::models::{GetStock, AddStock};

pub struct StocksState {}

impl StocksState {
    pub async fn update(updated_stock: GetStock, pool: &Pool) -> u64 {
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

        let res = sqlx::query(&query).execute(pool).await.unwrap();

        res.rows_affected()
    }

    pub async fn insert(new_stock: AddStock, pool: &Pool) -> u64 {
        // TODO: correct staff_stocked and dispenser_id
        let res = sqlx::query(
            "
INSERT INTO stocks(stock_name, uprice, quantity, date_expiry, staff_stocked, dispenser_id) VALUES (
    $1, $2, $3, $4, 1, 100
 );
        ",
        )
        .bind(new_stock.stock_name)
        .bind(new_stock.uprice)
        .bind(new_stock.quantity)
        .bind(new_stock.date_expiry)
        .execute(pool)
        .await
        .unwrap();

        res.rows_affected()
    }
}
