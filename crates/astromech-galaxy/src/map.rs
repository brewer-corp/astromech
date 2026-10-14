//! Jump counts over the known stars: A* where two stars are linked when they are within the
//! ship's jump range. Inside covered spheres the count is exact for that range (no neutron
//! boosts, no fuel limits); elsewhere, or if the search gives up, it is the straight-line
//! estimate `ceil(distance / range)`.

use crate::store::{dist, Sphere, Star};
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Grid cell edge, light years.
const CELL: f32 = 16.0;
/// Node expansions before an A* search falls back to the estimate.
const MAX_EXPANSIONS: usize = 60_000;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Hop {
    pub ly: f64,
    pub jumps: u32,
    /// True when found on the star map inside covered space; false for the estimate.
    pub exact: bool,
}

pub struct StarMap {
    stars: Vec<Star>,
    grid: HashMap<(i32, i32, i32), Vec<u32>>,
    coverage: Vec<Sphere>,
}

fn cell_of(p: [f32; 3]) -> (i32, i32, i32) {
    ((p[0] / CELL).floor() as i32, (p[1] / CELL).floor() as i32, (p[2] / CELL).floor() as i32)
}

fn d32(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn f32p(p: [f64; 3]) -> [f32; 3] {
    [p[0] as f32, p[1] as f32, p[2] as f32]
}

pub fn estimate(ly: f64, range: f64) -> u32 {
    if ly <= 0.0 {
        0
    } else {
        (ly / range.max(0.1)).ceil() as u32
    }
}

impl StarMap {
    pub fn new(stars: Vec<Star>, coverage: Vec<Sphere>) -> Self {
        let mut grid: HashMap<(i32, i32, i32), Vec<u32>> = HashMap::new();
        for (i, s) in stars.iter().enumerate() {
            grid.entry(cell_of(s.pos)).or_default().push(i as u32);
        }
        StarMap { stars, grid, coverage }
    }

    pub fn len(&self) -> usize {
        self.stars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stars.is_empty()
    }

    pub fn covered(&self, p: [f64; 3]) -> bool {
        self.coverage.iter().any(|s| s.contains(p))
    }

    fn neighbours(&self, p: [f32; 3], range: f32, mut f: impl FnMut(u32)) {
        let (cx, cy, cz) = cell_of(p);
        let r = (range / CELL).ceil() as i32;
        for dx in -r..=r {
            for dy in -r..=r {
                for dz in -r..=r {
                    if let Some(v) = self.grid.get(&(cx + dx, cy + dy, cz + dz)) {
                        for &i in v {
                            if d32(self.stars[i as usize].pos, p) <= range {
                                f(i);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Jumps from `a` to `b` at `range` ly per jump.
    pub fn hop(&self, a: [f64; 3], b: [f64; 3], range: f64) -> Hop {
        let ly = dist(a, b);
        let est = Hop { ly, jumps: estimate(ly, range), exact: false };
        if range <= 0.0 || est.jumps <= 1 {
            return Hop { exact: self.covered(a) && self.covered(b), ..est };
        }
        if !(self.covered(a) && self.covered(b)) {
            return est;
        }
        match self.astar(f32p(a), f32p(b), range as f32) {
            Some(j) => Hop { ly, jumps: j.max(est.jumps), exact: true },
            None => est,
        }
    }

    /// A* from `a` to `b`. Node `u32::MAX` is the start point (it may not be a known star).
    fn astar(&self, a: [f32; 3], b: [f32; 3], range: f32) -> Option<u32> {
        let h = |p: [f32; 3]| (d32(p, b) / range).ceil() as u32;
        let mut best: HashMap<u32, u32> = HashMap::new();
        // (f, h-distance tie-break, g, node)
        let mut open: BinaryHeap<Reverse<(u32, u32, u32, u32)>> = BinaryHeap::new();
        let start = u32::MAX;
        open.push(Reverse((h(a), (d32(a, b) * 32.0) as u32, 0, start)));
        best.insert(start, 0);
        let mut expanded = 0;
        while let Some(Reverse((_, _, g, n))) = open.pop() {
            let p = if n == start { a } else { self.stars[n as usize].pos };
            if d32(p, b) <= range {
                return Some(g + 1);
            }
            if best.get(&n).is_some_and(|&bg| bg < g) {
                continue;
            }
            expanded += 1;
            if expanded > MAX_EXPANSIONS {
                return None;
            }
            self.neighbours(p, range, |i| {
                let ng = g + 1;
                if best.get(&i).is_none_or(|&bg| ng < bg) {
                    best.insert(i, ng);
                    let q = self.stars[i as usize].pos;
                    open.push(Reverse((ng + h(q), (d32(q, b) * 32.0) as u32, ng, i)));
                }
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(n: i64, step: f32) -> Vec<Star> {
        (0..n).map(|i| Star { id64: i, pos: [i as f32 * step, 0.0, 0.0] }).collect()
    }

    #[test]
    fn exact_on_map_estimate_off_map() {
        // stars every 9 ly: a 20 ly ship covers 18 ly per jump, not 20
        let m = StarMap::new(line(30, 9.0), vec![Sphere { center: [0.0; 3], radius: 300.0 }]);
        let h = m.hop([0.0; 3], [180.0, 0.0, 0.0], 20.0);
        assert!(h.exact);
        assert_eq!(h.jumps, 10);
        assert_eq!(estimate(180.0, 20.0), 9);
        // a gap wider than the range: no route on the map → estimate
        let mut s = line(5, 9.0);
        s.push(Star { id64: 99, pos: [100.0, 0.0, 0.0] });
        let m = StarMap::new(s, vec![Sphere { center: [0.0; 3], radius: 300.0 }]);
        let h = m.hop([0.0; 3], [100.0, 0.0, 0.0], 20.0);
        assert!(!h.exact);
        assert_eq!(h.jumps, 5);
        // outside coverage → estimate
        let h = m.hop([0.0; 3], [1000.0, 0.0, 0.0], 50.0);
        assert_eq!((h.jumps, h.exact), (20, false));
    }
}
