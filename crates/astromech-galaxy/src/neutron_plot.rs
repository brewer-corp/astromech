//! The local neutron plotter: a neutron route worked out on this PC from the neutron star file
//! ([`crate::neutron`]), with no network. [`route`] is the search (positions in, route out);
//! [`plot`] runs it for a ship and reads the stops' names, giving the same [`Plotted`] as the
//! Spansh plotter.
//!
//! The model (astromech-1842): a route's stops are neutron stars and the destination, and the
//! galaxy map plots each leg. A jump from a neutron star is boosted and reaches `k·R` (k the
//! drive's supercharge multiplier, R its full-tank range); the rest of the leg is normal jumps
//! of R. So a leg of d ly costs `1 + ceil(max(0, d − k·R) / R)` jumps from a neutron star and
//! `ceil(d / R)` from an ordinary start ([`leg_jumps`]). A start on a neutron star boosts its
//! first jump. The route with no neutron stars is a candidate. The search minimises jumps, then
//! stops. Fuel and whether real stars fill the gaps between neutron stars are not modelled.
//!
//! The search is A* over the neutron stars, the start and the destination, with the cost a
//! pair (jumps, stops) compared in that order. Edges are not stored: a star's neighbours are
//! found in the file's grid when the search reaches it, one shell at a time (the stars one jump
//! away, then those two jumps away, and so on), and a farther shell is only scanned when the
//! search gets to it, so dense space costs the stars within k·R and only sparse space scans
//! wider. Shells stop where going straight to the destination is no worse. A star's neighbours
//! that the star before it already reached as cheaply aren't looked at again.
//!
//! First, plain A* with the straight-line bound (distance left over k·R, which never
//! overestimates) for up to `EXACT_WORK` (stars and cells looked at): if it finishes, the route
//! has the fewest jumps, then stops, in the model ([`Route::fewest`]). That covers short routes,
//! but not long ones: real routes make 70–95% of k·R a jump, so plain A* must try every star the
//! bound says might still do better, and Sol to Beagle Point didn't finish in two minutes.
//! Then a guided A*: the estimate of the jumps left comes from a coarse grid around the route
//! (`Field`), each block costing per light year what a boosted jump makes there given how many
//! neutron stars are near it (or normal jumps, where boosts can't do better), with Dijkstra over
//! the grid giving each block's jumps to the destination. It can be a little high or low, so this
//! route is near the fewest rather than proven. Its work is bounded too: after each `WORK` the
//! estimate counts for a little more (`GREEDIER`), which narrows the search, so a hard route ends
//! in bounded time, a little longer. A last pass drops any stop that saves no jumps.
//!
//! Measured on this PC (12 cores shared with other builds, load average about 9; the 22 Oct 2026
//! file of 5,045,154 neutron stars; release build; `NEUTRON_BIN=<file> cargo test --release -p
//! astromech-galaxy --test neutron_plot -- --ignored --nocapture`, one route a run with
//! `NEUTRON_CASE=0..3`):
//!
//! | Route | R, k | Jumps | Stops | Search | Peak memory |
//! | --- | --- | --- | --- | --- | --- |
//! | Sol to Colonia | 60 ly, ×4 | 114 | 101 | 0.63 s | 93 MB |
//! | Sol to Colonia | 80 ly, ×6 | 53 | 51 | 1.00 s | 100 MB |
//! | Sol to Beagle Point | 60 ly, ×4 | 383 | 279 | 0.60 s | 91 MB |
//! | Sol to Beagle Point | 80 ly, ×6 | 161 | 146 | 1.34 s | 103 MB |
//!
//! Peak memory is the process's during the search, 79 MB of it the loaded file, so a search
//! takes 12–24 MB more. Opening the file takes 0.1–0.2 s and peaks at 153 MB (the positions are
//! read whole, then converted). Names are read only for the stops, in under 1 ms. Plain A* with
//! no work limit gets 110 jumps for Sol to Colonia at 60 ly ×4 (in 2.9 s) and 51 at 80 ly ×6
//! (2.6 s), so there the guided routes are 2–4 jumps longer than the fewest. Against Spansh's
//! routes recorded the same day (efficiency 60), in this model: Sol to Colonia at 50 ly ×4, 139
//! jumps and 119 stops to Spansh's 160 (166 by its own count) and 128; Sol to Beagle Point at
//! 60 ly ×4, 383 and 279 to 438 (467) and 298, and at 80 ly ×6, 161 and 146 to 181 (190) and 150.
//!
//! Spansh's own neutron plotter counts the normal jumps after a boost more cautiously than this
//! model (tests/neutron_plot.rs explains the difference on recorded routes), so its jump counts
//! for the same route are a little higher.

