use fixedbitset::FixedBitSet;
use getset::CopyGetters;
use rust_decimal::Decimal;
use crate::mapper::{Mapper};
use crate::orientation::{Orientation, OrientationIterator};
use crate::point::{Axis3D, Finite3DDimension, Point3D};


/// Describes an arrangement of blocks joined at their faces in a rotation and directionless manner.
#[derive(Debug, Clone)]
#[derive(CopyGetters)]
pub struct BlockArrangement {
    /// Represents the block placement
    bitset: FixedBitSet,
    /// The number of blocks in this arrangement.
    /// Is always > 0
    #[get_copy = "pub"]
    num_blocks: u8,
    /// Offset from origin
    center_off_mass: Point3D<i32>,
    mapper: Mapper,
}

impl PartialEq for BlockArrangement {
    fn eq(&self, other: &Self) -> bool {
        let mut mapper = self.mapper.clone();
        OrientationIterator::default().any(|orientation| {
            mapper.set_orientation(orientation);

            // Todo: remove debugging
            let (equal_block_coordinates, unequal_block_coordinates): (Vec<_>, Vec<_>) = self.center_mass_iter().map(|mut p| {
                p.apply_orientation(&orientation);
                (p, mapper.unresolve(p), self.mapper.unresolve(p), other.mapper.unresolve(p))
            }
            )
                .partition(|(p, _, _, _)| {
                    other.is_set(&p)
                });
            dbg!(
                equal_block_coordinates,
                unequal_block_coordinates,
                self.center_mass_iter()
                .map(|mut p| {
                    p.apply_orientation(&orientation);
                    p
                })
                .all(|p| other.is_set_relative_to_center_of_mass(&p)),
                orientation
            );


            self.num_blocks == other.num_blocks
                && self.center_mass_iter()
                .map(|mut p| {
                    p.apply_orientation(&orientation);
                    p
                })
                .all(|p| other.is_set_relative_to_center_of_mass(&p))
        })
    }
}

impl Eq for BlockArrangement {}

#[derive(Debug, Eq, PartialEq)]
pub enum PlacementError {
    NotAdjacentToBlock
}

impl BlockArrangement {

    /// Creates a block arrangement with one block at the origin.
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(cap: usize) -> Self {
        let dim = Finite3DDimension::new(cap);
        let mut arr = Self {
            bitset: FixedBitSet::with_capacity(dim.size()),
            num_blocks: 0,
            center_off_mass: Point3D::default(),
            mapper: Mapper::new(dim)
        };
        arr.set_origin_block();
        arr
    }

    pub fn add_block_at(&mut self, point: &Point3D<i32>) -> Result<(), PlacementError> {
        if !self.has_neighbors(point) {
            return Err(PlacementError::NotAdjacentToBlock);
        }
        if self.num_blocks + 1 > self.mapper.dimension().arm_size() as u8 {
            self.grow((self.num_blocks + 1) as usize)
        }
        let index = self.mapper.unresolve(*point)
            .unwrap_or_else(|| panic!("Expected a save resolve from point {point} but was unsafe."));
        if !self.bitset[index] {
            self.num_blocks += 1;
        }
        self.bitset.set(index, true);
        self.update_center_of_mass();
        Ok(())
    }

    fn grow(&mut self, cap: usize) {
        let mut new_block = BlockArrangement::with_capacity(cap);
        self.bitset.ones()
            .map(|index| self.mapper.resolve(index).expect("Save mappings expected"))
            .map(|coordinate| new_block.mapper.unresolve(coordinate).expect("Save mapping expected since it of larger capacity"))
            .for_each(|index| new_block.bitset.set(index, true));
        new_block.num_blocks = self.num_blocks;
        *self = new_block;
    }

