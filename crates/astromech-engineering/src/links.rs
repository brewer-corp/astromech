//! Import links that open a journal `Loadout` event in the community ship builders.
//!
//! Both sites take the raw event JSON, compressed and base64-encoded, so the build opens exactly
//! as the game wrote it, modules, priorities and engineering included.
//!
//! - Coriolis (<https://coriolis.io>, MIT): `/import?data=<base64 of zlib-deflated JSON>`. Its
//!   router reads the `data` query parameter, base64-decodes and inflates it, then builds the ship
//!   from the `Ship` and `Modules` fields.
//! - EDSY (<https://edsy.org>): `/#/I=<base64url of gzipped JSON>`, the "import a Loadout event"
//!   fragment (checked in a browser: EDSY lists the found builds in a dialog, and OKAY loads one).

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use serde_json::Value;
use std::io::Write;

fn compact(loadout: &Value) -> Vec<u8> {
    serde_json::to_vec(loadout).unwrap_or_default()
}

fn deflate<W: Write, F: FnOnce(Vec<u8>) -> W>(raw: &[u8], new: F, done: impl FnOnce(W) -> Vec<u8>) -> Vec<u8> {
    let mut enc = new(Vec::new());
    // Writing to a Vec cannot fail.
    let _ = enc.write_all(raw);
    done(enc)
}

/// Coriolis import URL for a `Loadout` event.
pub fn coriolis_url(loadout: &Value) -> String {
    let z =
        deflate(&compact(loadout), |v| ZlibEncoder::new(v, Compression::default()), |e| e.finish().unwrap_or_default());
    let b64 = STANDARD.encode(z);
    // `+`, `/` and `=` are query-string metacharacters; Coriolis percent-decodes the parameter.
    let q = b64.replace('+', "%2B").replace('/', "%2F").replace('=', "%3D");
    format!("https://coriolis.io/import?data={q}")
}

/// EDSY import URL for a `Loadout` event.
pub fn edsy_url(loadout: &Value) -> String {
    let z =
        deflate(&compact(loadout), |v| GzEncoder::new(v, Compression::default()), |e| e.finish().unwrap_or_default());
    format!("https://edsy.org/#/I={}", URL_SAFE_NO_PAD.encode(z))
}

/// URL prefixes the app may open in the browser for the builder and lookup links. The trailing
/// separator in each keeps look-alike hosts (`https://spansh.co.uk.evil/`) out.
const OPENABLE: &[&str] =
    &["https://coriolis.io/import?", "https://edsy.org/#/I=", "https://spansh.co.uk/", "https://inara.cz/elite/"];

