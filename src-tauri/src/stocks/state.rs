use sqlx::PgPool as Pool;
use super::models::{StockGet, StockInsert};

pub struct StocksState {}

impl StocksState {
    pub async fn update(updated_stock: StockGet, pool: &Pool) -> u64 {
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

    pub async fn search(term: String, pool: &Pool) -> Vec<StockGet> {
        let stocks: Vec<StockGet> = sqlx::query_as!(
            StockGet,
            r#"
SELECT
    stock_id,
     stock_name,
    uprice, 
    quantity, 
    date_expiry,
    dispensers.name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE stocks.search_tokens @@ plainto_tsquery($1)
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
            term
        )
        .fetch_all(pool)
        .await
        .unwrap();

        stocks
    }

    pub async fn get(offset: i64, pool: &Pool) -> Vec<StockGet> {
        let stocks: Vec<StockGet> = sqlx::query_as!(
            StockGet,
            r#"
SELECT
    stock_id,
    stock_name,
    uprice,
    quantity,
    date_expiry,
    dispensers.name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE
    date_expiry >= CURRENT_DATE + interval '+7 day' * $1
        AND
    date_expiry < CURRENT_DATE + interval '+7 day' + interval '+7 day' * $1
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
            offset as f64
        )
        .fetch_all(pool)
        .await
        .unwrap();

        stocks
    }

    pub async fn insert(new_stock: StockInsert, pool: &Pool) -> u64 {
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