    /// Returns true if the point has any neighbor blocks.
    pub fn has_neighbors(&self, point: &Point3D<i32>) -> bool {
        const NEIGHBOR_OFFSETS: [Point3D<i32>; 26] = [
            Point3D::new(0, 0, -1),
            Point3D::new(0, 0, 1),
            Point3D::new(0, -1, 0),
            Point3D::new(0, -1, -1),
            Point3D::new(0, -1, 1),
            Point3D::new(0, 1, 0),
            Point3D::new(0, 1, -1),
            Point3D::new(0, 1, 1),
            Point3D::new(-1, 0, 0),
            Point3D::new(-1, 0, -1),
            Point3D::new(-1, 0, 1),
            Point3D::new(-1, -1, 0),
            Point3D::new(-1, -1, -1),
            Point3D::new(-1, -1, 1),
            Point3D::new(-1, 1, 0),
            Point3D::new(-1, 1, -1),
            Point3D::new(-1, 1, 1),
            Point3D::new(1, 0, 0),
            Point3D::new(1, 0, -1),
            Point3D::new(1, 0, 1),
            Point3D::new(1, -1, 0),
            Point3D::new(1, -1, -1),
            Point3D::new(1, -1, 1),
            Point3D::new(1, 1, 0),
            Point3D::new(1, 1, -1),
            Point3D::new(1, 1, 1)
        ];
        NEIGHBOR_OFFSETS.iter().cloned().map(|offset| offset + *point)
            .map(|coordinate| self.global_to_this(coordinate))
            // Resolves the point to the corresponding index and filters only in bound indices.
            .filter_map(|coordinate| self.mapper.unresolve(coordinate))
            .any(|i| self.bitset[i])
    }

    /// Applies the inverse of the current mapper orientation to the [Point3D<i32>].
    fn global_to_this(&self, mut point: Point3D<i32>) -> Point3D<i32> {
        point.apply_orientation(&self.mapper.orientation().inverse());
        point
    }

    /// Updates the center off mass.
    fn update_center_of_mass(&mut self) {
        self.center_off_mass = self.center_of_mass();
    }

    pub fn orientation_mut(&mut self) -> &mut Orientation {
        self.mapper.orientation_mut()
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.mapper.set_orientation(orientation);
    }

    /// Calculates the center of mass of the collection of blocks.
    /// If there are no blocks no center can be found.
    pub fn center_of_mass(&self) -> Point3D<i32> {
        self.center_mass_iter()
            .map(|p| (p, 1))
            .reduce(|a, b| {
            (a.0 + b.0, a.1 + b.1)
        }).map(|(sum_p, count)| sum_p.map_all(|v| v / count))
            .expect("Save call since there is always at least one block.")
    }

    /// Calculates the center of mass of the collection of blocks.
    /// If there are no blocks no center can be found.
    pub fn precise_center_of_mass(&self) -> Option<Point3D<f64>> {
        self.bitset.ones()
            .map(|i| {
                self.mapper.resolve(i).unwrap_or_else(|| panic!("An expected save index of {i} is out of bounds."))
            })
            .map(|coordinate| (coordinate.map_all(|i_val| i_val as f64), 1f64))
            .reduce(|a, b| {(a.0 + b.0, a.1 + b.1)})
            .map(|(sum_p, count)| sum_p.map_all(|v| v / count))
    }

