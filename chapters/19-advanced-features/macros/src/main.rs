use hello_macro_derive::{HelloMacro, route};
use hello_macro::HelloMacro;

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        {
            // let mut v = Vec::new();
            $(
                println!("{}", $x);
            )*
            1
        }
    }
}

#[derive(HelloMacro)]
struct Pancake {}

#[route(GET, "/")]
fn index() {
    println!("routing to /")
}

fn main() {
    // let v = vec![2];
    // println!("{:?}", v);
    index();
    Pancake::hello_macro();
}
