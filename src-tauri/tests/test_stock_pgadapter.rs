use pretty_assertions::{assert_eq};
use mhcc::{
	adapters::PgAdapter,
	ports::StockService
};

#[test]
fn it_works(){
    assert_eq!(1,0);
}

#[sqlx_database_tester::test(
	pool(variable = "test_pool",),
)]
async fn test_get_stocks() {
	let adapter = PgAdapter { pool: test_pool };
	let stocks = PgAdapter::get_stock(&adapter, 0.0).await;
	println!("{:#?}", stocks);
	todo!("write tests for get_stock")
}
