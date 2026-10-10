//! Sample voice commands (S9 Test 1) and spoken confirmations (S9 Test 2).

pub const COMMANDS: &[&str] = &[
    "landing gear down",
    "lights on",
    "plot route to Sol",
    "deploy hardpoints",
    "request docking",
    "open cargo scoop",
    "full throttle",
    "set speed to fifty percent",
    "target next hostile",
    "silent running on",
    "night vision on",
    "open galaxy map",
    "jump to hyperspace",
    "launch fighter",
    "retract hardpoints",
];

pub const CONFIRMATIONS: &[&str] = &[
    "Landing gear deployed.",
    "Lights on.",
    "Route plotted to Sol.",
    "Hardpoints deployed.",
    "Docking requested.",
    "Cargo scoop open.",
    "Frame shift drive charging.",
    "Silent running engaged.",
    "Sorry, I did not catch that. Please repeat.",
    "Kill switch engaged. All voice actions stopped.",
];
