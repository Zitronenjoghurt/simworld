#[derive(Debug, Default, Copy, Clone)]
pub struct Size<N> {
    pub width: N,
    pub height: N,
}

impl<N> Size<N> {
    pub fn new(width: N, height: N) -> Self {
        Self { width, height }
    }
}
