use std::time::Duration;

use criterion::Criterion;

pub const LONG_BENCH_DURATION: Duration = Duration::from_secs(60);
pub const LONG_BENCH_SAMPLE_SIZE: usize = 50;

pub fn slow_config() -> Criterion {
    Criterion::default()
        .measurement_time(LONG_BENCH_DURATION)
        .sample_size(LONG_BENCH_SAMPLE_SIZE)
}
