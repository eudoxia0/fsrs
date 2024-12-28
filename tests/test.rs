use fsrs::interval;

/// Approximate equality.
fn eq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < 1e-10
}

/// Test that, for R_d = 0.9, I(S) = S.
#[test]
fn test_interval_equals_stability() {
    let samples = 100;
    let start = 0.1;
    let end = 5.0;
    let step = (end - start) / (samples as f64 - 1.0);
    for i in 0..samples {
        let s = start + (i as f64) * step;
        assert!(eq(interval(0.9, s), s));
    }
}
