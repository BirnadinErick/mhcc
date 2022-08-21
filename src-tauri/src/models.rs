use super::schema::{grns, sales, sales_item, staffs, stocks, visitors};

#[derive(Queryable)]
pub struct Staff {
    pub staff_id: Option<i32>,
    pub name: String,
    pub uname: String,
    pub passwd: String,
    pub role: i32,
    pub date_enrolled: i32,
}

#[derive(Queryable)]
pub struct Visitor {
    pub v_id: Option<i32>,
    pub name: String,
    pub address: String,
    pub tpno: String,
    pub dob: i32,
    pub nic: String,
}

#[derive(Queryable)]
pub struct Stock {
    pub stock_id: Option<i32>,
    pub name: String,
    pub dispenser: String,
    pub uprice: i32,
    pub quantity: i32,
    pub date_expiry: i32,
    pub staff_stocked: i32,
}

#[derive(Queryable)]
pub struct GRN {
    pub grn_id: Option<i32>,
    pub date_returned: i32,
    pub staff_returned: i32,
}

#[derive(Queryable)]
pub struct Sales {
    pub sales_id: Option<i32>,
    pub visitor: i32,
    pub staff: i32,
    pub date_sold: i32,
}

#[derive(Queryable)]
pub struct SalesItem {
    pub sales_id: i32,
    pub stock_id: i32,
    pub quantity: i32,
    pub uprice: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "staffs"]
pub struct NewStaff<'a> {
    pub name: &'a str,
    pub uname: &'a str,
    pub passwd: &'a str,
    pub role: i32,
    pub date_enrolled: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "visitors"]
pub struct NewVisitor<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub tpno: &'a str,
    pub dob: i32,
    pub nic: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "stocks"]
pub struct NewStock<'a> {
    pub name: &'a str,
    pub dispenser: &'a str,
    pub uprice: f32,
    pub quantity: i32,
    pub date_expiry: i32,
    pub staff_stocked: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "grns"]
pub struct NewGRN {
    pub date_returned: i32,
    pub staff_returned: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sales"]
pub struct NewSales {
    pub v_id: i32,
    pub staff_id: i32,
    pub date_sold: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sales_item"]
pub struct NewSalesItem {
    pub sales_id: i32,
    pub stock_id: i32,
    pub quantity: i32,
    pub uprice: f32,
}
