use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::block_arrangement::block_sets::BlockSet;
use crate::block_arrangement::BlockArrangement;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct HashBlockset {
    hashset: HashSet<BlockArrangement>,
}

impl<'a> BlockSet<'a> for HashBlockset {
    fn contains(&self, ba: &BlockArrangement) -> bool {
        self.hashset.contains(ba)
    }

    fn insert(&mut self, ba: BlockArrangement) -> bool {
        self.hashset.insert(ba)
    }

    fn len(&self) -> usize {
        self.hashset.len()
    }

    fn count_arrangements_with_n_blocks(&self, num_blocks: u8) -> usize {
        self.hashset.iter()
            .filter(|v| v.num_blocks() == num_blocks)
            .count()
    }
}

