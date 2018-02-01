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

extern crate num_traits;

pub mod steps;
pub mod octant;

mod bresenham;
mod midpoint;
mod xiaolin_wu;
mod grid_walking;
mod fuzzing;
mod bresenham_3d;
mod walk_voxels;
mod bresenham_circle;

pub use bresenham::*;
pub use midpoint::*;
pub use xiaolin_wu::*;
pub use grid_walking::*;
pub use bresenham_3d::*;
pub use walk_voxels::*;
pub use bresenham_circle::*;

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
