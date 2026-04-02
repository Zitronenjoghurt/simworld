#[derive(Debug, Copy, Clone)]
pub struct SimConfig {
    pub update_rate: u16,
    pub max_update_debt: u32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            update_rate: 60,
            max_update_debt: 5,
        }
    }
}
