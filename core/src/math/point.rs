#[derive(Debug, Default, Clone, Copy)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

impl<N> Point<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}
