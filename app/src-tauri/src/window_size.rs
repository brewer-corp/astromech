//! The window's sizes in logical px: DESIGN.md's 1100 x 700 minimum, which every page is
//! designed and checked at, and the size a new window opens at. A display whose work area
//! leaves less room than either gets the window at the room it has instead, and the page
//! scrolls; tao sets the minimum as asked, so a minimum larger than the display would hold the
//! window partly off screen (astromech-7o1g).

/// DESIGN.md's minimum window (Frame).
pub const MINIMUM: (f64, f64) = (1100.0, 700.0);
/// The size a new window opens at.
pub const START: (f64, f64) = (1200.0, 800.0);

/// The minimum and the opening size for a window with `room`: the inner size the display's
/// work area leaves once the window's own frame (title bar, borders) is taken off.
pub fn fit(room: (f64, f64)) -> Fit {
    let cap = |(w, h): (f64, f64)| (w.min(room.0), h.min(room.1));
    Fit { min: cap(MINIMUM), start: cap(START) }
}

#[derive(Debug, PartialEq)]
pub struct Fit {
    pub min: (f64, f64),
    pub start: (f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_display_with_room_keeps_the_design_minimum() {
        assert_eq!(fit((1920.0, 1009.0)), Fit { min: (1100.0, 700.0), start: (1200.0, 800.0) });
    }

    /// 1366 x 768 at 100 percent: a 40 px taskbar and a 31 px title bar leave 697 px of height.
    #[test]
    fn a_short_display_gets_the_window_at_its_room() {
        assert_eq!(fit((1350.0, 697.0)), Fit { min: (1100.0, 697.0), start: (1200.0, 697.0) });
    }
}
