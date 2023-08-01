use std::array::IntoIter;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use getset::{CopyGetters, MutGetters, Setters};
use strum::EnumIter;
use crate::point::Axis3D;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
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
    pub fn additive_complement(&self) -> Self {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Default, Hash)]
pub enum RotationAmount {
    #[default]
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl RotationAmount {

    /// Returns the inverse [RotationAmount] so that adding it it to self will result in
    /// [RotationAmount::Zero].
    pub fn inverse(&self) -> Self {
        Self::Zero - *self
    }
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

/// An iterator iterating over every possible orientation.
#[derive(Debug)]
pub struct OrientationIterator {
    y_mir_prev: Option<bool>,
    x_mir_prev: Option<bool>,
    z_rot_prev: Option<RotationAmount>,
    y_rot_prev: Option<RotationAmount>,
    x_rot_prev: Option<RotationAmount>,
    z_mir_iter: IntoIter<bool, 2>,
    y_mir_iter: IntoIter<bool, 2>,
    x_mir_iter: IntoIter<bool, 2>,
    z_rot_iter: IntoIter<RotationAmount, 4>,
    y_rot_iter: IntoIter<RotationAmount, 4>,
    x_rot_iter: IntoIter<RotationAmount, 4>,
}

impl OrientationIterator {
    // Helper function to create an iterator for RotationAmount.
    fn rotation_amount_iter() -> IntoIter<RotationAmount, 4> {
        [
            RotationAmount::Zero,
            RotationAmount::Ninety,
            RotationAmount::OneEighty,
            RotationAmount::TwoSeventy,
        ].into_iter()
    }

    // Helper function to create an iterator for bool (mirroring).
    fn mirroring_iter() -> IntoIter<bool, 2> {
        [false, true].into_iter()
    }
}

impl Default for OrientationIterator {
    fn default() -> Self {
        OrientationIterator {
            y_mir_prev: None,
            x_mir_prev: None,
            z_rot_prev: None,
            y_rot_prev: None,
            x_rot_prev: None,
            x_rot_iter: OrientationIterator::rotation_amount_iter(),
            y_rot_iter: OrientationIterator::rotation_amount_iter(),
            z_rot_iter: OrientationIterator::rotation_amount_iter(),
            x_mir_iter: OrientationIterator::mirroring_iter(),
            y_mir_iter: OrientationIterator::mirroring_iter(),
            z_mir_iter: OrientationIterator::mirroring_iter(),
        }
    }
}

// Implement the Iterator trait for OrientationIterator.
impl Iterator for OrientationIterator {
    type Item = Orientation;

    fn next(&mut self) -> Option<Self::Item> {
        // Try to get the next combination of values.

        let (z_mir, y_mir_o) = if let Some(z_mir) = self.z_mir_iter.next() {
            (z_mir, self.y_mir_prev.or_else(|| self.y_mir_iter.next()))
        } else {
            self.z_mir_iter = Self::mirroring_iter();
            (self.z_mir_iter.next().expect("Expect newly created iterator to have at least one value."), self.y_mir_iter.next())
        };

        macro_rules! cascading_fun {
            ($option_val:expr, $val_prev_ident:ident, $val_iter_ident:ident, $eval_prev_ident:ident, $eval_iter_ident:ident, $generator_fn:expr) => {
                if let Some(value) = $option_val {
                    self.$val_prev_ident = Some(value);
                    (value, self.$eval_prev_ident.or_else(|| self.$eval_iter_ident.next()))
                } else {
                    self.$val_iter_ident = $generator_fn;
                    let new_value = self.$val_iter_ident.next().expect("Expect newly created iterator to have at least one value.");
                    self.$val_prev_ident = Some(new_value);
                    (new_value, self.$eval_iter_ident.next())
                }
            };
        }

        let (y_mir, x_mir_o) = cascading_fun!(
            y_mir_o,
            y_mir_prev,
            y_mir_iter,
            x_mir_prev,
            x_mir_iter,
            Self::mirroring_iter()
        );

        let (x_mir, z_rot_o) = cascading_fun!(
            x_mir_o,
            x_mir_prev,
            x_mir_iter,
            z_rot_prev,
            z_rot_iter,
            Self::mirroring_iter()
        );

        let (z_rot, y_rot_o) = cascading_fun!(
            z_rot_o,
            z_rot_prev,
            z_rot_iter,
            y_rot_prev,
            y_rot_iter,
            Self::rotation_amount_iter()
        );

        let (y_rot, x_rot_o) = cascading_fun!(
            y_rot_o,
            y_rot_prev,
            y_rot_iter,
            x_rot_prev,
            x_rot_iter,
            Self::rotation_amount_iter()
        );

        let x_rot = x_rot_o?;
        self.x_rot_prev = Some(x_rot);

        Some(Orientation {
            x_rot,
            y_rot,
            z_rot,
            x_mir,
            y_mir,
            z_mir,
        })
    }
}

#[cfg(test)]
mod orientation_iter_tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_iter() {
        let itr = OrientationIterator::default();
        let set: HashSet<_> = OrientationIterator::default().collect();
        assert_eq!(512, set.len());
    }
}