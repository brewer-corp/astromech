#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub n: usize,
    pub misses: usize,
    pub min: Option<f64>,
    pub median: Option<f64>,
    pub p95: Option<f64>,
    pub max: Option<f64>,
}

/// Nearest-rank percentile on sorted data.
fn pct(sorted: &[f64], p: f64) -> f64 {
    let rank = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

pub fn summarize(samples: &[Option<f64>]) -> Summary {
    let mut v: Vec<f64> = samples.iter().flatten().copied().collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let misses = samples.len() - v.len();
    if v.is_empty() {
        return Summary { n: 0, misses, min: None, median: None, p95: None, max: None };
    }
    let median = if v.len() % 2 == 1 { v[v.len() / 2] } else { (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0 };
    Summary { n: v.len(), misses, min: Some(v[0]), median: Some(median), p95: Some(pct(&v, 95.0)), max: Some(*v.last().unwrap()) }
}

pub fn fmt_ms(x: Option<f64>) -> String {
    x.map(|v| format!("{v:.1}")).unwrap_or_else(|| "-".into())
}