    /// Returns an iterator over the coordinates of the blocks. The coordinates are offset
    /// by the center of mass.
    pub fn center_mass_iter(&self) -> impl Iterator<Item = Point3D<i32>> + '_ {
        let oriented_offset = self.oriented_offset_center_of_mass();
        self.bitset.ones()
            .map(move |index| self.mapper.resolve(index).expect("Expected save conversion") - oriented_offset)
    }

    /// Calculates the density of the blocks.
    /// It is the average distance to the origin.
    pub fn density(&self) -> Decimal {
        let sum: Decimal = self.center_mass_iter()
            .map(|p| p.distance_to_origin())
            .sum();
        sum / Decimal::from(self.num_blocks)
    }

    /// Calculates the alignment along the different axis.
    /// Returns an array of the alignment values with 0 being a straight line along the axis.
    /// The order is X Y Z.
    pub fn axis_alignments(&self) -> [Decimal; 3] {
        [
            self.axis_alignment(Axis3D::X),
            self.axis_alignment(Axis3D::Y),
            self.axis_alignment(Axis3D::Z),
        ]
    }

    /// Calculates the average distance of the block to the specified axis.
    /// The lower the value ther stronger the alignment.
    fn axis_alignment(&self, axis: Axis3D) -> Decimal {
        let sum: Decimal = self.center_mass_iter()
            .map(|point| {
                let distance = match axis {
                    Axis3D::X => {point.x()}
                    Axis3D::Y => {point.y()}
                    Axis3D::Z => {point.z()}
                }.abs();
                Decimal::from(distance)
            })
            .sum();
        sum / Decimal::from(self.num_blocks)
    }

    fn set_origin_block(&mut self) {
        self.bitset.set(self.mapper.unresolve(Point3D::default()).expect("Save conversion"), true);
        self.num_blocks += 1;
    }

    /// Returns the offset center off mass with the current mapper [Orientation] applied.
    fn oriented_offset_center_of_mass(&self) -> Point3D<i32> {
        let mut oriented_center = self.center_off_mass;
        oriented_center.apply_orientation(&self.mapper.orientation());
        oriented_center
    }

    /// Checks if a block at the point is set.
    pub fn is_set(&self, point: &Point3D<i32>) -> bool {
        self.mapper.unresolve(*point)
            .map(|index| self.bitset[index])
            .unwrap_or_default()
    }


    pub fn is_set_relative_to_center_of_mass(&self, point: &Point3D<i32>) -> bool {
        self.mapper.unresolve(*point + self.oriented_offset_center_of_mass())
            .map(|index| self.bitset[index])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod block_arrangement_tests {
    use crate::orientation::RotationAmount;
    use super::*;

    #[test]
    fn test_num_blocks() {
        let mut blocks = BlockArrangement::new();
        assert_eq!(1, blocks.num_blocks());
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        assert_eq!(2, blocks.num_blocks());
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        assert_eq!(3, blocks.num_blocks());
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        assert_eq!(3, blocks.num_blocks());
    }

    #[test]
    fn test_is_set() {
        let mut blocks = BlockArrangement::new();
        assert!(blocks.is_set(&Point3D::new(0,0,0)));
        let p = Point3D::new(1,0,0);
        blocks.add_block_at(&p).expect("Checked coordinates.");
        assert!(blocks.is_set(&p));
        let p = Point3D::new(2,0,0);
        blocks.add_block_at(&p).expect("Checked coordinates.");
        assert!(blocks.is_set(&p));
        let p = Point3D::new(0,0,-1);
        blocks.add_block_at(&p).expect("Checked coordinates.");
        assert!(blocks.is_set(&p));
        let p = Point3D::new(0,-1,-1);
        blocks.add_block_at(&p).expect("Checked coordinates.");
        assert!(blocks.is_set(&p));
        let p = Point3D::new(0,-1,0);
        blocks.add_block_at(&p).expect("Checked coordinates.");
        assert!(blocks.is_set(&p));
    }

    #[test]
    fn test_x_mirroring() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.orientation_mut().mirror(Axis3D::X);
        assert!(!blocks.is_set(&Point3D::new(1,0,0)));
        assert!(blocks.is_set(&Point3D::new(0,0,0)));
        assert!(blocks.is_set(&Point3D::new(-1,0,0)));
        assert!(blocks.is_set(&Point3D::new(-2,0,0)));
    }

    #[test]
    fn test_eq_with_rotations() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,0,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        OrientationIterator::default().enumerate().for_each(|(index, orientation)| {
            clone.set_orientation(orientation);
            assert_eq!(blocks, clone, "Blocks do not equal at index {index} with orientation {orientation:?}");
        })
    }

    #[test]
    fn test_eq_with_x_mir() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,0,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.mirror(Axis3D::X);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }

    #[test]
    fn test_eq_with_y_mir() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(0, 1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(0, 2,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(1, 3,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(0, 3,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.mirror(Axis3D::X);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }

    #[test]
    fn test_eq_with_z_mir() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(0,0, 1)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(0,0, 2)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(1,0, 3)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(0,1, 3)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.mirror(Axis3D::X);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }

    #[test]
    fn test_eq_with_x_rot() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,0,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.rotate(Axis3D::X, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::X, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::X, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::X, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }

    #[test]
    fn test_eq_with_y_rot() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,0,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.rotate(Axis3D::Y, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Y, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Y, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Y, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }

    #[test]
    fn test_eq_with_z_rot() {
        let mut blocks = BlockArrangement::new();
        blocks.add_block_at(&Point3D::new(1,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(2,0,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,1,0)).expect("Checked coordinates.");
        blocks.add_block_at(&Point3D::new(3,0,1)).expect("Checked coordinates.");
        let mut clone = blocks.clone();
        let mut o = Orientation::default();
        o.rotate(Axis3D::Z, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Z, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Z, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
        o.rotate(Axis3D::Z, RotationAmount::Ninety);
        clone.set_orientation(o);
        assert_eq!(blocks, clone);
    }
}