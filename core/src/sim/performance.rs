use crate::math::ema::EmaTimer;

#[derive(Default, Clone)]
pub struct SimPerformance {
    pub tick: EmaTimer,
    pub update: EmaTimer,
}