use crate::neutron::{NeutronStars, Stars};
use crate::plot::{Plotted, Ship, Stop, NEUTRON};
use crate::store::dist;
use crate::Here;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// The route's `source` on the Route page.
pub const SOURCE: &str = "neutron plotter (this PC)";

/// Jumps for a leg of `d` ly whose first jump reaches `first` ly (k·R from a neutron star, R
/// otherwise) and every other jump `range` ly.
pub fn leg_jumps(d: f64, first: f64, range: f64) -> u32 {
    if d <= 0.0 {
        0
    } else if d <= first {
        1
    } else {
        1 + ((d - first) / range).ceil() as u32
    }
}

/// A leg of a route: to neutron star `star`, or to the destination (`None`).
#[derive(Debug, Clone, PartialEq)]
pub struct Leg {
    pub star: Option<usize>,
    pub jumps: u32,
    pub ly: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    /// The neutron star the route starts on, if it does (its first jump is boosted).
    pub start: Option<usize>,
    /// The stops in order; the last is the destination.
    pub legs: Vec<Leg>,
    /// Plain A* finished: no route has fewer jumps (then stops) in the model. False when the
    /// guided search found it, which is near the fewest but not proven.
    pub fewest: bool,
}

impl Route {
    pub fn jumps(&self) -> u32 {
        self.legs.iter().map(|l| l.jumps).sum()
    }
}

/// A node's cost so far, plus one (0 is "not reached"): jumps in the high half, stops in the
/// low, so comparing the numbers compares jumps, then stops.
type Cost = u64;

/// `c` more jumps and one more stop.
fn plus(cost: Cost, jumps: u32) -> Cost {
    cost + (u64::from(jumps) << 32) + 1
}

/// A cost and an estimate of the jumps still to go, as one number to order the search by: the
/// stops count for 1/65536 of a jump, so they only break ties.
fn key(c: Cost, more_jumps: u32, more_stops: u32, h: f64) -> u64 {
    let c = c - 1;
    let jumps = (c >> 32) as f64 + f64::from(more_jumps);
    let stops = (c & 0xffff_ffff) as f64 + f64::from(more_stops);
    ((jumps + stops / 65536.0 + h) * 1048576.0) as u64
}

/// A heap entry: node `node` reached at `cost` (shell 0), or the stars in shell `shell` of the
/// node, which cost `shell` jumps more. Ordered by the estimated cost of the whole route,
/// then by the distance left, so ties go to the stars nearest the destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    estimate: u64,
    /// Distance left (or its lower bound for a shell), 1/32 ly.
    left: u32,
    cost: Cost,
    node: u32,
    shell: u32,
    /// Estimated jumps from here (unweighted), as `f32` bits.
    h: u32,
}

impl Entry {
    fn new(cost: Cost, shell: u32, h: f64, left: f64, node: u32, weight: f64) -> Entry {
        let more = (shell, u32::from(shell > 0));
        let estimate = key(cost, more.0, more.1, weight * h);
        Entry { estimate, left: (left * 32.0) as u32, cost, node, shell, h: (h as f32).to_bits() }
    }

    fn reweigh(self, weight: f64) -> Entry {
        let more = (self.shell, u32::from(self.shell > 0));
        Entry { estimate: key(self.cost, more.0, more.1, weight * f64::from(f32::from_bits(self.h))), ..self }
    }
}

/// Blocks in the estimate's grid at most; a longer route gets bigger blocks.
const MAX_BLOCKS: usize = 300_000;
/// How far around the start and destination the estimate's grid reaches, ly.
const MARGIN: f64 = 2000.0;
/// How far short of k·R a boosted jump lands, in units of the depth of a cap of the jump's
/// sphere that holds one star on average (below 1: the search can choose among several).
/// Tuned on the measured routes in the module docs.
const SHORT: f64 = 0.6;

