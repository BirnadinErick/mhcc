extern crate mhcc;
extern crate diesel;

use self::diesel::prelude::*;
use std::env::args;

fn main() {

    use mhcc::schema::staffs::dsl::*;

    let target = args()
        .nth(1)
        .expect("Provide an id!")
        .parse::<i32>()
        .expect("Couldn't parse the id!");

    let conn = &mut mhcc::db::Misc::establish_connection();

    let del_num = diesel::delete(staffs.filter(staff_id.eq(target)))
    .execute(conn)
    .expect("Error deleting staff");

    println!("{} of records deleted", del_num);
}