use std::usize;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use getset::{CopyGetters, MutGetters, Setters};
use strum::EnumIter;
use crate::point::{Axis3D, Finite3DDimension, Point3D};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(CopyGetters, Setters, MutGetters)]
pub struct Mapper {
    #[getset(get_copy = "pub", set = "pub")]
    dimension: Finite3DDimension,
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    orientation: Orientation,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[derive(CopyGetters, MutGetters, Setters)]
pub struct Orientation {
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    x_rot: RotationAmount,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    y_rot: RotationAmount,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    z_rot: RotationAmount,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    x_mir: bool,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    y_mir: bool,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    z_mir: bool,
}

impl Add for Orientation {
    type Output = Orientation;

    fn add(mut self, rhs: Self) -> Self::Output {

        self.x_mir ^= rhs.x_mir;
        self.y_mir ^= rhs.y_mir;
        self.z_mir ^= rhs.z_mir;

        self.x_rot += rhs.x_rot;
        self.y_rot += rhs.y_rot;
        self.z_rot += rhs.z_rot;

        self
    }
}

impl Orientation {
    /// Returns
    /// An [Orientation] that when added to the input will result in the default orientation.
    pub fn negative(&self) -> Self {
        Self {
            x_mir: self.x_mir,
            y_mir: self.y_mir,
            z_mir: self.z_mir,
            x_rot: RotationAmount::Zero - self.x_rot,
            y_rot: RotationAmount::Zero - self.y_rot,
            z_rot: RotationAmount::Zero - self.z_rot,
        }
    }

    pub fn rotate(&mut self, axis: Axis3D, amount: RotationAmount) {
        match axis {
            Axis3D::X => {self.set_x_rot(self.x_rot() + amount)}
            Axis3D::Y => {self.set_y_rot(self.y_rot() + amount)}
            Axis3D::Z => {self.set_z_rot(self.z_rot() + amount)}
        };
    }

    pub fn mirror(&mut self, axis: Axis3D) {
        match axis {
            Axis3D::X => {self.set_x_mir(!self.x_mir())}
            Axis3D::Y => {self.set_y_mir(!self.y_mir())}
            Axis3D::Z => {self.set_z_mir(!self.z_mir())}
        };
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Default)]
pub enum RotationAmount {
    #[default]
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl SubAssign for RotationAmount {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub for RotationAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use RotationAmount::*;
        match (self, rhs) {
            (_, Zero) => self,
            (Ninety, Ninety) | (OneEighty, OneEighty) | (TwoSeventy, TwoSeventy) => Zero,
            (Zero, TwoSeventy) |(OneEighty, Ninety) | (TwoSeventy, OneEighty) => Ninety,
            (Zero, OneEighty) | (Ninety, TwoSeventy) | (TwoSeventy, Ninety) => OneEighty,
            (Zero, Ninety) | (Ninety, OneEighty) | (OneEighty, TwoSeventy) => TwoSeventy,
        }
    }
}

impl AddAssign for RotationAmount {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add for RotationAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use RotationAmount::*;
        match (self, rhs) {
            (Zero, _) => rhs,
            (_, Zero) => self,
            (Ninety, Ninety) => OneEighty,
            (Ninety, OneEighty) | (OneEighty, Ninety) => TwoSeventy,
            (OneEighty, OneEighty) | (TwoSeventy, Ninety) | (Ninety, TwoSeventy) => Zero,
            (TwoSeventy, TwoSeventy) => OneEighty,
            (OneEighty, TwoSeventy) | (TwoSeventy, OneEighty) => Ninety,
        }
    }
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
        if !self.dimension.in_bounds(point) {
            return None;
        }

        point.apply_orientation(&self.orientation.negative());

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
        if self.dimension.in_bounds(p) {
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