/// The estimate of the jumps left: a coarse grid over the space around the route, each block
/// with a cost per light year from the neutron stars near it, and each block's jumps to the
/// destination worked out once per route by Dijkstra over the grid. Dense space costs a little
/// over 1/(k·R) a light year and empty space 1/R, so a gap of d ly between two neutron stars
/// costs about `1 + (d − k·R) / R`, as the model does, and the search heads for the stars that
/// get across sparse space.
struct Field {
    /// Block key of the grid's low corner.
    lo: [i32; 3],
    dims: [i32; 3],
    /// Block edge, ly.
    edge: f64,
    /// Estimated jumps from each block to the destination.
    left: Vec<f32>,
    range: f64,
}

impl Field {
    fn new(stars: &Stars, from: [f64; 3], to: [f64; 3], range: f64, boost: f64) -> Field {
        let cell = f64::from(stars.cell_ly());
        let low: [f64; 3] = std::array::from_fn(|a| from[a].min(to[a]) - MARGIN);
        let high: [f64; 3] = std::array::from_fn(|a| from[a].max(to[a]) + MARGIN);
        let mut edge = cell;
        let (lo, dims) = loop {
            let lo = low.map(|v| (v / edge).floor() as i32);
            let dims: [i32; 3] = std::array::from_fn(|a| (high[a] / edge).floor() as i32 - lo[a] + 1);
            if dims.iter().map(|&d| d as usize).product::<usize>() <= MAX_BLOCKS {
                break (lo, dims);
            }
            edge *= 2.0;
        };
        let mut field = Field { lo, dims, edge, left: Vec::new(), range };
        let n = dims.iter().map(|&d| d as usize).product();

        // Neutron stars per block, summed over the blocks within about k·R.
        let mut count = vec![0f64; n];
        let (clo, chi) = (low.map(|v| (v / cell).floor() as i32), high.map(|v| (v / cell).floor() as i32));
        for (key, stars) in stars.cells_in(clo, chi) {
            let p = key.map(|k| (f64::from(k) + 0.5) * cell);
            if let Some(b) = field.block(p) {
                count[b] += stars.len() as f64;
            }
        }
        let reach = ((boost / edge).round() as i32).max(1);
        for axis in 0..3 {
            let step = [dims[1] as usize * dims[2] as usize, dims[2] as usize, 1][axis];
            let had = count.clone();
            for (b, c) in count.iter_mut().enumerate() {
                let at = (b / step) as i32 % dims[axis];
                let near = (at - reach).max(0)..=(at + reach).min(dims[axis] - 1);
                *c = near.into_iter().map(|o| had[(b as i64 + i64::from(o - at) * step as i64) as usize]).sum();
            }
        }
        let volume = (f64::from(2 * reach + 1) * edge).powi(3);

        // A boosted jump lands on the star nearest the far edge of its reach toward the
        // destination: about δ short of k·R, where a cap of depth δ (volume π·k·R·δ²) holds one
        // star on average. Short of R, normal jumps do better.
        let per_ly: Vec<f64> = count
            .iter()
            .map(|&c| {
                let step = boost - SHORT / (std::f64::consts::PI * c / volume * boost).sqrt();
                if step > range {
                    1.0 / step
                } else {
                    1.0 / range
                }
            })
            .collect();
        let per_ly = |b: usize| per_ly[b];

        // Dijkstra from the destination's block.
        let mut left = vec![f32::INFINITY; n];
        let mut open = BinaryHeap::new();
        let start = field.block(to).expect("the destination is inside the grid");
        left[start] = 0.0;
        open.push(Reverse((0u32, start)));
        let steps: Vec<([i32; 3], f64)> = (0..27)
            .map(|i| [i / 9 - 1, i / 3 % 3 - 1, i % 3 - 1])
            .filter(|d| *d != [0, 0, 0])
            .map(|d| (d, f64::from(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() * edge))
            .collect();
        while let Some(Reverse((bits, b))) = open.pop() {
            let g = f32::from_bits(bits);
            if g > left[b] {
                continue;
            }
            let at = field.coords(b);
            for &(d, len) in &steps {
                let next = [at[0] + d[0], at[1] + d[1], at[2] + d[2]];
                if (0..3).any(|a| next[a] < 0 || next[a] >= dims[a]) {
                    continue;
                }
                let nb = field.index(next);
                let ng = g + (len * (per_ly(b) + per_ly(nb)) / 2.0) as f32;
                if ng < left[nb] {
                    left[nb] = ng;
                    open.push(Reverse((ng.to_bits(), nb)));
                }
            }
        }
        field.left = left;
        field
    }

    fn index(&self, c: [i32; 3]) -> usize {
        (c[0] as usize * self.dims[1] as usize + c[1] as usize) * self.dims[2] as usize + c[2] as usize
    }

    fn coords(&self, b: usize) -> [i32; 3] {
        let (dy, dz) = (self.dims[1] as usize, self.dims[2] as usize);
        [(b / (dy * dz)) as i32, (b / dz % dy) as i32, (b % dz) as i32]
    }

    /// The block holding `p`, if it is inside the grid.
    fn block(&self, p: [f64; 3]) -> Option<usize> {
        let c: [i32; 3] = std::array::from_fn(|a| (p[a] / self.edge).floor() as i32 - self.lo[a]);
        (0..3).all(|a| c[a] >= 0 && c[a] < self.dims[a]).then(|| self.index(c))
    }

    /// Estimated jumps from `p` to the destination. Outside the grid: from the nearest block, plus
    /// normal jumps to reach it.
    fn at(&self, p: [f64; 3]) -> f64 {
        let lo = self.lo.map(|v| f64::from(v) * self.edge);
        let hi: [f64; 3] = std::array::from_fn(|a| lo[a] + f64::from(self.dims[a]) * self.edge - 1e-6);
        let inside: [f64; 3] = std::array::from_fn(|a| p[a].clamp(lo[a], hi[a]));
        let b = self.block(inside).expect("clamped into the grid");
        f64::from(self.left[b]) + dist(p, inside) / self.range
    }
}

struct Search<'a> {
    stars: &'a Stars,
    /// The estimate, once plain A* has given up.
    field: Option<Field>,
    to: [f64; 3],
    range: f64,
    /// k·R.
    boost: f64,
    /// Node ids: the stars, then the start (when it isn't on a star), then the destination.
    start: u32,
    dest: u32,
    start_pos: [f64; 3],
    /// Per node: its cost, and the node before it plus one (0: none). Zeroed allocations, so
    /// only the pages the search touches take memory.
    best: Vec<Cost>,
    prev: Vec<u32>,
    open: BinaryHeap<Reverse<Entry>>,
    /// How much the estimate counts: 1 is plain A*, more is greedier.
    weight: f64,
    /// Work since the weight last went up: stars and cells looked at, nodes taken off the queue.
    work: u64,
}

