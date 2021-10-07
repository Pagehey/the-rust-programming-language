use std::io;

fn main() {
    println!("enter an index");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line!");

    let index: u8 = index
        .trim()
        .parse().
        expect("not a number!");

    println!("index is {}", index);
}
