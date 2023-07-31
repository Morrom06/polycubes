mod blocks;
mod mapper;
mod point;
mod block_hash;
mod orientation;

use std::env;
use crate::block_hash::BlockHash;
use crate::blocks::BlockArrangement;
use crate::point::{Axis3D, Point3D};

/// This program calculates out how many unique arangements can be made for n cubes attached to one another
/// at the faces.
fn main() {
    t();
    let n: usize = env::args().next().map(|s| s.parse())
        .expect("Expected at least one numeric arguments")
        .expect("The argument has to be a valid number");
    println!("The number of arrangements is {}", "todo");
    todo!()
}

fn t() {
    let mut block = BlockArrangement::new();
    let hash = BlockHash::from(&block);
    dbg!(hash);
    block.add_block_at(Point3D::new(1,0,0)).expect("Save adding");
    dbg!(BlockHash::from(&block));
    block.add_block_at(Point3D::new(0,1,0)).expect("Save adding");
    dbg!(BlockHash::from(&block));
    // dbg!(block.iter().collect::<Vec<_>>());
    let hash1 = BlockHash::from(&block);
    block.orientation_mut().mirror(Axis3D::Z);
    let hash2 = BlockHash::from(&block);
    dbg!(BlockHash::from(&block));
    // dbg!(block.iter().collect::<Vec<_>>());
    assert_eq!(hash1, hash2)
}

