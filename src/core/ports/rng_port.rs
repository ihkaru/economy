pub trait RngPort: Send + Sync {
    fn next_u64(&mut self) -> u64;
    fn next_f64(&mut self) -> f64;
    fn gen_range_u64(&mut self, low: u64, high: u64) -> u64;
    fn gen_range_f64(&mut self, low: f64, high: f64) -> f64;
    fn check_probability(&mut self, p: f64) -> bool;
}
