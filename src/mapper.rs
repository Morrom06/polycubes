use std::usize;
use getset::{CopyGetters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use crate::orientation::Orientation;
use crate::point::{Finite3DDimension, Point3D};

#[derive(Debug, Eq, PartialEq, Clone)]
#[derive(CopyGetters, Setters, MutGetters)]
#[derive(Serialize, Deserialize)]
pub struct Mapper {
    #[getset(get_copy = "pub", set = "pub")]
    dimension: Finite3DDimension,
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    orientation: Orientation,
}

impl Mapper {

    pub fn new(dim: Finite3DDimension) -> Self {
        Self {
            dimension: dim,
            orientation: Default::default(),
        }
    }

    pub fn unresolve(&self, mut point: Point3D<i32>) -> Option<usize> {
        point.apply_inverse_orientation(&self.orientation);
        if !self.dimension.in_bounds(&point) {
            return None;
        }

        let u_point = point.map_each(|x_val| {
            (x_val + self.dimension.x_neg() as i32) as usize
        }, |y_val| {
            (y_val + self.dimension.y_neg() as i32) as usize
        }, |z_val| {
            (z_val + self.dimension.z_neg() as i32) as usize
        });

        let (width, depth, _height) = self.dimension().all_axis_len();

        let index = u_point.x() + width as usize * (u_point.y() + (depth) as usize * u_point.z());

        Some(index)
    }

    pub fn resolve(&self, index: usize) -> Option<Point3D<i32>> {
        let (width, depth, _height) = self.dimension().all_axis_len();

        let z = (index / (width * depth) as usize) as i32 - self.dimension().z_neg() as i32;
        let y = ((index / width as usize) % depth as usize) as i32  - self.dimension().y_neg() as i32;
        let x = (index % width as usize) as i32 - self.dimension().x_neg() as i32;

        let mut p = Point3D::from((x, y, z));
        if self.dimension.in_bounds(&p) {
            p.apply_orientation(&self.orientation);
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
    fn test_mapping_small() {
        let dim = Finite3DDimension::new(1, 1, 1, 1, 1, 1);
        let mapper = Mapper::new(dim);
        for i in 0..dim.size() as usize {
            let point = mapper.resolve(i).unwrap_or_else(|| panic!("Expected save resolving of index {i}"));
            assert!(dim.in_bounds(&point));
            let resolved_index = mapper.unresolve(point).unwrap_or_else(|| panic!("Expected save unresolve of point {point}"));
            assert_eq!(i, resolved_index, "The expected index of {i} was not converted back, but got {resolved_index} and point {point}")
        }
    }

    #[test]
    fn test_mapping_medium() {
        let dim = Finite3DDimension::new(5, 3, 7, 9, 11, 13);
        let mapper = Mapper::new(dim);
        for i in 0..dim.size() as usize {
            let point = mapper.resolve(i).unwrap_or_else(|| panic!("Expected save resolving of index {i}"));
            assert!(dim.in_bounds(&point));
            let resolved_index = mapper.unresolve(point).unwrap_or_else(|| panic!("Expected save unresolve of point {point}"));
            assert_eq!(i, resolved_index, "The expected index of {i} was not converted back, but got {resolved_index} and point {point}")
        }
    }

    #[test]
    #[ignore]
    fn test_mapping_large() {
        let dim = Finite3DDimension::new(10, 15, 18, 19, 13, 11);
        let mapper = Mapper::new(dim);
        for i in 0..dim.size() as usize {
            let point = mapper.resolve(i).unwrap_or_else(|| panic!("Expected save resolving of index {i}"));
            assert!(dim.in_bounds(&point));
            let resolved_index = mapper.unresolve(point).unwrap_or_else(|| panic!("Expected save unresolve of point {point}"));
            assert_eq!(i, resolved_index, "The expected index of {i} was not converted back, but got {resolved_index} and point {point}")
        }
    }
}