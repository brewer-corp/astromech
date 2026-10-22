//! The neutron star file reader against the fixture `scripts/gen-neutron.py` builds from the
//! sample dump (`scripts/test_gen_neutron.py` checks the script still builds the same bytes).
//! The expected stars and their order are worked out here from the sample, so a change to the
//! format or the order on either side fails one of the two tests.

use astromech_galaxy::neutron::{read_header, NeutronStars, Stars};
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// Cell, position in 1/32 ly and name of each neutron star in a sample dump, in file order.
fn expected(dump: &str) -> Vec<([i32; 3], [i32; 3], String)> {
    let text = std::fs::read_to_string(fixture(dump)).unwrap();
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
    let mut stars: Vec<_> = rows
        .iter()
        .filter(|r| r["mainStar"] == "Neutron Star")
        .map(|r| {
            let q = ["x", "y", "z"].map(|a| (r["coords"][a].as_f64().unwrap() * 32.0).round() as i32);
            (q.map(|v| v.div_euclid(128 * 32)), q, r["name"].as_str().unwrap().to_string())
        })
        .collect();
    stars.sort();
    stars
}

#[test]
fn reads_every_neutron_star_in_cell_order() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    let want = expected("neutron-sample.json");
    assert_eq!(want.len(), 35);
    assert_eq!(file.stars().len(), want.len());

    let h = file.header();
    assert_eq!(h, read_header(&fixture("neutron-sample.bin")).unwrap());
    assert_eq!(h.cell_ly, 128);
    assert_eq!(h.stars, 35);
    assert_eq!(h.name_group, 256);
    let built = chrono::DateTime::from_timestamp(h.built, 0).unwrap();
    assert_eq!(built.format("%Y-%m-%d %H:%M:%S").to_string(), "2026-10-22 00:00:00");

    for (i, (cell, q, name)) in want.iter().enumerate() {
        assert_eq!(file.stars().pos32(i), *q, "star {i}");
        assert_eq!(file.name(i).unwrap(), *name, "star {i}");
        assert_eq!(file.stars().pos(i), q.map(|v| f64::from(v) / 32.0));
        assert_eq!(file.stars().cell_of(file.stars().pos(i)), *cell, "{name}");
        let range = file.stars().cell(*cell);
        assert!(range.contains(&i), "{name} outside its cell's range {range:?}");
        assert!(range.clone().all(|j| want[j].0 == *cell), "cell {cell:?} range {range:?} has other stars");
        assert_eq!(range.len(), want.iter().filter(|w| w.0 == *cell).count(), "cell {cell:?}");
    }
    assert!(file.name(want.len()).is_err());
}

/// 600 synthetic stars: three groups of names, so a group size that changes on one side only
/// gives every name after star 255 wrong.
#[test]
fn reads_names_across_name_groups() {
    let file = NeutronStars::open(&fixture("neutron-synthetic.bin")).unwrap();
    let want = expected("neutron-synthetic.json");
    assert_eq!(want.len(), 600);
    assert_eq!((file.stars().len(), file.header().name_group), (600, 256));
    for (i, (cell, q, name)) in want.iter().enumerate() {
        assert_eq!(file.stars().pos32(i), *q, "star {i}");
        assert_eq!(file.name(i).unwrap(), *name, "star {i}");
        assert!(file.stars().cell(*cell).contains(&i), "star {i}");
    }
}

#[test]
fn drops_the_systems_without_a_neutron_main_star() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    let names: Vec<String> = (0..file.stars().len()).map(|i| file.name(i).unwrap()).collect();
    for dropped in ["Pyroomoa AA-A h0", "Schao Pruae YE-A g0", "HD 117024", "HR 6164"] {
        assert!(!names.iter().any(|n| n == dropped), "{dropped} is in the file");
    }
    assert!(names.iter().any(|n| n == "Jackson's Lighthouse"));
}

