use std::collections::BTreeMap;
use sqlx::postgres::PgPoolOptions;

pub struct PgAdapter {
    pub pool: sqlx::PgPool,
	pub d_index: Option<BTreeMap<String, i32>>,
}


impl PgAdapter {
    pub async fn new(url: &str, conn: u32) -> PgAdapter {
        let pool = PgPoolOptions::new()
        .max_connections(conn)
        .connect(url)
        .await
        .expect("Failed to initiate database pool");

		PgAdapter {pool, d_index:None}
    }
}
