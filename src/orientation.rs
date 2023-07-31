use std::ops::{Add, AddAssign, Sub, SubAssign};
use getset::{CopyGetters, MutGetters, Setters};
use strum::EnumIter;
use crate::point::Axis3D;

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