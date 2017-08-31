A collection of line-drawing algorithms for use in graphics and video games.

Currently implemented:

* `bresenham` through [`bresenham-rs`].
* The [mid-point line drawing algorithm].
* [Xiaolin Wu's line algorithm].
* `walk_grid` and `supercover` implemented from [this article by Red Blob Games][article].
* `Bresenham3d` - A 3-Dimensional implementation of bresenham.
* `WalkVoxels` - A similar 3-Dimensional algorithm that only takes orthogonal steps.

[`bresenham-rs`]: https://crates.io/crates/bresenham
[mid-point line drawing algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
[Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
[article]: http://www.redblobgames.com/grids/line-drawing.html