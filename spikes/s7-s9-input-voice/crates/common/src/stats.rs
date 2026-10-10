//! Small summary statistics helpers.

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Summary {
    pub n: usize,
    pub min: f64,
    pub median: f64,
    pub mean: f64,
    /// Nearest-rank 95th percentile.
    pub p95: f64,
    pub max: f64,
}

pub fn summarize(values: &[f64]) -> Option<Summary> {
    let mut v: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    let median = if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 };
    let rank = ((0.95 * n as f64).ceil() as usize).clamp(1, n);
    Some(Summary {
        n,
        min: v[0],
        median,
        mean: v.iter().sum::<f64>() / n as f64,
        p95: v[rank - 1],
        max: v[n - 1],
    })
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n={} min={:.1} median={:.1} mean={:.1} p95={:.1} max={:.1}",
            self.n, self.min, self.median, self.mean, self.p95, self.max
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s = summarize(&[5.0, 1.0, 3.0, 2.0, 4.0]).unwrap();
        assert_eq!((s.n, s.min, s.median, s.mean, s.p95, s.max), (5, 1.0, 3.0, 3.0, 5.0, 5.0));
        let s = summarize(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(s.median, 2.5);
        assert!(summarize(&[]).is_none());
        assert!(summarize(&[f64::NAN]).is_none());
        let v: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        assert_eq!(summarize(&v).unwrap().p95, 19.0);
    }
}
