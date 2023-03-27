use std::collections::BTreeMap;
use async_trait::async_trait;
use super::models::{Dispenser, NewDispenser};
use crate::ports::DispenserService;
use crate::adapters::PgAdapter;

#[async_trait]
impl DispenserService for PgAdapter {
	async fn make_dispensers_index(&self) -> BTreeMap<String, i32> {

		let mut d_index: BTreeMap<String,i32> = BTreeMap::new();

		let dispensers: Vec<Dispenser> = sqlx::query_as!(
			Dispenser,
			r#"
SELECT dispenser_id, dispenser_name FROM dispensers
ORDER BY dispenser_id ASC, dispenser_name ASC;
			"#
		)
		.fetch_all(&self.pool)
		.await
		.expect("querying dispensers failed");

		for dispenser in dispensers.iter() {
			d_index.insert(
				dispenser.dispenser_name.to_owned(),
				dispenser.dispenser_id
			);
		}

		d_index
	}

	async fn add_dispenser(&self, dispenser_name:String) -> i32 {
		let new_dispenser:Vec<NewDispenser> = sqlx::query_as!(
			NewDispenser,
			r#"
INSERT INTO dispensers(dispenser_name) VALUES
($1)
RETURNING dispenser_id;
			"#,
			dispenser_name
		)
		.fetch_all(&self.pool)
		.await
		.expect("dispenser newly not created");

		new_dispenser[0].dispenser_id
	}
}
