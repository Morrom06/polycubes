use fixedbitset::FixedBitSet;
use getset::CopyGetters;
use rust_decimal::Decimal;
use crate::mapper::{Mapper};
use crate::orientation::Orientation;
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
    offset: Point3D<i32>,
    mapper: Mapper,
}

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
            offset: Point3D::default(),
            mapper: Mapper::new(dim)
        };
        arr.set_origin_block();
        arr
    }

    pub fn add_block_at(&mut self, point: Point3D<i32>) -> Result<(), PlacementError> {
        if !self.has_neighbors(point) {
            return Err(PlacementError::NotAdjacentToBlock);
        }
        if self.num_blocks + 1 > self.mapper.dimension().arm_size() as u8 {
            self.grow((self.num_blocks + 1) as usize)
        }
        let index = self.mapper.unresolve(point)
            .unwrap_or_else(|| panic!("Expected a save resolve from point {point} but was unsafe."));
        self.bitset.set(index, true);
        self.num_blocks += 1;
        self.recenter();
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
    fn has_neighbors(&self, point: Point3D<i32>) -> bool {
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
        NEIGHBOR_OFFSETS.iter().cloned().map(|offset| offset + point)
            .map(|coordinate| coordinate - self.offset)
            // Resolves the point to the corresponding index and filters only in bound indices.
            .filter_map(|coordinate| self.mapper.unresolve(coordinate))
            .any(|i| self.bitset[i])
    }

    /// Centers the center of mass of the block formation to the coordinate origin.
    fn recenter(&mut self) {
        let center = self.center_of_mass();
        if center == Point3D::default() {
            return;
        }
        self.offset = self.offset + center;
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
        self.iter()
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

    /// Returns an iterator over the coordinates of the blocks.
    pub fn iter(&self) -> impl Iterator<Item = Point3D<i32>> + '_ {
        self.bitset.ones()
            .map(|index| self.mapper.resolve(index).expect("Expected save conversion") + self.offset)
    }

    /// Calculates the density of the blocks.
    /// It is the average distance to the origin.
    pub fn density(&self) -> Decimal {
        let sum: Decimal = self.iter()
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
        let sum: Decimal = self.iter()
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
}