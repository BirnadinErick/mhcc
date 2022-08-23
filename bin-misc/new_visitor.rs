extern crate mhcc;
extern crate diesel;
extern crate chrono;

use self::mhcc::*;
use std::io;

fn main(){
    let mut conn = db::Misc::establish_connection();

    let mut name = String::new();
    let mut address = String::new() ;
    let mut tpno = String::new();
    let dob = chrono::Local::now().timestamp() as i32;
    let mut nic = String::new();

    println!("Enter visitors's name: ");
    io::stdin().read_line(&mut name).unwrap();
    println!("Enter visitor's address: ");
    io::stdin().read_line(&mut address).unwrap();
    println!("Enter visitor's  tpno: ");
    io::stdin().read_line(&mut tpno).unwrap();
    println!("Enter visitor's nic: ");
    io::stdin().read_line(&mut nic).unwrap();

    db::Visitor::create_visitor(&mut conn, &name, &address, &tpno, dob, &nic);

    println!("\nRecorded succesfully");
}