impl Search<'_> {
    fn pos(&self, n: u32) -> [f64; 3] {
        if n == self.dest {
            self.to
        } else if (n as usize) < self.stars.len() {
            self.stars.pos(n as usize)
        } else {
            self.start_pos
        }
    }

    /// How far a node's first jump reaches.
    fn first(&self, n: u32) -> f64 {
        if (n as usize) < self.stars.len() {
            self.boost
        } else {
            self.range
        }
    }

    fn leg(&self, a: u32, b: u32) -> u32 {
        leg_jumps(dist(self.pos(a), self.pos(b)), self.first(a), self.range)
    }

    fn reach(&mut self, n: u32, from: u32, c: Cost) {
        let i = n as usize;
        if self.best[i] != 0 && self.best[i] <= c {
            return;
        }
        self.best[i] = c;
        self.prev[i] = from.wrapping_add(1); // the start's (u32::MAX): none
        let p = self.pos(n);
        let left = dist(p, self.to);
        let h = match &self.field {
            _ if left <= 0.0 => 0.0,
            None => (left / self.boost).ceil(),
            Some(f) => (left / self.boost).max(f.at(p)),
        };
        self.open.push(Reverse(Entry::new(c, 0, h, left, n, self.weight)));
    }

    /// Shell `c` of node `n`: the radii of the stars `c` jumps away.
    fn shell(&self, n: u32, c: u32) -> (f64, f64) {
        let first = self.first(n);
        let outer = first + f64::from(c - 1) * self.range;
        (if c == 1 { 0.0 } else { outer - self.range }, outer)
    }

    /// Queues shell `c` of node `n` (reached at cost `g`) if it can beat going straight on. Its
    /// estimate: from the point of the shell nearest the destination.
    fn queue_shell(&mut self, n: u32, g: Cost, c: u32, direct: u32) {
        if c >= direct {
            return;
        }
        let p = self.pos(n);
        let outer = self.shell(n, c).1;
        let d = dist(p, self.to);
        let left = (d - outer).max(0.0);
        let h = match &self.field {
            _ if left <= 0.0 => 0.0,
            None => (left / self.boost).ceil(),
            Some(f) => {
                let ahead: [f64; 3] = std::array::from_fn(|a| p[a] + (self.to[a] - p[a]) * outer / d);
                (left / self.boost).max(f.at(ahead))
            }
        };
        self.open.push(Reverse(Entry::new(g, c, h, left, n, self.weight)));
    }

    /// The stars `c` jumps from node `n`: further than shell c − 1 reaches, within shell c.
    fn scan_shell(&mut self, n: u32, g: Cost, c: u32) {
        let p = self.pos(n);
        let (inner, outer) = self.shell(n, c);
        let (outer2, inner2) = (outer * outer, inner * inner);
        let edge = f64::from(self.stars.cell_ly());
        let lo = self.stars.cell_of(p.map(|v| v - outer));
        let hi = self.stars.cell_of(p.map(|v| v + outer));
        // The node before reached the stars within the shell that got here no dearer than this.
        let covered = match self.prev[n as usize] {
            0 => None,
            b => {
                let b = b - 1;
                let r = self.shell(b, self.leg(b, n)).1;
                Some((self.pos(b), r * r))
            }
        };
        self.work += (hi[0] - lo[0] + 1) as u64;
        for (key, stars) in self.stars.cells_in(lo, hi) {
            self.work += 1;
            let (near2, far2) = box_dist2(p, key, edge);
            if near2 > outer2 || far2 <= inner2 || covered.is_some_and(|(b, r2)| box_dist2(b, key, edge).1 <= r2) {
                continue;
            }
            self.work += stars.len() as u64;
            for i in stars {
                let d2 = dist2(self.stars.pos(i), p);
                if d2 <= outer2 && d2 > inner2 && i as u32 != n {
                    self.reach(i as u32, n, plus(g, c));
                }
            }
        }
    }

    /// A* to the destination. Plain A* gives up after `EXACT_WORK` (false); with the estimate,
    /// past each `WORK` the estimate counts for more (`GREEDIER`), so the search ends.
    fn run(&mut self) -> bool {
        self.best = vec![0; self.best.len()];
        self.prev = vec![0; self.prev.len()];
        self.open.clear();
        self.work = 0;
        self.reach(self.start, u32::MAX, 1);
        while let Some(Reverse(e)) = self.open.pop() {
            if self.best[e.node as usize] != e.cost {
                continue; // reached more cheaply since
            }
            if e.node == self.dest {
                return true;
            }
            self.work += 1;
            let direct = leg_jumps(dist(self.pos(e.node), self.to), self.first(e.node), self.range);
            if e.shell == 0 {
                self.reach(self.dest, e.node, plus(e.cost, direct));
                self.scan_shell(e.node, e.cost, 1);
                self.queue_shell(e.node, e.cost, 2, direct);
            } else {
                self.scan_shell(e.node, e.cost, e.shell);
                self.queue_shell(e.node, e.cost, e.shell + 1, direct);
            }
            if self.field.is_none() && self.work > EXACT_WORK {
                return false;
            }
            if self.work > WORK {
                self.work = 0;
                self.weight *= GREEDIER;
                let w = self.weight;
                let open = std::mem::take(&mut self.open);
                self.open = open.into_iter().map(|Reverse(e)| Reverse(e.reweigh(w))).collect();
            }
        }
        true // only the start, so never: the destination is always queued
    }

    /// The nodes from the start to the destination.
    fn path(&self) -> Vec<u32> {
        let mut path = vec![self.dest];
        while let Some(&last) = path.last() {
            match self.prev[last as usize] {
                0 => break,
                p => path.push(p - 1),
            }
        }
        path.reverse();
        path
    }
}

