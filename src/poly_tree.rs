use serde::{Deserialize, Serialize};
use crate::block_arrangement::BlockArrangement;

/// A datastructure for efficiently storing polycubes.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PolyTree {

}

impl PolyTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, block: BlockArrangement) -> bool {
        todo!()
    }

    pub fn contains(&self, block: &BlockArrangement) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point3D;
    use super::*;

    #[test]
    fn test_creation() {
        let _tree = PolyTree::default();
        let _tree = PolyTree::new();
    }

    #[test]
    fn test_add_contains_1() {
        let block = BlockArrangement::new();
        let mut tree = PolyTree::default();
        assert!(!tree.contains(&block));
        assert!(!tree.put(block.clone()));
        assert!(tree.contains(&block));
    }

    #[test]
    fn test_size_same() {
        let block = BlockArrangement::new();
        let mut tree = PolyTree::default();
        assert_eq!(0, tree.size());
        tree.put(block.clone());
        assert_eq!(1, tree.size());
        tree.put(block.clone());
        assert_eq!(1, tree.size());
        let mut block = block;
        block.add_block_at(&Point3D::new(1,0,0))
            .expect("Expected save adding.");
        tree.put(block.clone());
        assert_eq!(2, tree.size());
        tree.put(block.clone());
        assert_eq!(2, tree.size());
    }
}