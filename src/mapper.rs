use std::usize;
use getset::{CopyGetters, MutGetters, Setters};
use crate::orientation::Orientation;
use crate::point::{Finite3DDimension, Point3D};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(CopyGetters, Setters, MutGetters)]
pub struct Mapper {
    #[getset(get_copy = "pub", set = "pub")]
    dimension: Finite3DDimension,
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    orientation: Orientation,
}


/// Maps the positive to all integers by flipping it to negative on every even number.
/// ```
/// // 0, 1,  2,  3,  4
/// // 0, 1, -1,  2, -2
/// assert_eq!(0 , double_invert(0));
/// assert_eq!(1 , double_invert(1));
/// assert_eq!(-1 , double_invert(2));
/// assert_eq!(2 , double_invert(3));
/// assert_eq!(-2 , double_invert(4));
/// ```
fn double_invert(n: usize) -> isize {
    let half = n / 2;
    if n % 2 == 0 {
        -(half as isize)
    } else {
        (half + 1) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_invert() {
        assert_eq!(0 , double_invert(0));
        assert_eq!(1 , double_invert(1));
        assert_eq!(-1 , double_invert(2));
        assert_eq!(2 , double_invert(3));
        assert_eq!(-2 , double_invert(4));
    }
}

impl Mapper {

    pub fn new(dim: Finite3DDimension) -> Self {
        Self {
            dimension: dim,
            orientation: Default::default(),
        }
    }

    pub fn unresolve(&self, mut point: Point3D<i32>) -> Option<usize> {
        if !self.dimension.in_bounds(&point) {
            return None;
        }
        point.apply_orientation(&self.orientation.inverse());

        let u_point = point.map_all(|i_val| {
            (i_val + self.dimension.arm_size() as i32) as usize
        });

        // Since size only specifies one direction awy from origin
        let width_height_depth = self.dimension.arm_size() * 2;

        return Some(u_point.x() + width_height_depth * (u_point.y() + width_height_depth * u_point.z()));
    }

    pub fn resolve(&self, index: usize) -> Option<Point3D<i32>> {
        let width_height_depth = self.dimension.arm_size() * 2;

        let z = index / (width_height_depth * width_height_depth);
        let y = (index / width_height_depth) % width_height_depth;
        let x = index % width_height_depth;

        let p = Point3D::from((x, y, z));
        let mut p = p.map_all(|u_val| u_val as i32 - self.dimension.arm_size() as i32);
        p.apply_orientation(&self.orientation);
        if self.dimension.in_bounds(&p) {
            Some(p)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod mapper_tests {
    use super::*;

    #[test]
    fn test_mapping() {
        let arm_size = 5;
        let dim = Finite3DDimension::new(arm_size);
        let mapper = Mapper::new(dim);
        for i in 0..dim.size() {
            let point = mapper.resolve(i).expect("Save");
            let resolved_index = mapper.unresolve(point).expect("Save");
            assert_eq!(i, resolved_index)
        }
    }
}