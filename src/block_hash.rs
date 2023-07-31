use getset::CopyGetters;
use rust_decimal::{Decimal, RoundingStrategy};
use crate::blocks::BlockArrangement;

/// A hash like value for a [BlockArrangement].
/// The values aim to uniquely identify a Block arrangement independent of any mirroring or
/// rotational symmetry.
#[derive(Eq, PartialEq, Default, Hash, Clone, Ord, PartialOrd, Debug)]
#[derive(CopyGetters)]
pub struct BlockHash {
    #[get_copy = "pub"]
    num_blocks: u8,
    /// A measure for how close blocks are to the center of mass.
    #[get_copy = "pub"]
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

