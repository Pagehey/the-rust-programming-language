mod guess;
use crate::guess::Guess;

fn main() {
    let g = Guess::new(1);

    println!("Guess value is {:?}", g.value());
}
