mod block_arrangement;
mod mapper;
mod point;
mod block_hash;
mod orientation;

use std::env;

/// This program calculates out how many unique arangements can be made for n cubes attached to one another
/// at the faces.
fn main() {
    let n: usize = env::args().next().map(|s| s.parse())
        .expect("Expected at least one numeric arguments")
        .expect("The argument has to be a valid number");
    println!("The number of arrangements is {}", "todo");
    todo!()
}

