//! An iterator that returns `(start, end)` tuples from the walk.

/// An iterator that returns `(start, end)` tuples from the walk.
///
/// All the algorithms in this crate should have a `steps()` function associated with them to turn
/// them into a [`Steps`] iterator.
///
/// Example using [`WalkGrid`]:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::WalkGrid;
///
/// fn main() {
///     for (start, end) in WalkGrid::new((0, 0), (5, 3)).steps() {
///         println!("{:?} -> {:?}", start, end);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0) -> (1, 0)
/// (1, 0) -> (1, 1)
/// (1, 1) -> (2, 1)
/// (2, 1) -> (2, 2)
/// (2, 2) -> (3, 2)
/// (3, 2) -> (4, 2)
/// (4, 2) -> (4, 3)
/// (4, 3) -> (5, 3)
/// ```
///
/// [`Steps`]: struct.Steps.html
/// [`WalkGrid`]: ../struct.WalkGrid.html
pub struct Steps<T: Copy, I: Iterator<Item = T>> {
    iterator: I,
    prev: Option<T>
}

impl<T: Copy, I: Iterator<Item = T>> Steps<T, I> {
    #[inline]
    pub fn new(mut iterator: I) -> Steps<T, I> {
        Steps {
            prev: iterator.next(),
            iterator
        }
    }
}

impl<T: Copy, I: Iterator<Item = T>> Iterator for Steps<T, I> {
    type Item = (T, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|next| self.prev.map(|prev| {
            self.prev = Some(next);
            (prev, next)
        }))
    }
}

#[test]
fn steps() {
    use Midpoint;

    assert_eq!(
        Midpoint::new((0.0, 0.0), (3.0, 4.0)).steps().collect::<Vec<_>>(),
        [
            ((0, 0), (1, 1)),
            ((1, 1), (2, 2)),
            ((2, 2), (2, 3)),
            ((2, 3), (3, 4))
        ]
    );
}