fn dist2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

/// Squared distances from `p` to the nearest and farthest points of a grid cell.
fn box_dist2(p: [f64; 3], key: [i32; 3], edge: f64) -> (f64, f64) {
    let (mut near, mut far) = (0.0, 0.0);
    for a in 0..3 {
        let lo = f64::from(key[a]) * edge;
        let hi = lo + edge;
        near += (lo - p[a]).max(p[a] - hi).max(0.0).powi(2);
        far += (p[a] - lo).abs().max((hi - p[a]).abs()).powi(2);
    }
    (near, far)
}

/// Work (stars and cells looked at, nodes taken off the queue) plain A* may do, then the work
/// before the estimate counts for more, and how much more.
const EXACT_WORK: u64 = 20_000_000;
const WORK: u64 = 25_000_000;
const GREEDIER: f64 = 1.05;

/// The route from `from` to `to` (ly) with the fewest jumps, then the fewest stops, that the
/// search finds, for a jump range of `range` ly and a neutron boost of `boost` times it. `range`
/// must be positive and `boost` at least 1.
pub fn route(stars: &Stars, from: [f64; 3], to: [f64; 3], range: f64, boost: f64) -> Route {
    assert!(range > 0.0 && boost >= 1.0, "range {range}, boost {boost}");
    let n = stars.len() as u32;
    let q = from.map(|v| (v * 32.0).round() as i32);
    let on_star = stars.cell(stars.cell_of(from)).find(|&i| stars.pos32(i) == q);
    let mut s = Search {
        stars,
        field: None,
        to,
        range,
        boost: boost * range,
        start: on_star.map_or(n, |i| i as u32),
        dest: n + 1,
        start_pos: from,
        best: vec![0; n as usize + 2],
        prev: vec![0; n as usize + 2],
        open: BinaryHeap::new(),
        weight: 1.0,
        work: 0,
    };
    let fewest = s.run();
    if !fewest {
        s.field = Some(Field::new(stars, from, to, range, boost * range));
        s.run();
    }
    let mut path = s.path();
    drop_needless_stops(&mut path, |a, b| s.leg(a, b));
    let legs = path
        .windows(2)
        .map(|w| Leg {
            star: (w[1] < n).then_some(w[1] as usize),
            jumps: s.leg(w[0], w[1]),
            ly: dist(s.pos(w[0]), s.pos(w[1])),
        })
        .collect();
    Route { start: on_star, legs, fewest }
}

