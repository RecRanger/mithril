use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam_channel::Receiver;

struct Sample {
    at: Instant,
    count: u64,
}

pub struct MetricsAggregator {
    samples: VecDeque<Sample>,
    first_sample_at: Option<Instant>,
    /// Lifetime hash counter — clone the Arc before spawning the metrics
    /// thread so `main` can read it for the bandit reward calculation.
    pub total_hashes: Arc<AtomicU64>,

    window_15s: Duration,
    window_60s: Duration,
    window_15m: Duration,
}

impl MetricsAggregator {
    /// Returns `(aggregator, total_hashes_handle)`.
    /// Keep `total_hashes_handle` in `main`; move the aggregator into the thread.
    pub fn new() -> (Self, Arc<AtomicU64>) {
        let total_hashes = Arc::new(AtomicU64::new(0));
        let agg = MetricsAggregator {
            samples: VecDeque::new(),
            first_sample_at: None,
            total_hashes: total_hashes.clone(),
            window_15s: Duration::from_secs(15),
            window_60s: Duration::from_secs(60),
            window_15m: Duration::from_secs(15 * 60),
        };
        (agg, total_hashes)
    }

    /// Drain all pending hash counts from the channel (never blocks),
    /// then return (15 s, 60 s, 15 min) rates in H/s.
    /// `None` means the window hasn't filled yet — callers stay silent.
    pub fn update(&mut self, rx: &Receiver<u64>) -> (Option<f64>, Option<f64>, Option<f64>) {
        let now = Instant::now();

        while let Ok(count) = rx.try_recv() {
            if self.first_sample_at.is_none() {
                self.first_sample_at = Some(now);
            }
            self.total_hashes.fetch_add(count, Ordering::Relaxed);
            self.samples.push_back(Sample { at: now, count });
        }

        // Drop samples older than the largest window.
        let horizon = now - self.window_15m;
        while self.samples.front().map_or(false, |s| s.at < horizon) {
            self.samples.pop_front();
        }

        let elapsed = match self.first_sample_at {
            Some(t) => now.duration_since(t),
            None => return (None, None, None),
        };

        let rate_for = |window: Duration| -> Option<f64> {
            if elapsed < window {
                return None;
            }
            let cutoff = now - window;
            let hashes: u64 = self
                .samples
                .iter()
                .filter(|s| s.at >= cutoff)
                .map(|s| s.count)
                .sum();
            Some(hashes as f64 / window.as_secs_f64())
        };

        (
            rate_for(self.window_15s),
            rate_for(self.window_60s),
            rate_for(self.window_15m),
        )
    }
}

/// Format a hashrate value.  Returns aligned whitespace for `None` so the
/// log columns stay stable while windows are still filling.
pub fn fmt_rate(rate: Option<f64>) -> String {
    match rate {
        None => "           ".to_string(), // blank — window not yet full
        Some(h) if h >= 1_000_000.0 => format!("{:>7.2} MH/s", h / 1_000_000.0),
        Some(h) if h >= 1_000.0 => format!("{:>7.2} KH/s", h / 1_000.0),
        Some(h) => format!("{:>7.2}  H/s", h),
    }
}

/// Run this on a dedicated thread.  `stop` is flipped by `main` when the
/// pool is torn down — the thread exits cleanly instead of leaking.
pub fn run_metrics_loop(mut agg: MetricsAggregator, rx: &Receiver<u64>, stop: &AtomicBool) {
    let poll_interval = Duration::from_secs(5);

    loop {
        std::thread::sleep(poll_interval);

        if stop.load(Ordering::Relaxed) {
            // One final drain so total_hashes is accurate for the bandit.
            agg.update(rx);
            return;
        }

        let (r15s, r60s, r15m) = agg.update(rx);

        if r15s.is_some() || r60s.is_some() || r15m.is_some() {
            info!(
                "hashrate  15s: {}  60s: {}  15m: {}",
                fmt_rate(r15s),
                fmt_rate(r60s),
                fmt_rate(r15m),
            );
        }
    }
}
