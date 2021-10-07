fn main() {
    let result = another_function(7);
    println!("another_function returns: {}", result.0);
}

fn another_function(x: u8) -> (u16,u8) {
    return ((255 + x).into(),1);
}
