use sqlx::postgres::PgPoolOptions;

pub struct PgAdapter {
    pub pool: sqlx::PgPool,
}


impl PgAdapter {
    pub async fn new(url: &str, conn: u32) -> PgAdapter {
        let pool = PgPoolOptions::new()
        .max_connections(conn)
        .connect(url)
        .await
        .expect("Failed to initiate database pool");

		PgAdapter {pool}
    }
}
