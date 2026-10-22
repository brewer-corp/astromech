//! The neutron star file reader against the fixture `scripts/gen-neutron.py` builds from the
//! sample dump (`scripts/test_gen_neutron.py` checks the script still builds the same bytes).
//! The expected stars and their order are worked out here from the sample, so a change to the
//! format or the order on either side fails one of the two tests.

use astromech_galaxy::neutron::{read_header, NeutronStars};
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// Cell, position in 1/32 ly and name of each neutron star in the sample, in file order.
fn expected() -> Vec<([i32; 3], [i32; 3], String)> {
    let text = std::fs::read_to_string(fixture("neutron-sample.json")).unwrap();
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
    let want = expected();
    assert_eq!(want.len(), 35);
    assert_eq!(file.len(), want.len());

    let h = file.header();
    assert_eq!(h, read_header(&fixture("neutron-sample.bin")).unwrap());
    assert_eq!(h.cell_ly, 128);
    assert_eq!(h.stars, 35);
    let built = chrono::DateTime::from_timestamp(h.built, 0).unwrap();
    assert_eq!(built.format("%Y-%m-%d %H:%M:%S").to_string(), "2026-10-22 00:00:00");

    for (i, (cell, q, name)) in want.iter().enumerate() {
        assert_eq!(file.pos32(i), *q, "star {i}");
        assert_eq!(file.name(i).unwrap(), *name, "star {i}");
        assert_eq!(file.pos(i), q.map(|v| f64::from(v) / 32.0));
        assert_eq!(file.cell_of(file.pos(i)), *cell, "{name}");
        let range = file.cell(*cell);
        assert!(range.contains(&i), "{name} outside its cell's range {range:?}");
        assert!(range.clone().all(|j| want[j].0 == *cell), "cell {cell:?} range {range:?} has other stars");
        assert_eq!(range.len(), want.iter().filter(|w| w.0 == *cell).count(), "cell {cell:?}");
    }
    assert!(file.name(want.len()).is_err());
}

#[test]
fn drops_the_systems_without_a_neutron_main_star() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    let names: Vec<String> = (0..file.len()).map(|i| file.name(i).unwrap()).collect();
    for dropped in ["Pyroomoa AA-A h0", "Schao Pruae YE-A g0", "HD 117024", "HR 6164"] {
        assert!(!names.iter().any(|n| n == dropped), "{dropped} is in the file");
    }
    assert!(names.iter().any(|n| n == "Jackson's Lighthouse"));
}

#[test]
fn a_cell_with_no_stars_is_empty() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    assert_eq!(file.cell(file.cell_of([0.0, 0.0, 0.0])), 0..0);
    assert_eq!(file.cell([i32::MAX, 0, 0]), 0..0);
    // Sol's cell is empty, but Lalande 25224 (22, 183, 65 ly) is one up in y.
    let r = file.cell([0, 1, 0]);
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

    let newer = dir.path().join("v2.bin");
    std::fs::write(&newer, [&good[..8], &2u32.to_le_bytes(), &good[12..]].concat()).unwrap();
    assert!(NeutronStars::open(&newer).err().unwrap().to_string().contains("version 2"));

    let short = dir.path().join("short.bin");
    std::fs::write(&short, &good[..100]).unwrap();
    assert!(NeutronStars::open(&short).is_err());
}
