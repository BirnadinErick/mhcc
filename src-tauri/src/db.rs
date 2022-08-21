use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use super::models;
use super::schema;

pub struct Misc {}

impl Misc {
    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DB_URL MUST BE SET!");

        SqliteConnection::establish(&db_url)
            .expect(&format!("Error establishing the db connection!"))
    }

    pub fn backup() {
        todo!()
    }
}

pub struct Staff {}

impl Staff {
    pub fn create_staff<'a>(
        conn: &mut SqliteConnection,
        name: &'a str,
        uname: &'a str,
        passwd: &'a str,
        role: i32,
        date_enrolled: i32,
    ) -> usize {
        use schema::staffs;

        let new_staff = models::NewStaff {
            name,
            uname,
            passwd,
            role,
            date_enrolled,
        };

        diesel::insert_into(staffs::table)
            .values(&new_staff)
            .execute(conn)
            .expect("Error Saving new staff!")
    }

    pub fn read_staffs() {
        // use schema::staffs::dsl::*;

        // let res = staffs
        //     .filter(role.eq(1))
        //     .limit(5)
        //     .load::<models::Staff>(conn)
        //     .expect("Error loading staffs details!");

        // println!("Displaying {} staffs", res.len());
        // for staff in res {
        //     println!(
        //         "{}::{:?} with role number {}",
        //         staff.name,
        //         staff.staff_id.unwrap(),
        //         staff.role
        //     );
        // }
        todo!()
    }
}
pub struct Visitor {
    // total: u64,
}

impl Visitor {
    pub fn create_visitor<'a>(
        conn: &mut SqliteConnection,
        name: &'a str,
        address: &'a str,
        tpno: &'a str,
        dob: i32,
        nic: &'a str,
    ) -> usize {
        use schema::visitors;

        let new_visitor = models::NewVisitor {
            name,
            address,
            tpno,
            dob,
            nic,
        };

        diesel::insert_into(visitors::table)
            .values(&new_visitor)
            .execute(conn)
            .expect("Error Saving new visitor!")
    }
}

pub struct Stock {}

impl Stock {
    pub fn create_stock<'a>(
        conn: &mut SqliteConnection,
        name: &'a str,
        dispenser: &'a str,
        uprice: f32,
        quantity: i32,
        date_expiry: i32,
        staff_stocked: i32,
    ) -> usize {
        use schema::stocks;

        let new_stock = models::NewStock {
            name,
            dispenser,
            uprice,
            quantity,
            date_expiry,
            staff_stocked,
        };

        diesel::insert_into(stocks::table)
            .values(&new_stock)
            .execute(conn)
            .expect("Error saving new stock!")
    }
}

pub struct GRN {}

impl GRN {
    pub fn create_grn(conn: &SqliteConnection, date_returned: i32, staff_returned: i32) -> usize {
        use schema::grns;

        let new_grn = models::NewGRN {
            date_returned,
            staff_returned,
        };

        diesel::insert_into(grns::table)
            .values(&new_grn)
            .execute(conn)
            .expect("Error saving new GRN")
    }
}

pub struct Sales {}

impl Sales {
    pub fn create_sales(
        conn: &SqliteConnection,
        v_id: i32,
        staff_id: i32,
        date_sold: i32,
    ) -> usize {
        use schema::sales;

        let new_sale = models::NewSales {
            v_id,
            staff_id,
            date_sold,
        };

        diesel::insert_into(sales::table)
            .values(&new_sale)
            .execute(conn)
            .expect("Error saving new sale")
    }
}

pub struct SalesItem {}

impl SalesItem {
    pub fn create_sales_item(
        conn: &SqliteConnection,
        sales_id: i32,
        stock_id: i32,
        quantity: i32,
        uprice: f32,
    ) -> usize {
        use schema::sales_item;

        let new_item = models::NewSalesItem {
            sales_id,
            stock_id,
            quantity,
            uprice,
        };

        diesel::insert_into(sales_item::table)
            .values(&new_item)
            .execute(conn)
            .expect("Error saving new sales item")
    }
}
