use std::fmt::{Debug, Display};
use std::time;
use std::time::Duration;

/// Exponential Moving Average
#[derive(Debug, Default, Clone, Copy)]
pub struct EMA<N: EMAValue>(N);

impl<N> EMA<N>
where
    N: EMAValue,
{
    pub fn new(value: N) -> Self {
        Self(value)
    }

    pub fn update(&mut self, new: N, periods: usize) {
        let current = self.0.to_f64();
        if current == 0.0 {
            self.0 = new;
        } else {
            let alpha = 2.0 / (periods as f64 + 1.0);
            self.0 = N::from_f64((new.to_f64() * alpha) + (current * (1.0 - alpha)));
        }
    }

    pub fn value(&self) -> N {
        self.0
    }
}

pub trait EMAValue: Copy {
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;
}

impl EMAValue for f64 {
    fn to_f64(self) -> f64 {
        self
    }

    fn from_f64(value: f64) -> Self {
        value
    }
}

impl EMAValue for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }
}

impl EMAValue for Duration {
    fn to_f64(self) -> f64 {
        self.as_secs_f64()
    }

    fn from_f64(value: f64) -> Self {
        Duration::from_secs_f64(value)
    }
}

impl EMAValue for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        value.floor() as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EmaTimer {
    start: time::Instant,
    last_stop: Option<time::Instant>,
    ema_secs: EMA<f32>,
    ema_interval: EMA<f32>,
}

impl Default for EmaTimer {
    fn default() -> Self {
        Self {
            start: time::Instant::now(),
            last_stop: None,
            ema_secs: Default::default(),
            ema_interval: Default::default(),
        }
    }
}

impl EmaTimer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.start = time::Instant::now();
    }

    pub fn stop(&mut self) {
        let now = time::Instant::now();
        let elapsed = (now - self.start).as_secs_f32();
        self.ema_secs.update(elapsed, 1200);

        if let Some(last) = self.last_stop {
            let interval = (now - last).as_secs_f32();
            if interval > 0.0 {
                self.ema_interval.update(interval, 1200);
            }
        }
        self.last_stop = Some(now);
    }

    pub fn update(&mut self, duration: Duration, periods: usize) {
        self.ema_secs.update(duration.as_secs_f32(), periods);
    }

    pub fn average_secs(&self) -> f32 {
        self.ema_secs.value()
    }

    pub fn display_average_secs(&self) -> String {
        format!("{:.02}ms", self.average_secs() * 1000.0)
    }

    pub fn updates_per_sec(&self) -> f32 {
        let interval = self.ema_interval.value();
        if interval > 0.0 { 1.0 / interval } else { 0.0 }
    }

    pub fn display_updates_per_sec(&self) -> String {
        format!("{:.01}/s", self.updates_per_sec())
    }

    pub fn budget(&self) -> f32 {
        self.average_secs() * self.updates_per_sec()
    }

    pub fn display_budget(&self) -> String {
        format!("{:.01}%", self.budget() * 100.0)
    }
}

impl Display for EmaTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} ({})",
            self.display_average_secs(),
            self.display_updates_per_sec(),
            self.display_budget()
        )
    }
}
