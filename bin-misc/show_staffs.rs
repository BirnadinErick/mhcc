extern crate diesel;
extern crate mhcc;

use self::diesel::prelude::*;
use self::mhcc::*;
use self::models::*;

fn main() {
    use mhcc::schema::staffs::dsl::*;

    let conn = &mut db::Misc::establish_connection();

    let res = staffs
        .filter(role.eq(1))
        .limit(5)
        .load::<Staff>(conn)
        .expect("Error loading staffs details!");

    println!("Displaying {} staffs", res.len());
    for staff in res {
        println!(
            "{}::{:?} with role number {}",
            staff.name, staff.staff_id.unwrap(), staff.role
        );
    }
}
