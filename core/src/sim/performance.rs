use crate::math::ema::EmaTimer;

#[derive(Default, Clone)]
pub struct SimPerformance {
    pub update: EmaTimer,
}
