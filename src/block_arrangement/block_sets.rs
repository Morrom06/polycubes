use serde::{Deserialize, Serialize};
use crate::block_arrangement::BlockArrangement;

pub mod poly_tree;
mod hash_blockset;

pub trait BlockSet<'a>:
    Deserialize<'a> + Serialize
    + Default
{

    fn contains(&self, ba: &BlockArrangement) -> bool;

    fn insert(&mut self, ba: BlockArrangement) -> bool;

    fn len(&self) -> usize;

    fn count_arrangements_with_n_blocks(&self, num_blocks: u8) -> usize;
    
}