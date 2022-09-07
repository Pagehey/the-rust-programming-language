#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    admin: bool,
    rank: u16
}

#[allow(dead_code)]
fn build_user(username: String, email: String, admin: bool, rank: u16) -> User {
    User {
        username,
        email,
        admin,
        rank
    }
}

use std::ops::{Deref, Add};
struct Meter(u32);

impl Deref for Meter {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let u1 = build_user(String::from("Pagehey"), String::from("pagehey@pm.me"), true, 1);

    // println!("first user is {:?}", u1);
    let h = Meter(10);

    // println!("{}", *h + 10);
    // println!("{}", h.add(10));
    let n = 0xFusize;
    let n2: usize = 0;
    let sum =  n + n2;
    println!("{}", sum);
}
