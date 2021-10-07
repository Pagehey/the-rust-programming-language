use std::io;

fn main() {
    println!("How far?");

    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed");

    let nth: isize = nth.trim().parse().expect("Not an integer!");

    let result = fib2(nth);
    println!("The {}nth Fibonacci number is {}", nth, result);
}

// fn fib(nth: usize) -> usize {
//     let mut previous: usize = 0;
//     let mut result: usize = 1;

//     // 1 1 2 3 5 8 13 21 34 55
//     for _ in 1..nth {
//         let temp = previous;
//         previous = result;
//         result = result + temp;
//     }
//     result
// }

fn fib2(nth: isize) -> isize {
    if nth <= 1 { nth } else { fib2(nth - 1) + fib2(nth - 2) }
}

