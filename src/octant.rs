//! A simple octant struct for transforming line points.

use crate::Point;
use core::ops::{Neg, Sub};
use num_traits::Zero;

/// A simple octant struct for transforming line points.
pub struct Octant {
    value: u8,
}

impl Octant {
    #[inline]
    /// Get the relevant octant from a start and end point.
    pub fn new<T>(start: Point<T>, end: Point<T>) -> Self
    where
        T: Sub<Output = T> + Neg<Output = T> + PartialOrd + Zero,
    {
        let mut value = 0;
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        if dy < T::zero() {
            dx = -dx;
            dy = -dy;
            value += 4;
        }

        if dx < T::zero() {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            value += 2
        }

        if dx < dy {
            value += 1
        }

        Self { value }
    }

    /// Convert a point to its position in the octant.
    #[inline]
    pub fn to<T: Neg<Output = T>>(&self, point: Point<T>) -> Point<T> {
        match self.value {
            0 => (point.0, point.1),
            1 => (point.1, point.0),
            2 => (point.1, -point.0),
            3 => (-point.0, point.1),
            4 => (-point.0, -point.1),
            5 => (-point.1, -point.0),
            6 => (-point.1, point.0),
            7 => (point.0, -point.1),
            _ => unreachable!(),
        }
    }

    /// Convert a point from its position in the octant.
    #[inline]
    pub fn from<T: Neg<Output = T>>(&self, point: Point<T>) -> Point<T> {
        match self.value {
            0 => (point.0, point.1),
            1 => (point.1, point.0),
            2 => (-point.1, point.0),
            3 => (-point.0, point.1),
            4 => (-point.0, -point.1),
            5 => (-point.1, -point.0),
            6 => (point.1, -point.0),
            7 => (point.0, -point.1),
            _ => unreachable!(),
        }
    }
}
