pub mod impls {
	use crate::ports::GRNService;
	use crate::adapters::PgAdapter;
	use async_trait::async_trait;

	#[async_trait]
	impl GRNService for PgAdapter {
		async fn save_grn(
			&self,
			stock_id:i64, quantity:i64, staff_id:i64
		) -> bool {
			let row = sqlx::query("
INSERT INTO grns(stock_id, quantity, staff_id) VALUES(
	$1, $2, $3
);
"
			)
			.bind(stock_id)
			.bind(quantity)
			.bind(staff_id)
			.execute(&self.pool)
			.await
			.expect("failed to save grn");

			if row.rows_affected() == 1 {
				true
			}else{
				false
			}
		}
	}
}
