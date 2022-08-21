extern crate diesel;
extern crate mhcc;

use self::diesel::prelude::*;
use self::mhcc::*;
use std::env::args;

fn main() {
    use mhcc::schema::staffs::dsl::{role, staffs};

    let staff_id = args()
        .nth(1)
        .expect("provide a staff_id")
        .parse::<i32>()
        .expect("couldn't parse the id");

    let new_role = args()
        .nth(2)
        .expect("provide a role")
        .parse::<i32>()
        .expect("couldn't parse the role");

    let conn = &mut db::Misc::establish_connection();

    let _ = diesel::update(staffs.find(staff_id))
        .set(role.eq(new_role))
        .execute(conn)
        .expect(&format!("Unable to find staff {}", staff_id));

    println!("Changed role!");
}