#[test]
fn a_cell_with_no_stars_is_empty() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    assert_eq!(file.stars().cell(file.stars().cell_of([0.0, 0.0, 0.0])), 0..0);
    assert_eq!(file.stars().cell([i32::MAX, 0, 0]), 0..0);
    // Sol's cell is empty, but Lalande 25224 (22, 183, 65 ly) is one up in y.
    let r = file.stars().cell([0, 1, 0]);
    assert_eq!(r.len(), 1);
    assert_eq!(file.name(r.start).unwrap(), "Lalande 25224");
}

#[test]
fn refuses_other_files() {
    let dir = tempfile::tempdir().unwrap();
    let good = std::fs::read(fixture("neutron-sample.bin")).unwrap();

    let bad = dir.path().join("magic.bin");
    std::fs::write(&bad, [b"NOTNEUTR".as_slice(), &good[8..]].concat()).unwrap();
    assert!(NeutronStars::open(&bad).is_err());
    assert!(read_header(&bad).is_err());

    let older = dir.path().join("v1.bin");
    std::fs::write(&older, [&good[..8], &1u32.to_le_bytes(), &good[12..]].concat()).unwrap();
    assert!(NeutronStars::open(&older).is_err());

    let newer = dir.path().join("v3.bin");
    std::fs::write(&newer, [&good[..8], &3u32.to_le_bytes(), &good[12..]].concat()).unwrap();
    assert!(NeutronStars::open(&newer).err().unwrap().to_string().contains("version 3"));

    let short = dir.path().join("short.bin");
    std::fs::write(&short, &good[..100]).unwrap();
    assert!(NeutronStars::open(&short).is_err());
}

/// `cells_in` gives every non-empty cell in a box, and only those, against a scan of every cell
/// key in the box.
#[test]
fn finds_the_cells_in_a_box() {
    let file = NeutronStars::open(&fixture("neutron-synthetic.bin")).unwrap();
    let stars = file.stars();
    let keys: Vec<[i32; 3]> = (0..stars.len()).map(|i| stars.cell_of(stars.pos(i))).collect();
    // Exactly around the stars, wider, slices, and an empty box.
    let (lo, hi) = (
        [0, 1, 2].map(|a| keys.iter().map(|k| k[a]).min().unwrap()),
        [0, 1, 2].map(|a| keys.iter().map(|k| k[a]).max().unwrap()),
    );
    let wide = (lo.map(|v| v - 1), hi.map(|v| v + 1));
    for (blo, bhi) in
        [(lo, hi), wide, ([lo[0], lo[1], 0], [hi[0], 0, hi[2]]), ([1, -2, -1], [1, 0, 1]), ([5, 5, 5], [4, 9, 9])]
    {
        let mut want = vec![];
        for x in blo[0]..=bhi[0] {
            for y in blo[1]..=bhi[1] {
                for z in blo[2]..=bhi[2] {
                    let r = stars.cell([x, y, z]);
                    if !r.is_empty() {
                        want.push(([x, y, z], r));
                    }
                }
            }
        }
        let got: Vec<_> = stars.cells_in(blo, bhi).collect();
        assert_eq!(got, want, "{blo:?}..={bhi:?}");
    }
    let all: usize = stars.cells_in(lo, hi).map(|(_, r)| r.len()).sum();
    assert_eq!(all, stars.len());
}

#[test]
fn indexes_stars_given_in_any_order() {
    let q = [[40 * 32, 0, 0], [-5 * 32, 3, 9], [129 * 32, -1, 0], [0, 0, 0]];
    let stars = Stars::new(128, q.to_vec()).unwrap();
    assert_eq!(stars.len(), 4);
    let order: Vec<[i32; 3]> = (0..4).map(|i| stars.pos32(i)).collect();
    assert_eq!(order, [[-5 * 32, 3, 9], [0, 0, 0], [40 * 32, 0, 0], [129 * 32, -1, 0]]);
    assert_eq!(stars.cell([0, 0, 0]), 1..3);
    assert_eq!(stars.cell([1, -1, 0]), 3..4, "y just below 0 is cell −1");
    assert!(Stars::new(128, vec![[i32::MAX, 0, 0]]).is_err(), "outside the grid");
}
