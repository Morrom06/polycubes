use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use rust_decimal::Decimal;
use strum::EnumIter;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[derive(Setters, MutGetters, Getters)]
pub struct Point3D<T> {
    #[getset(get = "pub", get_copy = "pub", set = "pub", get_mut = "pub")]
    x: T,
    #[getset(get = "pub", get_copy = "pub", set = "pub", get_mut = "pub")]
    y: T,
    #[getset(get = "pub", get_copy = "pub", set = "pub", get_mut = "pub")]
    z: T,
}

macro_rules! num_funcs_for_point {
    ($num_type:ty) => {
        use crate::orientation::*;
        impl Point3D<$num_type> {

            /// Performs a clockwise 90 degree 2 dimensional rotation.
            fn rotate_2d(x: &mut $num_type, y: &mut $num_type) {
                let x_copy = *x;
                *x = -*y;
                *y = x_copy;
            }

            pub fn apply_orientation(&mut self, orientation: &Orientation) {
                if orientation.x_mir() {
                    self.mirror(Axis3D::X)
                }
                if orientation.y_mir() {
                    self.mirror(Axis3D::Y)
                }
                if orientation.z_mir() {
                    self.mirror(Axis3D::Z)
                }
                self.rotate(Axis3D::X, orientation.x_rot());
                self.rotate(Axis3D::Y, orientation.y_rot());
                self.rotate(Axis3D::Z, orientation.z_rot());
            }

            /// Applies the orientation inverse so that if it was previously applied
            /// it will no be reversed.
            pub fn apply_inverse_orientation(&mut self, orientation: &Orientation) {
                self.rotate(Axis3D::Z, orientation.z_rot().inverse());
                self.rotate(Axis3D::Y, orientation.y_rot().inverse());
                self.rotate(Axis3D::X, orientation.x_rot().inverse());

                if orientation.z_mir() {
                    self.mirror(Axis3D::Z)
                }
                if orientation.y_mir() {
                    self.mirror(Axis3D::Y)
                }
                if orientation.x_mir() {
                    self.mirror(Axis3D::X)
                }
            }

            pub fn rotate(&mut self, axis: Axis3D, amount: RotationAmount) {
                let rotations = match amount {
                    RotationAmount::Zero => {return;}
                    RotationAmount::Ninety => {1}
                    RotationAmount::OneEighty => {2}
                    RotationAmount::TwoSeventy => {3}
                };
                let (x_ref, y_ref) = match axis {
                    Axis3D::X => {
                        (&mut self.y, &mut self.z)
                    }
                    Axis3D::Y => {
                        (&mut self.x, &mut self.z)
                    }
                    Axis3D::Z => {
                        (&mut self.x, &mut self.y)
                    }
                };
                for _i in 0..rotations {
                    Self::rotate_2d(x_ref, y_ref);
                }
            }

            pub fn mirror(&mut self, axis: Axis3D) {
                match axis {
                    Axis3D::X => {
                        self.x = -self.x;
                    }
                    Axis3D::Y => {
                        self.y = -self.y;
                    }
                    Axis3D::Z => {
                        self.z = -self.z;
                    }
                }
            }

            /// Calculates the distance to the origin.
            pub fn distance_to_origin(&self) -> Decimal {
                let square_sum = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
                let sqroot = f64::sqrt(square_sum as f64);
                use rust_decimal::prelude::FromPrimitive;
                Decimal::from_f64(sqroot).expect("This is a save conversion since the result of sqrt is expected to be save")
            }

        }
    };
}

num_funcs_for_point!(i32);

impl<T: Add<Output = T>> Add for Point3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl<T> Point3D<T> {

    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn map_all<U, F: FnMut(T) -> U>(self, mut f: F) -> Point3D<U> {
        Point3D {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl<T> From<(T, T, T)> for Point3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self {
            x, y, z
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[derive(EnumIter)]
pub enum Axis3D {
    X, Y, Z
}

#[cfg(test)]
mod point_tests {
    use crate::orientation::RotationAmount::TwoSeventy;
    use super::*;

    #[test]
    fn test_default_point() {
        let p = Point3D::default();
        assert_eq!(Point3D::new(0,0,0), p);
    }

    #[test]
    fn test_mirroring() {
        let mut p = Point3D::new(1,1,1);
        p.mirror(Axis3D::X);
        assert_eq!(Point3D::new(-1,1,1), p);
        p.mirror(Axis3D::X);
        assert_eq!(Point3D::new(1,1,1), p);
        p.mirror(Axis3D::Y);
        assert_eq!(Point3D::new(1,-1,1), p);
        p.mirror(Axis3D::Y);
        assert_eq!(Point3D::new(1,1,1), p);
        p.mirror(Axis3D::Z);
        assert_eq!(Point3D::new(1,1,-1), p);
        p.mirror(Axis3D::Z);
        assert_eq!(Point3D::new(1,1,1), p);

        let mut p = Point3D::new(0,0,0);
        p.mirror(Axis3D::X);
        assert_eq!(Point3D::new(0,0,0), p);
        p.mirror(Axis3D::X);
        assert_eq!(Point3D::new(0,0,0), p);
        p.mirror(Axis3D::Y);
        assert_eq!(Point3D::new(0,0,0), p);
        p.mirror(Axis3D::Y);
        assert_eq!(Point3D::new(0,0,0), p);
        p.mirror(Axis3D::Z);
        assert_eq!(Point3D::new(0,0,0), p);
        p.mirror(Axis3D::Z);
        assert_eq!(Point3D::new(0,0,0), p);
    }

    #[test]
    fn test_apply_inverse() {
        use crate::orientation::RotationAmount::*;
        let p = Point3D::new(1,2,3);
        let mut p_clone = p;
        let mut orientation = Orientation::default();
        orientation
            .set_x_mir(true)
            .set_y_mir(true)
            .set_x_rot(Ninety)
            .set_z_rot(TwoSeventy);
        p_clone.apply_orientation(&orientation);
        p_clone.apply_inverse_orientation(&orientation);
        assert_eq!(p, p_clone)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[derive(CopyGetters)]
pub struct Finite3DDimension {
    #[get_copy = "pub"]
    arm_size: usize,
}

impl Finite3DDimension {
    /// Returns a new dimension.
    /// Size specifies the length along the 3 axis away from the origin.
    pub fn new(size: usize) -> Self {
        Self {
            arm_size: size
        }
    }

    /// The number of points contained in this dimension.
    pub fn size(&self) -> usize {
        let dim_size = self.arm_size * 2;
        dim_size * dim_size * dim_size
    }

    pub fn in_bounds(&self, p: &Point3D<i32>) -> bool {
        -(self.arm_size as i32) <= *p.x() && *p.x() <= self.arm_size as i32 &&
            -(self.arm_size as i32) <= *p.y() && *p.y() <= self.arm_size as i32 &&
            -(self.arm_size as i32) <= *p.z() && *p.z() <= self.arm_size as i32
    }
}

#[cfg(test)]
mod dimension_tests {
    use super::*;

    #[test]
    fn test_in_bounds() {
        let dim = Finite3DDimension::new(3);
        for x in -3..4 {
            for y in -3..4 {
                for z in -3..4 {
                    let p = Point3D::new(x,y,z);
                    assert!(dim.in_bounds(&p), "In bounds check failed at point {p}")
                }
            }
        }
    }
}