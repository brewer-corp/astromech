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
        "https://coriolis.io/import?data=eJy9Vt1uKjcQfpXI15vILCyccJcTkpAKmpRNq6pVhSbsAFa89so%2FOaFR3r1jLySg5rBwLg43rO35vhnPr1%2FZJZiFvoQKZsKtWL%2FVTdi1R%2Fmx88rGIBTrt9MznrAJWjTPyPr8rNt%2BS9jQSzlEkG5J2CAQNv4A6Umk2%2BpkvfN2QgQvv%2FiymoBa0HaHB7mxLrxEy%2Fp%2Fv7IdgluHJeuzC1tNL0ypvZneGCiwzRJ2R2Y44zFh90ZoUxucsFxqFxBRmr0lnxPeKje919%2FQVBLoMxf%2FYja9lGBt1kgdcfcBt5f%2BSi2EwuOog3NrnI3c9TcaoRbB91%2FJkxUt3O2AUOmXbq%2FdPe8kH%2Fu%2FQkk%2BZdf5YDrSalH7OHmnCUcoRQjlyTUYG7Y%2BTgNpm%2FMWp3tcvVSktUTlQF7N5zgL1tkKZwLkdG6L6RLhecU%2BEyTNM5DCYhEvZO3JGBQsoqoRPqNk%2FSxGXMwFmjrmI3gM%2B1E8ill7a2%2B0LqJz7owgC2GTSWlMmc0iO%2BPBUxsG8jwuolt3afj%2FaFo7PL0vuzwxyAMD3xrN4WdZ552GCiHbpqFI3FVOlCA%2FudonNvFs26hWp0fLt38S9psnl8ZcCWZ%2BN%2BeGK4pGYcTzQXnH3%2FPu2lDi5Esxd4MA3pvXIzHH3FeVNnXddGolaaOSLWBzXQ6EdUY8eqfND1TnFnqvqhyV1cYed48JFLCfNvTMB1BP25Y3t6wN6rvUw4qc46XFEVDfnd6I8pGqcYyF8GVza4liQzBFpalTtH6KlnSvlrGXTlyCUlpt1ORUKbJRS5Q68CqU1SiLr1pbh3Uq8UNT6UGo1YFaaN%2FlQj2NwKvZkhQ9eGPQTQPFcXrS%2FQkbb3ODCg1sSqN76H3CH2%2FVmMb8zWdaV8eVXuRPa8xe%2FglWIMw6HO2j6Ns1Zi99fMVMYPa0zd86jL9zAH%2BY%2FeguqspomC1zLxw2t6WIAbPahYXevn78rHv%2BOY%2B%2F8Lh69ETRy%2BolRb4iHrAVqxdhXHfS9XdBAzi0kD9PO%2Bn6fPMWEC%2FOGzy5iMDflaTnk4rDiN5ecdjQTI7gkYZCe0dSNLDQOiiDwpSn3dMWP%2BXtB37e57zPs7%2FY23%2Bbu0zI";
    const SNAPSHOT_EDSY: &str =
        "https://edsy.org/#/I=H4sIAAAAAAAA_71W3W4qNxB-lcjXm8gsLJxwlxOSkAqalE2rqlWFJuwAVrz2yj85oVHevWMvJKDmsHAuDjes7fm-Gc-vX9klmIW-hApmwq1Yv9VN2LVH-bHzysYgFOu30zOesAlaNM_I-vys235L2NBLOUSQbknYIBA2_gDpSaTb6mS983ZCBC-_-LKagFrQdocHubEuvETL-n-_sh2CW4cl67MLW00vTKm9md4YKLDNEnZHZjjjMWH3RmhTG5ywXGoXEFGavSWfE94qN73X39BUEugzF_9iNr2UYG3WSB1x9wG3l_5KLYTC46iDc2ucjdz1NxqhFsH3X8mTFS3c7YBQ6Zdur9097yQf-79CST5l1_lgOtJqUfs4eacJRyhFCOXJNRgbtj5OA2mb8xane1y9VKS1ROVAXs3nOAvW2QpnAuR0bovpEuF5xT4TJM0zkMJiES9k7ckYFCyiqhE-o2T9LEZczAWaOuYjeAz7UTyKWXtrb7QuonPujCALYZNJaUyZzSI748FTGwbyPC6iW3dp-P9oWjs8vS-7PDHIAwPfGs3hZ1nnnYYKIdumoUjcVU6UID-52ic28WzbqFanR8u3fxL2myeXxlwJZn4354YrikZhxPNBecff8-7aUOLkSzF3gwDem9cjMcfcV5U2dd10aiVpo5ItYHNdDoR1Rjx6p80PVOcWeq-qHJXVxh53jwkUsJ829MwHUE_blje3rA3qu9TDipzjpcURUN-d3ojykapxjIXwZXNriWJDMEWlqVO0foqWdK-WsZdOXIJSWm3U5FQpslFLlDrwKpTVKIuvWluHdSrxQ1PpQajVgVpo3-VCPY3Aq9mSFD14Y9BNA8VxetL9CRtvc4MKDWxKo3vofcIfb9WYxvzNZ1pXx5Ve5E9rzF7-CVYgzDoc7aPo2zVmL318xUxg9rTN3zqMv3MAf5j96C6qymiYLXMvHDa3pYgBs9qFhd6-fvyse_45j7_wuHr0RNHL6iVFviIesBWrF2Fcd9L1d0EDOLSQP0876fp88xYQL84bPLmIwN-VpOeTisOI3l5x2NBMjuCRhkJ7R1I0sNA6KIPClKfd0xY_5e0Hft7nvM-zv9jbf6XzD4srCgAA";
}
