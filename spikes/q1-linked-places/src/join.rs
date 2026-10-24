//! The local join: both sides are point sets; pair each A with its nearest B within a radius.
//! A grid keyed by cell keeps it linear for the set sizes seen (hundreds to thousands).

use std::collections::HashMap;

pub type Point = [f64; 3];

pub fn dist(a: Point, b: Point) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn cell(p: Point, size: f64) -> (i64, i64, i64) {
    ((p[0] / size).floor() as i64, (p[1] / size).floor() as i64, (p[2] / size).floor() as i64)
}

/// For each point of `a`, the nearest point of `b` within `radius`: (a index, b index, ly).
pub fn nearest_within(a: &[Point], b: &[Point], radius: f64) -> Vec<(usize, usize, f64)> {
    let size = radius.max(1.0);
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for (i, p) in b.iter().enumerate() {
        grid.entry(cell(*p, size)).or_default().push(i);
    }
    let mut out = vec![];
    for (ai, p) in a.iter().enumerate() {
        let (cx, cy, cz) = cell(*p, size);
        let mut best: Option<(usize, f64)> = None;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for &bi in grid.get(&(cx + dx, cy + dy, cz + dz)).into_iter().flatten() {
                        let d = dist(*p, b[bi]);
                        if d <= radius && best.is_none_or(|(_, bd)| d < bd) {
                            best = Some((bi, d));
                        }
                    }
                }
            }
        }
        if let Some((bi, d)) = best {
            out.push((ai, bi, d));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_each_a_with_its_nearest_b_inside_the_radius() {
        let a = [[0.0, 0.0, 0.0], [100.0, 0.0, 0.0], [0.0, 55.0, 0.0]];
        let b = [[0.0, 19.0, 0.0], [0.0, 5.0, 0.0], [130.0, 0.0, 0.0], [0.0, 74.0, 0.0]];
        let got = nearest_within(&a, &b, 20.0);
        assert_eq!(got.len(), 2);
        assert_eq!((got[0].0, got[0].1), (0, 1));
        assert!((got[0].2 - 5.0).abs() < 1e-9);
        assert_eq!((got[1].0, got[1].1), (2, 3));
    }

    #[test]
    fn brute_force_agrees() {
        let mut s: u64 = 7;
        let mut r = || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((s >> 33) as f64 / (1u64 << 31) as f64) * 200.0 - 100.0
        };
        let a: Vec<Point> = (0..200).map(|_| [r(), r(), r()]).collect();
        let b: Vec<Point> = (0..150).map(|_| [r(), r(), r()]).collect();
        let got = nearest_within(&a, &b, 15.0);
        let mut want = vec![];
        for (ai, p) in a.iter().enumerate() {
            let best = b
                .iter()
                .enumerate()
                .map(|(i, q)| (i, dist(*p, *q)))
                .filter(|x| x.1 <= 15.0)
                .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
            if let Some((bi, d)) = best {
                want.push((ai, bi, d));
            }
        }
        assert_eq!(got, want);
    }
}