/// Drops the stops between the ends of `path` that save no jumps (`leg` gives a leg's jumps):
/// the guided search can leave one.
fn drop_needless_stops(path: &mut Vec<u32>, leg: impl Fn(u32, u32) -> u32) {
    let mut i = 1;
    while i + 1 < path.len() {
        if leg(path[i - 1], path[i + 1]) <= leg(path[i - 1], path[i]) + leg(path[i], path[i + 1]) {
            path.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Plots a neutron route on this PC from `from` to `to` for `ship`, from the loaded neutron star
/// file. Errors are sentences for the Route page.
pub fn plot(file: &NeutronStars, from: &Here, to: &Here, ship: &Ship) -> Result<Plotted, String> {
    if ship.range.is_nan() || ship.range <= 0.0 {
        return Err(format!("Your {} can't jump with this cargo, so there is no route to plot.", ship.name));
    }
    let boost = f64::from(ship.fsd.supercharge_multiplier.max(1));
    let r = route(file.stars(), from.pos, to.pos, ship.range, boost);
    let stars: Vec<usize> = r.legs.iter().filter_map(|l| l.star).collect();
    let mut names = file.names(&stars).map_err(|e| format!("{e:#}"))?.into_iter();
    let boosted = |yes: bool| yes.then(|| NEUTRON.to_string());
    let mut waypoints = vec![Stop { system: from.system.clone(), label: boosted(r.start.is_some()) }];
    for leg in &r.legs {
        waypoints.push(match leg.star {
            Some(_) => Stop { system: names.next().unwrap_or_default(), label: boosted(true) },
            None => Stop { system: to.system.clone(), label: None },
        });
    }
    Ok(Plotted {
        name: format!("{} to {}", from.system, to.system),
        source: SOURCE,
        neutrons: stars.len() + usize::from(r.start.is_some()),
        jumps: r.jumps(),
        ly: dist(from.pos, to.pos),
        refuels: 0,
        waypoints,
        range: ship.range,
        local: true,
        fewest: Some(r.fewest),
        fallback: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_only_the_stops_that_save_no_jumps() {
        // Stops on a line at these ly; a leg costs a jump per 10 ly, but legs from 25 are free.
        let at = [0.0, 10.0, 20.0, 25.0, 40.0, 60.0];
        let leg = |a: u32, b: u32| {
            if a == 3 {
                0
            } else {
                ((at[b as usize] - at[a as usize]) / 10.0f64).ceil() as u32
            }
        };
        let mut path = vec![0, 1, 2, 3, 4, 5];
        drop_needless_stops(&mut path, leg);
        // 10 and 20 save nothing (0 to 25 is 3 jumps either way); 25 saves jumps; 40 doesn't.
        assert_eq!(path, [0, 3, 5]);
    }
}
