use {FloatNum, SignedNum, Voxel};
use steps::Steps;

#[inline]
fn compare<T: SignedNum>(a: T, b: T) -> T {
    if a > b {
        T::one()
    } else if a == b {
        T::zero()
    } else {
        -T::one()
    }
}

/// Where the center of the voxel is, at the center or a corner.
///
/// Generally you want `Center`.
pub enum VoxelOrigin {
    Corner,
    Center,
}

impl VoxelOrigin {
    #[inline]
    /// Round a voxel's position based on the origin.
    pub fn round<I: FloatNum, O: SignedNum>(&self, voxel: Voxel<I>) -> Voxel<O> {
        let (x, y, z) = match *self {
            VoxelOrigin::Corner => (voxel.0.floor(), voxel.1.floor(), voxel.2.floor()),
            VoxelOrigin::Center => (voxel.0.round(), voxel.1.round(), voxel.2.round()),
        };

        (O::cast(x), O::cast(y), O::cast(z))
    }
}

/// Walk between two voxels, taking orthogonal steps and visiting all voxels in between.
///
/// Implemented from [this Stack Overflow answer].
/// This algorithm takes floating-point numbers as input and should be symmetrical.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::{VoxelOrigin, WalkVoxels};
/// 
/// fn main() {
///     let a = (0.0, 0.0, 0.0);
///     let b = (5.0, 6.0, 7.0);
///
///     for (i, (x, y, z)) in WalkVoxels::<f32, i8>::new(a, b, &VoxelOrigin::Center).enumerate() {
///         if i > 0 && i % 5 == 0 {
///             println!();
///         }
///         print!("({}, {}, {}), ", x, y, z);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0, 0), (0, 0, 1), (0, 1, 1), (1, 1, 1), (1, 1, 2),
/// (1, 2, 2), (2, 2, 2), (2, 2, 3), (2, 3, 3), (2, 3, 4),
/// (3, 3, 4), (3, 4, 4), (3, 4, 5), (4, 4, 5), (4, 5, 5),
/// (4, 5, 6), (4, 5, 7), (4, 6, 7), (5, 6, 7),
/// ```
///
/// [this Stack Overflow answer]: https://stackoverflow.com/a/16507714
pub struct WalkVoxels<I, O> {
    voxel: Voxel<O>,
    count: O,
    sign_x: O,
    sign_y: O,
    sign_z: O,
    err_x: I,
    err_y: I,
    err_z: I,
    d_err_x: I,
    d_err_y: I,
    d_err_z: I,
}

impl<I: FloatNum, O: SignedNum> WalkVoxels<I, O> {
    #[inline]
    /// Create a new `WalkVoxels` iterator, with the origin of the voxels.
    pub fn new(start: Voxel<I>, end: Voxel<I>, origin: &VoxelOrigin) -> Self {
        let start_i: Voxel<O> = origin.round(start);
        let end_i: Voxel<O> = origin.round(end);

        let count =
            (start_i.0 - end_i.0).abs() + (start_i.1 - end_i.1).abs() + (start_i.2 - end_i.2).abs();

        let sign_x = compare(end_i.0, start_i.0);
        let sign_y = compare(end_i.1, start_i.1);
        let sign_z = compare(end_i.2, start_i.2);

        // Planes for each axis that we will next cross
        let x_plane = start_i.0 + (if end_i.0 > start_i.0 {
            O::one()
        } else {
            O::zero()
        });
        let y_plane = start_i.1 + (if end_i.1 > start_i.1 {
            O::one()
        } else {
            O::zero()
        });
        let z_plane = start_i.2 + (if end_i.2 > start_i.2 {
            O::one()
        } else {
            O::zero()
        });

        // Only used for multiplying up the error margins
        let vx = if start.0 == end.0 {
            I::one()
        } else {
            end.0 - start.0
        };
        let vy = if start.1 == end.1 {
            I::one()
        } else {
            end.1 - start.1
        };
        let vz = if start.2 == end.2 {
            I::one()
        } else {
            end.2 - start.2
        };

        // Error is normalized to vx * vy * vz so we only have to multiply up
        let vxvy = vx * vy;
        let vxvz = vx * vz;
        let vyvz = vy * vz;

        Self {
            sign_x,
            sign_y,
            sign_z,
            count,
            voxel: start_i,
            // Error from the next plane accumulators, scaled up by vx * vy * vz
            // gx0 + vx * rx === gxp
            // vx * rx === gxp - gx0
            // rx === (gxp - gx0) / vx
            err_x: (I::cast(x_plane) - start.0) * vyvz,
            err_y: (I::cast(y_plane) - start.1) * vxvz,
            err_z: (I::cast(z_plane) - start.2) * vxvy,
            d_err_x: I::cast(sign_x) * vyvz,
            d_err_y: I::cast(sign_y) * vxvz,
            d_err_z: I::cast(sign_z) * vxvy,
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Voxel<O>, Self> {
        Steps::new(self)
    }
}

impl<I: FloatNum, O: SignedNum> Iterator for WalkVoxels<I, O> {
    type Item = Voxel<O>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= O::zero() {
            self.count -= O::one();

            // Which plane do we cross first?
            let xr = self.err_x.abs();
            let yr = self.err_y.abs();
            let zr = self.err_z.abs();

            let x_zero = self.sign_x == O::zero();
            let y_zero = self.sign_y == O::zero();
            let z_zero = self.sign_z == O::zero();

            let voxel = self.voxel;

            if !x_zero && (y_zero || xr < yr) && (z_zero || xr < zr) {
                self.voxel.0 += self.sign_x;
                self.err_x += self.d_err_x;
            } else if !y_zero && (z_zero || yr < zr) {
                self.voxel.1 += self.sign_y;
                self.err_y += self.d_err_y;
            } else if !z_zero {
                self.voxel.2 += self.sign_z;
                self.err_z += self.d_err_z;
            }

            Some(voxel)
        } else {
            None
        }
    }
}

#[test]
fn tests() {
    assert_eq!(
        WalkVoxels::new(
            (0.472, -1.100, 0.179),
            (1.114, -0.391, 0.927),
            &VoxelOrigin::Center
        ).collect::<Vec<_>>(),
        [(0, -1, 0), (1, -1, 0), (1, -1, 1), (1, 0, 1)]
    );
}
