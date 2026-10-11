//! Ship engineering materials as the three material traders lay them out.
//!
//! Journal `Name` values are matched after stripping to lowercase ASCII letters. Display names
//! and trader rows come from community ID tables (EDCD/FDevIDs), not Frontier assets.

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Raw,
    Manufactured,
    Encoded,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Raw => "raw",
            Kind::Manufactured => "manufactured",
            Kind::Encoded => "encoded",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Kind::Raw => "Raw",
            Kind::Manufactured => "Manufactured",
            Kind::Encoded => "Encoded",
        }
    }

    pub fn trader(self) -> &'static str {
        match self {
            Kind::Raw => "Raw material trader",
            Kind::Manufactured => "Manufactured material trader",
            Kind::Encoded => "Encoded material trader",
        }
    }

    pub fn max_grade(self) -> u8 {
        match self {
            Kind::Raw => 4,
            Kind::Manufactured | Kind::Encoded => 5,
        }
    }

    pub fn from_journal(s: &str) -> Option<Self> {
        let t = s.to_ascii_lowercase();
        if t.contains("raw") {
            Some(Kind::Raw)
        } else if t.contains("manuf") {
            Some(Kind::Manufactured)
        } else if t.contains("encod") {
            Some(Kind::Encoded)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spec {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: Kind,
    pub group: &'static str,
    pub grade: u8,
    pub on_trader: bool,
}

pub fn capacity(grade: u8) -> i64 {
    match grade {
        1 => 300,
        2 => 250,
        3 => 200,
        4 => 150,
        5 => 100,
        _ => 300,
    }
}

pub fn rarity(grade: u8) -> &'static str {
    match grade {
        1 => "Very common",
        2 => "Common",
        3 => "Standard",
        4 => "Rare",
        5 => "Very rare",
        _ => "Unknown",
    }
}

/// Journal name → catalogue id (lowercase, no punctuation).
pub fn key(name: &str) -> String {
    name.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect()
}

pub fn lookup(name: &str) -> Option<&'static Spec> {
    index().get(&key(name)).copied()
}

pub fn catalog() -> &'static [Spec] {
    CATALOG
}

pub fn groups(kind: Kind) -> Vec<&'static str> {
    let mut out = vec![];
    for s in CATALOG {
        if s.kind == kind && s.on_trader && !out.contains(&s.group) {
            out.push(s.group);
        }
    }
    out
}

fn index() -> &'static std::collections::HashMap<String, &'static Spec> {
    static I: std::sync::OnceLock<std::collections::HashMap<String, &'static Spec>> = std::sync::OnceLock::new();
    I.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        for s in CATALOG {
            m.insert(s.id.to_string(), s);
        }
        for (alias, id) in ALIASES {
            if let Some(s) = CATALOG.iter().find(|x| x.id == *id) {
                m.insert((*alias).to_string(), s);
            }
        }
        m
    })
}

const ALIASES: &[(&str, &str)] = &[
    ("proprietarycomposites", "fedproprietarycomposites"),
    ("coredynamicscomposites", "fedcorecomposites"),
    ("flawedfocuscrystals", "uncutfocuscrystals"),
    ("exquisitecrystals", "exquisitefocuscrystals"),
];

macro_rules! specs {
    ($($id:ident, $name:expr, $kind:ident, $group:expr, $grade:expr, $trader:expr;)*) => {
        &[ $( Spec { id: stringify!($id), name: $name, kind: Kind::$kind, group: $group, grade: $grade, on_trader: $trader } ),* ]
    };
}

