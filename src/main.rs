mod blocks;
mod mapper;
mod point;
mod block_hash;

use std::env;
use crate::block_hash::BlockHash;
use crate::blocks::BlockArrangement;
use crate::mapper::RotationAmount;
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
    dbg!(block.iter().collect::<Vec<_>>());
    block.orientation_mut().rotate(Axis3D::Z, RotationAmount::Ninety);
    dbg!(BlockHash::from(&block));
    dbg!(block.iter().collect::<Vec<_>>());
}

