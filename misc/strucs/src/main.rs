#[derive(Debug)]
struct User {
    username: String,
    email: String,
    admin: bool,
    rank: u16
}

fn build_user(username: String, email: String, admin: bool, rank: u16) -> User {
    User {
        username,
        email,
        admin,
        rank
    }
}

fn main() {
    let u1 = build_user(String::from("Pagehey"), String::from("pagehey@pm.me"), true, 1);

    println!("first user is {:?}", u1);
}