/// Whether `url` is one of the links built here (the shell checks this before launching a browser).
pub fn is_openable_url(url: &str) -> bool {
    OPENABLE.iter().any(|p| url.starts_with(p)) && !url.chars().any(|c| c.is_control() || c == ' ')
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64URL;
    use flate2::read::{GzDecoder, ZlibDecoder};
    use std::io::Read;

    fn fixture() -> Value {
        serde_json::from_str(include_str!("../tests/links/loadout-asp.json")).unwrap()
    }

    #[test]
    fn coriolis_round_trips_the_loadout() {
        let url = coriolis_url(&fixture());
        let data = url.strip_prefix("https://coriolis.io/import?data=").unwrap();
        let b64 = data.replace("%2B", "+").replace("%2F", "/").replace("%3D", "=");
        let mut json = String::new();
        ZlibDecoder::new(&STANDARD.decode(b64).unwrap()[..]).read_to_string(&mut json).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), fixture());
    }

    #[test]
    fn edsy_round_trips_the_loadout() {
        let url = edsy_url(&fixture());
        let data = url.strip_prefix("https://edsy.org/#/I=").unwrap();
        let mut json = String::new();
        GzDecoder::new(&B64URL.decode(data).unwrap()[..]).read_to_string(&mut json).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), fixture());
    }

    #[test]
    fn only_our_links_are_openable() {
        let f = fixture();
        for ok in [
            coriolis_url(&f),
            edsy_url(&f),
            "https://spansh.co.uk/system/10477373803".to_string(),
            "https://spansh.co.uk/search/Col%2070%20Sector".to_string(),
            "https://inara.cz/elite/starsystem/?search=Sol".to_string(),
            "https://inara.cz/elite/stations/?search=Daedalus".to_string(),
        ] {
            assert!(is_openable_url(&ok), "{ok}");
        }
        for bad in [
            "https://coriolis.io.evil.com/import?data=x",
            "https://coriolis.io/outfit/asp",
            "https://coriolis.io@evil.com/import?data=x",
            "https://edsy.org.evil/#/I=abc",
            "https://edsy.org/#/L=abc",
            "https://spansh.co.uk.evil/system/1",
            "https://spansh.co.uk@evil.com/",
            "https://inara.cz/",
            "https://inara.cz.evil/elite/",
            "http://spansh.co.uk/system/1",
            "file:///C:/Windows/System32/calc.exe",
            "https://spansh.co.uk/ --evil",
            "https://spansh.co.uk/\nx",
            "",
        ] {
            assert!(!is_openable_url(bad), "{bad}");
        }
    }

    /// The URLs opened by hand in a browser (evidence on astromech-9mb9) still decode to the
    /// fixture, and the builders emit the same URL shape. The compressed bytes are not compared:
    /// they depend on which deflate backend cargo's feature unification picks for flate2.
    #[test]
    fn urls_snapshot() {
        let f = fixture();
        let unq = |s: &str| s.replace("%2B", "+").replace("%2F", "/").replace("%3D", "=");
        let mut json = String::new();
        let z =
            STANDARD.decode(unq(SNAPSHOT_CORIOLIS.strip_prefix("https://coriolis.io/import?data=").unwrap())).unwrap();
        ZlibDecoder::new(&z[..]).read_to_string(&mut json).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), f);
        let mut json = String::new();
        let z = B64URL.decode(SNAPSHOT_EDSY.strip_prefix("https://edsy.org/#/I=").unwrap()).unwrap();
        GzDecoder::new(&z[..]).read_to_string(&mut json).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), f);

        // zlib streams start 0x78 0x9c (base64 "eJ"), gzip 0x1f 0x8b (base64url "H4s").
        assert!(coriolis_url(&f).starts_with("https://coriolis.io/import?data=eJ"));
        assert!(edsy_url(&f).starts_with("https://edsy.org/#/I=H4s"));
        assert!(!coriolis_url(&f)["https://coriolis.io/import?data=".len()..].contains(['+', '/', '=']));
    }

    const SNAPSHOT_CORIOLIS: &str =
        "https://coriolis.io/import?data=eJy9lllPIzkQgP8K8nOD3FeuN4YAYZUsTJpdrXY0iky6klg4dssHQwbx36fsJqSzmyHJPMxTt%2B2q%2BmzX4XohF0zP1QWr2JTbFenFrYhcORCbmRcyYlySXpqc0YiMwYB%2BAtKjZ630NSIDJ8QAmLAL1PUCfuJvJhyKtOIsb3fTCA08%2F%2BGW1ZjJOU5n1MuNVOkEGNL78kK2DNxYWJIeOTfV5FwvldOTa81KSElEbnEbVjuIyJ3mStcbjkghlPUaQRrF3vB5t5vTtPMa7QbcSDu5U99AV4Lhb8G%2FQz65EMyYfC8q6N15vQ0u7nQ7nSz9CHcp51zCcSh%2F%2BbWe2cmq10BzOfe%2B%2BoTrFQ7sTR%2FFkk6rnba6WbSZ%2F5Mt0QC5KvqToZLz2ifRuxm%2FBIJ7159cMW381GbVG00pjSme6%2FK5QuoSpGXicjaDqd%2BtqWDKmZjMTDlZAHtakV2CSJ4ywQ2U4YDGnIyYZPOAGsITCHReiBA%2B46DrGBmyBz8fxIOYMTfmWqkyXNat5rhDto68JITYepCfUX9TawvoCZiHa942Q%2F9nJt6y0%2B5s2wlB0Nfs297t0LM8ezeDiZM3zaAnbivLl0zsONqOPdG8uak4a%2BPw9WtEPju80hA7fps%2FjcHBCr1Rav50UBzS9zi80hg4xYLPbN8rN9MsadM8%2BSjuh3wGhasqpes8y2poshfaUGwEP2236d6s7nNjNX9wVukm88DcbmhvwGmaJq3sI3IB0ihtmje7%2F5BjVrIGJUnyzofH8%2BX5nsnHJmV%2FdVxrbUDdFqU%2FBQ0qvEcnDAwZFvzJNV8%2BYFqPoORuub9mBbEB02WlsOTETd%2FR3wRNjoGOnLD8gkmp5JpaYEKKvdAg9WsHxVwCUX5SylioQ5QeGqL3XK5%2BDYrztuDyccicnC6Qe%2B%2B0BjvxFo%2FDHny%2FIS%2FCWa9BgmbrhGwdelr%2FoXGtcwzUR3wxVao67sENuKTWOQY3hopx%2FebK9ChaWuscQwtt25hNH5u4%2BDBcdjzO9zpgz6tKKzZdFI5b2F9Igw7Tq%2F%2BqbVO%2FvveC67oU5kOv%2BeDQYjuvhxhEFZplpiL1wHcjWfL2X2J%2F4cvcP6dZ8ra%2BbnX4s3UaTs6D4l9SYDcpw1uLrWh4S7HlCMpDxUrl%2FFOD7zEYy5YemNCkdRrTU5re026P0h7N%2FyWvPwDl7ZHb";
    const SNAPSHOT_EDSY: &str =
        "https://edsy.org/#/I=H4sIAAAAAAAC_72WWU8jORCA_wryc4PcV643hgBhlSxMml2tdjSKTLqSWDh2ywdDBvHfp-wmpLObIck8zFO37ar6bNfheiEXTM_VBavYlNsV6cWtiFw5EJuZFzJiXJJempzRiIzBgH4C0qNnrfQ1IgMnxACYsAvU9QJ-4m8mHIq04ixvd9MIDTz_4ZbVmMk5TmfUy41U6QQY0vvyQrYM3FhYkh45N9XkXC-V05NrzUpISURucRtWO4jIneZK1xuOSCGU9RpBGsXe8Hm3m9O08xrtBtxIO7lT30BXguFvwb9DPrkQzJh8Lyro3Xm9DS7udDudLP0IdynnXMJxKH_5tZ7ZyarXQHM59776hOsVDuxNH8WSTqudtrpZtJn_ky3RALkq-pOhkvPaJ9G7Gb8EgnvXn1wxbfzUZtUbTSmNKZ7r8rlC6hKkZeJyNoOp362pYMqZmMxMOVkAe1qRXYJInjLBDZThgMacjJhk84AawhMIdF6IED7joOsYGbIHPx_Eg5gxN-ZaqTJc1q3muEO2jrwkhNh6kJ9Rf1NrC-gJmIdr3jZD_2cm3rLT7mzbCUHQ1-zb3u3Qszx7N4OJkzfNoCduK8uXTOw42o490by5qThr4_D1a0Q-O7zSEDt-mz-NwcEKvVFq_nRQHNL3OLzSGDjFgs9s3ys30yxp0zz5KO6HfAaFqyql6zzLamiyF9pQbAQ_bbfp3qzuc2M1f3BW6SbzwNxuaG_AaZomrewjcgHSKG2aN7v_kGNWsgYlSfLOh8fz5fmeyccmZX91XGttQN0WpT8FDSq8RycMDBkW_Mk1Xz5gWo-g5G65v2YFsQHTZaWw5MRN39HfBE2OgY6csPyCSankmlpgQoq90CD1awfFXAJRflLKWKhDlB4aovdcrn4NivO24PJxyJycLpB777QGO_EWj8MefL8hL8JZr0GCZuuEbB16Wv-hca1zDNRHfDFVqjruwQ24pNY5BjeGinH95sr0KFpa6xxDC23bmE0fm7j4MFx2PM73OmDPq0orNl0UjlvYX0iDDtOr_6ptU7--94LruhTmQ6_54NBiO6-HGEQVmmWmIvXAdyNZ8vZfYn_hy9w_p1nytr5udfizdRpOzoPiX1JgNynDW4utaHhLseUIykPFSuX8U4PvMRjLlh6Y0KR1GtNTmt7Tbo_SHs3_Ja8_AAxx51g6CwAA";
}
