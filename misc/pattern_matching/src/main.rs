#[derive(Debug)]
enum UsState {
    Maine,
    Washington
}

enum Coin {
    Penny,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> usize {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Washington);
    let other_coin = Coin::Penny;
    value_in_cents(Coin::Quarter(UsState::Maine));

    plus_one(Some(5));

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value { println!("three"); }

    stuff(other_coin);
    stuff(coin);
}

fn stuff(coin: Coin) -> u32 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count
    } else {
        count += 1;
        count
    }
}
