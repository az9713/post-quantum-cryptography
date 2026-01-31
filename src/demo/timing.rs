//! Performance Timing Utilities
//!
//! Provides simple timing measurements for educational demonstrations.

use std::time::{Duration, Instant};

/// A simple timing utility for measuring operation duration
pub struct Timer {
    start: Instant,
    label: String,
}

impl Timer {
    /// Start a new timer with a label
    pub fn start(label: &str) -> Self {
        Self {
            start: Instant::now(),
            label: label.to_string(),
        }
    }

    /// Stop the timer and return elapsed duration
    pub fn stop(&self) -> Duration {
        self.start.elapsed()
    }

    /// Stop and format as string
    pub fn stop_formatted(&self) -> String {
        let elapsed = self.stop();
        format_duration(elapsed)
    }

    /// Stop and print result
    pub fn stop_and_print(&self) {
        let elapsed = self.stop();
        println!("  {} took {}", self.label, format_duration(elapsed));
    }
}

/// Format a duration in human-readable form
pub fn format_duration(d: Duration) -> String {
    let micros = d.as_micros();
    if micros < 1000 {
        format!("{}us", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", micros as f64 / 1_000_000.0)
    }
}

/// Time a closure and return (result, duration)
pub fn time_operation<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}

/// Performance comparison data
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub key_generation_us: u128,
    pub signing_us: u128,
    pub verification_us: u128,
    pub signature_size: usize,
    pub public_key_size: usize,
}

impl PerformanceMetrics {
    /// Compare to typical ECDSA performance
    pub fn compare_to_ecdsa(&self) -> String {
        // Approximate ECDSA metrics (from typical implementations)
        let ecdsa_keygen_us = 100; // ~100us
        let ecdsa_sign_us = 50; // ~50us
        let ecdsa_verify_us = 100; // ~100us
        let ecdsa_sig_size = 64;
        let ecdsa_pk_size = 33;

        format!(
            "Performance Comparison (WOTS+ vs ECDSA):\n\
             \n\
             Key Generation:\n\
             - WOTS+: {}us\n\
             - ECDSA: ~{}us (est.)\n\
             - Ratio: {:.1}x slower\n\
             \n\
             Signing:\n\
             - WOTS+: {}us\n\
             - ECDSA: ~{}us (est.)\n\
             - Ratio: {:.1}x slower\n\
             \n\
             Verification:\n\
             - WOTS+: {}us\n\
             - ECDSA: ~{}us (est.)\n\
             - Ratio: {:.1}x slower\n\
             \n\
             Signature Size:\n\
             - WOTS+: {} bytes\n\
             - ECDSA: {} bytes\n\
             - Ratio: {:.1}x larger\n\
             \n\
             Public Key Size:\n\
             - WOTS+: {} bytes\n\
             - ECDSA: {} bytes\n\
             - Ratio: {:.1}x larger",
            self.key_generation_us,
            ecdsa_keygen_us,
            self.key_generation_us as f64 / ecdsa_keygen_us as f64,
            self.signing_us,
            ecdsa_sign_us,
            self.signing_us as f64 / ecdsa_sign_us as f64,
            self.verification_us,
            ecdsa_verify_us,
            self.verification_us as f64 / ecdsa_verify_us as f64,
            self.signature_size,
            ecdsa_sig_size,
            self.signature_size as f64 / ecdsa_sig_size as f64,
            self.public_key_size,
            ecdsa_pk_size,
            self.public_key_size as f64 / ecdsa_pk_size as f64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let timer = Timer::start("test");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.stop();
        assert!(elapsed.as_millis() >= 10);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_micros(500)), "500us");
        assert_eq!(format_duration(Duration::from_micros(1500)), "1.50ms");
        assert_eq!(format_duration(Duration::from_micros(1_500_000)), "1.50s");
    }
}
