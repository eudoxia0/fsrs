use std::iter::zip;

use fsrs::{d_0, difficulty, interval, retrievability, s_0, stability, Grade, D, R, S, T, W};

/// Approximate equality.
fn feq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < 0.01
}

/// R_d = 0.9, I(S) = S.
#[test]
fn test_interval_equals_stability() {
    let samples = 100;
    let start = 0.1;
    let end = 5.0;
    let step = (end - start) / (samples as f64 - 1.0);
    for i in 0..samples {
        let s = start + (i as f64) * step;
        assert!(feq(interval(0.9, s), s))
    }
}

/// D_0(1) = w_4
#[test]
fn test_initial_difficulty_of_forgetting() {
    assert_eq!(d_0(Grade::Forgot), W[4])
}

/// A simulation step.
#[derive(Clone, Copy, Debug)]
struct Step {
    /// The time when the review took place.
    t: T,
    /// New stability.
    s: S,
    /// New difficulty.
    d: D,
    /// Next interval.
    i: T,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        feq(self.t, other.t) && feq(self.s, other.s) && feq(self.d, other.d) && feq(self.i, other.i)
    }
}

/// Simulate a series of reviews.
fn sim(grades: Vec<Grade>) -> Vec<Step> {
    let mut t: T = 0.0;
    let r_d: f64 = 0.9;
    let mut steps = vec![];

    // Initial review.
    assert!(!grades.is_empty());
    let mut grades = grades.clone();
    let g: Grade = grades.remove(0);
    let mut s: S = s_0(g);
    let mut d: D = d_0(g);
    let mut i: T = interval(r_d, s).round();
    steps.push(Step { t, s, d, i });

    // n-th review
    for g in grades {
        t += i;
        let r: R = retrievability(i, s);
        s = stability(d, s, r, g);
        d = difficulty(d, g);
        i = interval(r_d, s).round();
        steps.push(Step { t, s, d, i });
    }

    steps
}

/// Test a sequence of three easies.
#[test]
fn test_3e() {
    let e = Grade::Easy;
    let grades = vec![e, e, e];
    let expected = vec![
        Step {
            t: 0.0,
            s: 15.69,
            d: 3.22,
            i: 16.0,
        },
        Step {
            t: 16.0,
            s: 150.28,
            d: 2.13,
            i: 150.0,
        },
        Step {
            t: 166.0,
            s: 1252.22,
            d: 0.86,
            i: 1252.0,
        },
    ];
    let actual = sim(grades);
    assert_eq!(expected.len(), actual.len());
    for (expected, actual) in zip(expected, actual) {
        assert_eq!(actual, expected);
    }
}