static CATALOG: &[Spec] = specs! {
    carbon, "Carbon", Raw, "Category 1", 1, true;
    vanadium, "Vanadium", Raw, "Category 1", 2, true;
    niobium, "Niobium", Raw, "Category 1", 3, true;
    yttrium, "Yttrium", Raw, "Category 1", 4, true;
    phosphorus, "Phosphorus", Raw, "Category 2", 1, true;
    chromium, "Chromium", Raw, "Category 2", 2, true;
    molybdenum, "Molybdenum", Raw, "Category 2", 3, true;
    technetium, "Technetium", Raw, "Category 2", 4, true;
    sulphur, "Sulphur", Raw, "Category 3", 1, true;
    manganese, "Manganese", Raw, "Category 3", 2, true;
    cadmium, "Cadmium", Raw, "Category 3", 3, true;
    ruthenium, "Ruthenium", Raw, "Category 3", 4, true;
    iron, "Iron", Raw, "Category 4", 1, true;
    zinc, "Zinc", Raw, "Category 4", 2, true;
    tin, "Tin", Raw, "Category 4", 3, true;
    selenium, "Selenium", Raw, "Category 4", 4, true;
    nickel, "Nickel", Raw, "Category 5", 1, true;
    germanium, "Germanium", Raw, "Category 5", 2, true;
    tungsten, "Tungsten", Raw, "Category 5", 3, true;
    tellurium, "Tellurium", Raw, "Category 5", 4, true;
    rhenium, "Rhenium", Raw, "Category 6", 1, true;
    arsenic, "Arsenic", Raw, "Category 6", 2, true;
    mercury, "Mercury", Raw, "Category 6", 3, true;
    polonium, "Polonium", Raw, "Category 6", 4, true;
    lead, "Lead", Raw, "Category 7", 1, true;
    zirconium, "Zirconium", Raw, "Category 7", 2, true;
    boron, "Boron", Raw, "Category 7", 3, true;
    antimony, "Antimony", Raw, "Category 7", 4, true;

    chemicalstorageunits, "Chemical Storage Units", Manufactured, "Chemical", 1, true;
    chemicalprocessors, "Chemical Processors", Manufactured, "Chemical", 2, true;
    chemicaldistillery, "Chemical Distillery", Manufactured, "Chemical", 3, true;
    chemicalmanipulators, "Chemical Manipulators", Manufactured, "Chemical", 4, true;
    pharmaceuticalisolators, "Pharmaceutical Isolators", Manufactured, "Chemical", 5, true;
    temperedalloys, "Tempered Alloys", Manufactured, "Thermic", 1, true;
    heatresistantceramics, "Heat Resistant Ceramics", Manufactured, "Thermic", 2, true;
    precipitatedalloys, "Precipitated Alloys", Manufactured, "Thermic", 3, true;
    thermicalloys, "Thermic Alloys", Manufactured, "Thermic", 4, true;
    militarygradealloys, "Military Grade Alloys", Manufactured, "Thermic", 5, true;
    heatconductionwiring, "Heat Conduction Wiring", Manufactured, "Heat", 1, true;
    heatdispersionplate, "Heat Dispersion Plate", Manufactured, "Heat", 2, true;
    heatexchangers, "Heat Exchangers", Manufactured, "Heat", 3, true;
    heatvanes, "Heat Vanes", Manufactured, "Heat", 4, true;
    protoheatradiators, "Proto Heat Radiators", Manufactured, "Heat", 5, true;
    basicconductors, "Basic Conductors", Manufactured, "Conductive", 1, true;
    conductivecomponents, "Conductive Components", Manufactured, "Conductive", 2, true;
    conductiveceramics, "Conductive Ceramics", Manufactured, "Conductive", 3, true;
    conductivepolymers, "Conductive Polymers", Manufactured, "Conductive", 4, true;
    biotechconductors, "Biotech Conductors", Manufactured, "Conductive", 5, true;
    mechanicalscrap, "Mechanical Scrap", Manufactured, "Mechanical Components", 1, true;
    mechanicalequipment, "Mechanical Equipment", Manufactured, "Mechanical Components", 2, true;
    mechanicalcomponents, "Mechanical Components", Manufactured, "Mechanical Components", 3, true;
    configurablecomponents, "Configurable Components", Manufactured, "Mechanical Components", 4, true;
    improvisedcomponents, "Improvised Components", Manufactured, "Mechanical Components", 5, true;
    gridresistors, "Grid Resistors", Manufactured, "Capacitors", 1, true;
    hybridcapacitors, "Hybrid Capacitors", Manufactured, "Capacitors", 2, true;
    electrochemicalarrays, "Electrochemical Arrays", Manufactured, "Capacitors", 3, true;
    polymercapacitors, "Polymer Capacitors", Manufactured, "Capacitors", 4, true;
    militarysupercapacitors, "Military Supercapacitors", Manufactured, "Capacitors", 5, true;
    wornshieldemitters, "Worn Shield Emitters", Manufactured, "Shielding", 1, true;
    shieldemitters, "Shield Emitters", Manufactured, "Shielding", 2, true;
    shieldingsensors, "Shielding Sensors", Manufactured, "Shielding", 3, true;
    compoundshielding, "Compound Shielding", Manufactured, "Shielding", 4, true;
    imperialshielding, "Imperial Shielding", Manufactured, "Shielding", 5, true;
    compactcomposites, "Compact Composites", Manufactured, "Composite", 1, true;
    filamentcomposites, "Filament Composites", Manufactured, "Composite", 2, true;
    highdensitycomposites, "High Density Composites", Manufactured, "Composite", 3, true;
    fedproprietarycomposites, "Proprietary Composites", Manufactured, "Composite", 4, true;
    fedcorecomposites, "Core Dynamics Composites", Manufactured, "Composite", 5, true;
    crystalshards, "Crystal Shards", Manufactured, "Crystals", 1, true;
    uncutfocuscrystals, "Flawed Focus Crystals", Manufactured, "Crystals", 2, true;
    focuscrystals, "Focus Crystals", Manufactured, "Crystals", 3, true;
    refinedfocuscrystals, "Refined Focus Crystals", Manufactured, "Crystals", 4, true;
    exquisitefocuscrystals, "Exquisite Focus Crystals", Manufactured, "Crystals", 5, true;
    salvagedalloys, "Salvaged Alloys", Manufactured, "Alloys", 1, true;
    galvanisingalloys, "Galvanising Alloys", Manufactured, "Alloys", 2, true;
    phasealloys, "Phase Alloys", Manufactured, "Alloys", 3, true;
    protolightalloys, "Proto Light Alloys", Manufactured, "Alloys", 4, true;
    protoradiolicalloys, "Proto Radiolic Alloys", Manufactured, "Alloys", 5, true;

    scrambledemissiondata, "Exceptional Scrambled Emission Data", Encoded, "Emission Data", 1, true;
    archivedemissiondata, "Irregular Emission Data", Encoded, "Emission Data", 2, true;
    emissiondata, "Unexpected Emission Data", Encoded, "Emission Data", 3, true;
    decodedemissiondata, "Decoded Emission Data", Encoded, "Emission Data", 4, true;
    compactemissionsdata, "Abnormal Compact Emissions Data", Encoded, "Emission Data", 5, true;
    disruptedwakeechoes, "Atypical Disrupted Wake Echoes", Encoded, "Wake Scans", 1, true;
    fsdtelemetry, "Anomalous FSD Telemetry", Encoded, "Wake Scans", 2, true;
    wakesolutions, "Strange Wake Solutions", Encoded, "Wake Scans", 3, true;
    hyperspacetrajectories, "Eccentric Hyperspace Trajectories", Encoded, "Wake Scans", 4, true;
    dataminedwake, "Datamined Wake Exceptions", Encoded, "Wake Scans", 5, true;
    shieldcyclerecordings, "Distorted Shield Cycle Recordings", Encoded, "Shield Data", 1, true;
    shieldsoakanalysis, "Inconsistent Shield Soak Analysis", Encoded, "Shield Data", 2, true;
    shielddensityreports, "Untypical Shield Scans", Encoded, "Shield Data", 3, true;
    shieldpatternanalysis, "Aberrant Shield Pattern Analysis", Encoded, "Shield Data", 4, true;
    shieldfrequencydata, "Peculiar Shield Frequency Data", Encoded, "Shield Data", 5, true;
    encryptedfiles, "Unusual Encrypted Files", Encoded, "Encryption Files", 1, true;
    encryptioncodes, "Tagged Encryption Codes", Encoded, "Encryption Files", 2, true;
    symmetrickeys, "Open Symmetric Keys", Encoded, "Encryption Files", 3, true;
    encryptionarchives, "Atypical Encryption Archives", Encoded, "Encryption Files", 4, true;
    adaptiveencryptors, "Adaptive Encryptors Capture", Encoded, "Encryption Files", 5, true;
    bulkscandata, "Anomalous Bulk Scan Data", Encoded, "Data Archives", 1, true;
    scanarchives, "Unidentified Scan Archives", Encoded, "Data Archives", 2, true;
    scandatabanks, "Classified Scan Databanks", Encoded, "Data Archives", 3, true;
    encodedscandata, "Divergent Scan Data", Encoded, "Data Archives", 4, true;
    classifiedscandata, "Classified Scan Fragment", Encoded, "Data Archives", 5, true;
    legacyfirmware, "Specialised Legacy Firmware", Encoded, "Encoded Firmware", 1, true;
    consumerfirmware, "Modified Consumer Firmware", Encoded, "Encoded Firmware", 2, true;
    industrialfirmware, "Cracked Industrial Firmware", Encoded, "Encoded Firmware", 3, true;
    securityfirmware, "Security Firmware Patch", Encoded, "Encoded Firmware", 4, true;
    embeddedfirmware, "Modified Embedded Firmware", Encoded, "Encoded Firmware", 5, true;

    unknownenergysource, "Sensor Fragment", Manufactured, "Thargoid", 5, false;
    unknownshipsignature, "Thargoid Ship Signature", Encoded, "Thargoid", 3, false;
    unknownwakedata, "Thargoid Wake Data", Encoded, "Thargoid", 4, false;
    tgcompositiondata, "Thargoid Material Composition Data", Encoded, "Thargoid", 3, false;
    tgresiduedata, "Thargoid Residue Data", Encoded, "Thargoid", 4, false;
    tgstructuraldata, "Thargoid Structural Data", Encoded, "Thargoid", 2, false;
    unknowncarapace, "Thargoid Carapace", Manufactured, "Thargoid", 2, false;
    unknownenergycell, "Thargoid Energy Cell", Manufactured, "Thargoid", 3, false;
    unknownorganiccircuitry, "Thargoid Organic Circuitry", Manufactured, "Thargoid", 5, false;
    unknowntechnologycomponents, "Thargoid Technological Components", Manufactured, "Thargoid", 4, false;
    tgbiomechanicalconduits, "Bio-Mechanical Conduits", Manufactured, "Thargoid", 3, false;
    tgpropulsionelement, "Propulsion Elements", Manufactured, "Thargoid", 5, false;
    tgweaponparts, "Weapon Parts", Manufactured, "Thargoid", 4, false;
    tgwreckagecomponents, "Wreckage Components", Manufactured, "Thargoid", 3, false;
    tgshipflightdata, "Ship Flight Data", Encoded, "Thargoid", 3, false;
    tgshipsystemsdata, "Ship Systems Data", Encoded, "Thargoid", 4, false;
    guardianpowercell, "Guardian Power Cell", Manufactured, "Guardian", 1, false;
    guardianpowerconduit, "Guardian Power Conduit", Manufactured, "Guardian", 2, false;
    guardiantechcomponent, "Guardian Technology Component", Manufactured, "Guardian", 3, false;
    guardiansentinelweaponparts, "Guardian Sentinel Weapon Parts", Manufactured, "Guardian", 3, false;
    guardiansentinelwreckagecomponents, "Guardian Sentinel Wreckage Components", Manufactured, "Guardian", 1, false;
    guardianweaponblueprint, "Guardian Weapon Blueprint Segment", Encoded, "Guardian", 4, false;
    guardianmoduleblueprint, "Guardian Module Blueprint Segment", Encoded, "Guardian", 4, false;
    guardianvesselblueprint, "Guardian Vessel Blueprint Segment", Encoded, "Guardian", 5, false;
    ancientlanguagedata, "Pattern Delta Obelisk Data", Encoded, "Guardian", 4, false;
    ancientbiologicaldata, "Pattern Alpha Obelisk Data", Encoded, "Guardian", 4, false;
    ancientculturaldata, "Pattern Beta Obelisk Data", Encoded, "Guardian", 4, false;
    ancienthistoricaldata, "Pattern Gamma Obelisk Data", Encoded, "Guardian", 4, false;
    ancienttechnologicaldata, "Pattern Epsilon Obelisk Data", Encoded, "Guardian", 4, false;
};
