use Voxel;

#[inline]
fn compare(a: isize, b: isize) -> isize {
    if a > b { 1 } else if a == b { 0 } else { -1 }
}

#[inline]
fn round(voxel: Voxel<f32>) -> Voxel<isize> {
    (voxel.0.round() as isize, voxel.1.round() as isize, voxel.2.round() as isize)
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
/// use line_drawing::WalkVoxels; 
///
/// fn main() {
///     for (i, (x, y, z)) in WalkVoxels::new((0.0, 0.0, 0.0), (5.0, 6.0, 7.0)).enumerate() {
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
pub struct WalkVoxels {
    voxel: Voxel<isize>,
    count: isize,
    sign_x: isize,
    sign_y: isize,
    sign_z: isize,
    err_x: f32,
    err_y: f32,
    err_z: f32,
    d_err_x: f32,
    d_err_y: f32,
    d_err_z: f32
}

impl WalkVoxels {
    #[inline]
    pub fn new(start: Voxel<f32>, end: Voxel<f32>) -> WalkVoxels {
        let start_i = round(start);
        let end_i = round(end);

        let count = (start_i.0 - end_i.0).abs() +
                    (start_i.1 - end_i.1).abs() +
                    (start_i.2 - end_i.2).abs();

        let sign_x = compare(end_i.0, start_i.0);
        let sign_y = compare(end_i.1, start_i.1);
        let sign_z = compare(end_i.2, start_i.2);

        // Planes for each axis that we will next cross
        let x_plane = start_i.0 + (if end_i.0 > start_i.0 {1} else {0});
        let y_plane = start_i.1 + (if end_i.1 > start_i.1 {1} else {0});
        let z_plane = start_i.2 + (if end_i.2 > start_i.2 {1} else {0});

        // Only used for multiplying up the error margins
        let vx = if start.0 == end.0 {1.0} else {end.0 - start.0};
        let vy = if start.1 == end.1 {1.0} else {end.1 - start.1};
        let vz = if start.2 == end.2 {1.0} else {end.2 - start.2};

        // Error is normalized to vx * vy * vz so we only have to multiply up
        let vxvy = vx * vy;
        let vxvz = vx * vz;
        let vyvz = vy * vz;

        WalkVoxels {
            sign_x, sign_y, sign_z, count,
            voxel: start_i,
            // Error from the next plane accumulators, scaled up by vx * vy * vz
            // gx0 + vx * rx === gxp
            // vx * rx === gxp - gx0
            // rx === (gxp - gx0) / vx
            err_x: (x_plane as f32 - start.0) * vyvz,
            err_y: (y_plane as f32 - start.1) * vxvz,
            err_z: (z_plane as f32 - start.2) * vxvy,
            d_err_x: sign_x as f32 * vyvz,
            d_err_y: sign_y as f32 * vxvz,
            d_err_z: sign_z as f32 * vxvy
        }
    }
}

impl Iterator for WalkVoxels {
    type Item = Voxel<isize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 0 {
            self.count -= 1;
            
            // Which plane do we cross first?
            let xr = self.err_x.abs();
            let yr = self.err_y.abs();
            let zr = self.err_z.abs();

            let voxel = self.voxel;

            if self.sign_x != 0 && (self.sign_y == 0 || xr < yr) && (self.sign_z == 0 || xr < zr) {
                self.voxel.0 += self.sign_x;
                self.err_x += self.d_err_x;
            }
            else if self.sign_y != 0 && (self.sign_z == 0 || yr < zr) {
                self.voxel.1 += self.sign_y;
                self.err_y += self.d_err_y;
            }
            else if self.sign_z != 0 {
                self.voxel.2 += self.sign_z;
                self.err_z += self.d_err_z;
            }

            Some(voxel)
        } else {
            None
        }
    }
}

/// A convenience function to collect the points from [`WalkVoxels`] into a [`Vec`].
/// [`WalkVoxels`]: struct.WalkVoxels.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn walk_voxels(start: Voxel<f32>, end: Voxel<f32>) -> Vec<Voxel<isize>> {
    WalkVoxels::new(start, end).collect()
}

#[test]
fn tests() {
    assert_eq!(
        walk_voxels(
            (0.472, -1.100, 0.179),
            (1.114, -0.391, 0.927)
        ),
        [(0, -1, 0), (1, -1, 0), (1, -1, 1), (1, 0, 1)]
    );
}