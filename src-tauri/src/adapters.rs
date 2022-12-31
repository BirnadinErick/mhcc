pub struct PgAdapter {
    pub pool: sqlx::PgPool,
}


impl PgAdapter {
    async fn new() {
        panic!("not impl'd");
    }
}