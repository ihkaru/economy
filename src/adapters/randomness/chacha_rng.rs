use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::core::ports::rng_port::RngPort;

pub struct ChaChaRngAdapter {
    rng: ChaCha8Rng,
}

impl ChaChaRngAdapter {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl RngPort for ChaChaRngAdapter {
    fn next_u64(&mut self) -> u64 {
        self.rng.r#gen()
    }

    fn next_f64(&mut self) -> f64 {
        self.rng.r#gen::<f64>()
    }

    fn gen_range_u64(&mut self, low: u64, high: u64) -> u64 {
        if low >= high {
            return low;
        }
        self.rng.gen_range(low..high)
    }

    fn gen_range_f64(&mut self, low: f64, high: f64) -> f64 {
        if low >= high {
            return low;
        }
        self.rng.gen_range(low..high)
    }

    fn check_probability(&mut self, p: f64) -> bool {
        if p <= 0.0 {
            false
        } else if p >= 1.0 {
            true
        } else {
            self.next_f64() < p
        }
    }
}
