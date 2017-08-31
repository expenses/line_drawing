use Voxel;
use steps::Steps;
use std::cmp::max;

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
pub struct Bresenham3d {
    sign_x: isize,
    sign_y: isize,
    sign_z: isize,
    err_x: isize,
    err_y: isize,
    err_z: isize,
    len_x: isize,
    len_y: isize,
    len_z: isize,
    longest: isize,
    count: isize,
    voxel: Voxel<isize>,
}

impl Bresenham3d {
    #[inline]
    pub fn new(start: Voxel<isize>, end: Voxel<isize>) -> Bresenham3d {
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
            err_x: longest / 2,
            err_y: longest / 2,
            err_z: longest / 2,
            sign_x: delta_x.signum(),
            sign_y: delta_y.signum(),
            sign_z: delta_z.signum(),
            voxel: start
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Voxel<isize>, Bresenham3d> {
        Steps::new(self)
    }
}

impl Iterator for Bresenham3d {
    type Item = Voxel<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 0 {
            self.count -= 1;
            self.err_x -= self.len_x;
            self.err_y -= self.len_y; 
            self.err_z -= self.len_z; 

            let voxel = self.voxel;
            
            if self.err_x < 0 {
                self.err_x += self.longest;
                self.voxel.0 += self.sign_x;
            }
            
            if self.err_y < 0 {
                self.err_y += self.longest;
                self.voxel.1 += self.sign_y;
            }

            if self.err_z < 0 {
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
pub fn bresenham_3d(start: Voxel<isize>, end: Voxel<isize>) -> Vec<Voxel<isize>> {
    Bresenham3d::new(start, end).collect()
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