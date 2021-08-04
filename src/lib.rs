//! A collection of line-drawing algorithms for use in graphics and video games.
//!
//! Currently implemented:
//!
//! * [`Bresenham`] - An implementation of [Bresenham's line algorithm].
//! * [`Bresenham3d`] - A 3-Dimensional implementation of bresenham.
//! * [`BresenhamCircle`] - Bresenham's circle algorithm.
//! * [`Midpoint`] - The [mid-point line algorithm].
//! * [`WalkGrid`] and [`Supercover`] - implemented from [this article by Red Blob Games][article].
//! * [`WalkVoxels`] - A similar 3-Dimensional algorithm that only takes orthogonal steps.
//! * [`XiaolinWu`] - [Xiaolin Wu's line algorithm].
//!
//! [`Bresenham`]: struct.Bresenham.html
//! [Bresenham's line algorithm]: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
//! [`Bresenham3d`]: struct.Bresenham3d.html
//! [`BresenhamCircle`]: struct.BresenhamCircle.html
//! [`Midpoint`]: struct.Midpoint.html
//! [mid-point line algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
//! [`WalkGrid`]: struct.WalkGrid.html
//! [`Supercover`]: struct.Supercover.html
//! [article]: http://www.redblobgames.com/grids/line-drawing.html
//! [`XiaolinWu`]: struct.XiaolinWu.html
//! [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
//! [`WalkVoxels`]: struct.WalkVoxels.html

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    unused_extern_crates
)]
#![cfg_attr(not(test), no_std)]

pub mod octant;
pub mod steps;

mod bresenham;
mod bresenham_3d;
mod bresenham_circle;
mod fuzzing;
mod grid_walking;
mod midpoint;
mod walk_voxels;
mod xiaolin_wu;

pub use crate::bresenham::*;
pub use crate::bresenham_3d::*;
pub use crate::bresenham_circle::*;
pub use crate::grid_walking::*;
pub use crate::midpoint::*;
pub use crate::walk_voxels::*;
pub use crate::xiaolin_wu::*;

use num_traits::{Float, NumAssignOps, NumCast, Signed};

/// A point in 2D space.
pub type Point<T> = (T, T);
/// An point in 3D space.
pub type Voxel<T> = (T, T, T);

/// All the floating-point primitives.
pub trait FloatNum: Float + NumAssignOps {
    #[inline]
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}

impl<T: Float + NumAssignOps> FloatNum for T {}

/// All the signed integer primitives.
pub trait SignedNum: Signed + Ord + Copy + NumCast + NumAssignOps {
    #[inline]
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}

impl<T: Signed + Ord + Copy + NumCast + NumAssignOps> SignedNum for T {}
