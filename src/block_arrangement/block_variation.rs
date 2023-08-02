use std::collections::HashSet;
use std::collections::hash_set::IntoIter;
use crate::block_arrangement::BlockArrangement;
use crate::point::Point3D;

/// Creates different variations of a [BlockArrangement] that has one more block.
/// Generated variations are guaranteed to be unique against each other.
pub struct VariationGenerator {
    block: BlockArrangement,
    new_block_pos_iter: IntoIter<Point3D<i32>>,
}

impl VariationGenerator {
    pub fn new(ba: BlockArrangement) -> Self {
        let p_set: HashSet<_> = ba.block_iter()
            .flat_map(|block_p| BlockArrangement::NEIGHBOR_OFFSETS
                .map(|o| o + block_p))
            .filter(|p| !ba.is_set(p))
            .collect();
        Self {
            block: ba,
            new_block_pos_iter: p_set.into_iter(),
        }
    }
}


impl Iterator for VariationGenerator {
    type Item = BlockArrangement;

    fn next(&mut self) -> Option<Self::Item> {
        let mut variation = self.block.clone();
        let p = self.new_block_pos_iter.next()?;
        variation.add_block_at(&p)
            .unwrap_or_else(|_e| panic!("Expected save block placement at point {p} but wasn't"));
        Some(variation)
    }
}

#[cfg(test)]
mod tests {
    use crate::block_hash::BlockHash;
    use super::*;

    #[test]
    fn test_single_variations() {
        let block = BlockArrangement::new();
        let variations = VariationGenerator::new(block)
            .collect::<Vec<_>>();
        let expected_len = 6;
        assert_eq!(expected_len, variations.len());
        assert_eq!(1, variations.into_iter().collect::<HashSet<_>>().len())
    }

    #[test]
    fn test_double_variations() {
        let mut block = BlockArrangement::new();
        block.add_block_at(&Point3D::new(1,0,0)).expect("Save placement");
        let variations = VariationGenerator::new(block)
            .map(|b| (BlockHash::from(&b), b))
            .collect::<Vec<_>>();
        let expected_len = 10;
        assert_eq!(expected_len, variations.len());
        let set = variations.into_iter()
            .map(|t|t.1)
            .collect::<HashSet<_>>();
        dbg!(
            &set.iter()
                .map(|ba| (
                    ba.block_iter().collect::<Vec<_>>(),
                    ba.center_mass_iter().collect::<Vec<_>>(),
                    BlockHash::from(ba)))
                .collect::<Vec<_>>()
        );
        assert_eq!(2, set.len(), "Number of unique shapes does not match expected amount")
    }

    #[test]
    fn test_triple_l_variation() {
        let mut block = BlockArrangement::new();
        block.add_block_at(&Point3D::new(1,0,0)).expect("Save placement");
        block.add_block_at(&Point3D::new(0,1,0)).expect("Save placement");
        let variations = VariationGenerator::new(block)
            .collect::<Vec<_>>();
        let expected_len = 13;
        assert_eq!(expected_len, variations.len());
    }
}