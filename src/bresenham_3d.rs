use {Voxel, SignedNum, sort_voxels, collect_vec_deque};
use steps::Steps;
use std::cmp::max;
use std::collections::VecDeque;

/// An 3-D implementation of bresenham, sourced from [this site].
///
/// It includes both the start and end point and is asymmetrical.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::Bresenham3d; 
///
/// fn main() {
///     for (x, y, z) in Bresenham3d::new((0, 0, 0), (5, 6, 7)) {
///         print!("({}, {}, {}), ", x, y, z);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0, 0), (1, 1, 1), (1, 2, 2), (2, 3, 3), (3, 3, 4), (4, 4, 5), (4, 5, 6), (5, 6, 7),
/// ```
///
/// [this site]: http://members.chello.at/~easyfilter/bresenham.html
pub struct Bresenham3d<T> {
    sign_x: T,
    sign_y: T,
    sign_z: T,
    err_x: T,
    err_y: T,
    err_z: T,
    len_x: T,
    len_y: T,
    len_z: T,
    longest: T,
    count: T,
    voxel: Voxel<T>,
}

impl<T: SignedNum> Bresenham3d<T> {
    #[inline]
    pub fn new(start: Voxel<T>, end: Voxel<T>) -> Bresenham3d<T> {
        let delta_x = end.0 - start.0;
        let delta_y = end.1 - start.1;
        let delta_z = end.2 - start.2;

        let len_x = delta_x.abs();
        let len_y = delta_y.abs();
        let len_z = delta_z.abs();

        let longest = max(len_x, max(len_y, len_z));

        Bresenham3d {
            len_x, len_y, len_z, longest,
            count: longest,
            err_x: longest / T::cast(2),
            err_y: longest / T::cast(2),
            err_z: longest / T::cast(2),
            sign_x: delta_x.signum(),
            sign_y: delta_y.signum(),
            sign_z: delta_z.signum(),
            voxel: start
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Voxel<T>, Bresenham3d<T>> {
        Steps::new(self)
    }
}

impl<T: SignedNum> Iterator for Bresenham3d<T> {
    type Item = Voxel<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= T::zero() {
            self.count -= T::one();
            self.err_x -= self.len_x;
            self.err_y -= self.len_y; 
            self.err_z -= self.len_z; 

            let voxel = self.voxel;
            
            if self.err_x < T::zero() {
                self.err_x += self.longest;
                self.voxel.0 += self.sign_x;
            }
            
            if self.err_y < T::zero() {
                self.err_y += self.longest;
                self.voxel.1 += self.sign_y;
            }

            if self.err_z < T::zero() {
                self.err_z += self.longest;
                self.voxel.2 += self.sign_z;
            }

            Some(voxel)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`Bresenham3d`] into a [`Vec`].
/// [`Bresenham3d`]: struct.Bresenham3d.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[inline]
pub fn bresenham_3d<T: SignedNum>(start: Voxel<T>, end: Voxel<T>) -> Vec<Voxel<T>> {
    Bresenham3d::new(start, end).collect()
}

/// Sorts the voxels before hand to ensure that the line is symmetrical and collects into a
/// [`VecDeque`].
/// [`VecDeque`]: https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html
#[inline]
pub fn bresenham_3d_sorted<T: SignedNum>(start: Voxel<T>, end: Voxel<T>) -> VecDeque<Voxel<T>> {
    let (start, end, reordered) = sort_voxels(start, end);
    collect_vec_deque(Bresenham3d::new(start, end), reordered)
}

#[test]
fn tests() {
    assert_eq!(
        bresenham_3d((0, 0, 0), (5, 5, 5)),
        [(0, 0, 0), (1, 1, 1), (2, 2, 2), (3, 3, 3), (4, 4, 4), (5, 5, 5)]
    );

    assert_eq!(
        Bresenham3d::new((0, 0, 0), (500, 678, 1000)).count(),
        1001
    );

    assert_eq!(
        Bresenham3d::new((500, 678, 1000), (0, 0, 0)).count(),
        1001
    );
}