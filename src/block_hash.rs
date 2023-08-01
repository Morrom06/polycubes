use getset::CopyGetters;
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use crate::block_arrangement::BlockArrangement;

/// A hash like value for a [BlockArrangement].
/// The values aim to uniquely identify a Block arrangement independent of any mirroring or
/// rotational symmetry.
#[derive(Eq, PartialEq, Default, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
#[derive(CopyGetters)]
#[derive(Serialize, Deserialize)]
pub struct BlockHash {
    #[get_copy = "pub"]
    num_blocks: u8,
    /// A measure for how close blocks are to the center of mass.
    #[get_copy = "pub"]
    #[serde(with = "rust_decimal::serde::str")]
    density: Decimal,
    /// Sorted by size for consistency.
    #[get_copy = "pub"]
    axis_alignments: [Decimal; 3]
}

impl BlockHash {
    fn round(&mut self) {
        let default_round = |dec: &mut Decimal| {
            *dec = dec.round_dp_with_strategy(5, RoundingStrategy::MidpointAwayFromZero)
        };
        self.axis_alignments.iter_mut()
            .for_each(default_round);
        default_round(&mut self.density)
    }
}

impl From<&BlockArrangement> for BlockHash {
    fn from(ba: &BlockArrangement) -> Self {
        let mut alignment = ba.axis_alignments();
        alignment.sort();
        let mut hash = Self {
            num_blocks: ba.num_blocks(),
            density: ba.density(),
            axis_alignments: alignment,
        };
        hash.round();
        hash
    }
}

#[cfg(test)]
mod tests {
    use crate::orientation::OrientationIterator;
    use crate::point::Point3D;
    use super::*;

    #[test]
    fn test_orientation_hashing() {
        let mut block = BlockArrangement::new();
        block.add_block_at(&Point3D::new(1,0,0)).expect("Save adding");
        block.add_block_at(&Point3D::new(0,1,0)).expect("Save adding");
        let hash = BlockHash::from(&block);
        OrientationIterator::default()
            .for_each(|orientation| {
                block.set_orientation(orientation);
                let oriented_hash = BlockHash::from(&block);
                assert_eq!(hash, oriented_hash)
            })
    }

    #[test]
    fn test_serde() {
        let mut block = BlockArrangement::new();
        block.add_block_at(&Point3D::new(1,0,0)).expect("Save adding");
        block.add_block_at(&Point3D::new(0,1,0)).expect("Save adding");
        let hash = BlockHash::from(&block);

        let serial = bincode::serde::encode_to_vec(
            hash,
            bincode::config::standard()
        ).expect("Expecting a save serialization.");
        let (deser_hash, _): (BlockHash, _) = bincode::serde::decode_from_slice(&serial[..], bincode::config::standard())
            .expect("Expecting save decoding.");
        assert_eq!(hash, deser_hash);
    }
}