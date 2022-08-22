extern crate diesel;
extern crate mhcc;

use self::mhcc::*;
use std::io;

fn main() {
    let conn = &mut db::Misc::establish_connection();

    let mut name = String::new();
    let mut uname = String::new();
    let mut passwd = String::new();

    println!("Enter staff's name: ");
    io::stdin().read_line(&mut name).unwrap();
    println!("\nEnter a username: ");
    io::stdin().read_line(&mut uname).unwrap();
    println!("\nEnter a passwd: ");
    io::stdin().read_line(&mut passwd).unwrap();

    let new_name = &name[..(name.len() -2)];
    let new_uname = &uname[..(uname.len() -2)];
    let new_passwd = &passwd[..(passwd.len() -2)];

    db::Staff::create_staff(
        conn, &new_name, &new_uname, &new_passwd, 1, 20030519
    );

    println!("\nRecorded succesfully");
}
