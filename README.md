# use-geology

> Composable sets of primitive Rust utility crates for fellow crustaceans.

`use-geology` is the RustUse sibling workspace for conservative geology vocabulary crates.
It provides small, explicit value types for minerals, rocks, strata, formations, faults,
tectonic plates, geologic time, fossils, geologic processes, and sediment descriptors.

The root `use-geology` crate is intentionally a thin facade. Most implementation lives in the
focused crates under `crates/`, and each focused crate is meant to stay independently useful.

## Workspace crates

- `use-geology`: thin facade crate that reexports the workspace by topic
- `use-mineral`: mineral names, classes, crystal systems, and Mohs hardness values
- `use-rock`: rock names, kinds, textures, and simple composition labels
- `use-stratum`: stratum names, kinds, ordering, and thickness values
- `use-formation`: formation names, groups, members, and broad formation kinds
- `use-fault`: fault names, kinds, movements, and activity states
- `use-tectonic-plate`: tectonic plate names, plate kinds, boundary kinds, and motion labels
- `use-geologic-time`: geologic time units, eons, eras, periods, epochs, and ages
- `use-fossil`: fossil names, fossil kinds, preservation descriptors, and occurrence labels
- `use-geologic-process`: process labels, process kinds, and simple process rates
- `use-sediment`: sediment names, kinds, grain sizes, sorting, and roundness

## Non-goals

`use-geology` is not trying to become:

- a geology simulator
- a GIS engine
- mining software
- a geophysics engine
- a seismic analysis toolkit
- a stratigraphic database
- a mineral database
- a plate tectonics simulator
- an Earth-science framework

The workspace stays on the vocabulary side of the boundary: small domain primitives, conservative
validation, and explicit labels that compose well with application code.

## Complementary crates

`use-geology` is designed to sit beside other RustUse crates rather than replace them.
Common companions in this workspace include:

- `use-math` for numeric helpers that sit above these domain values
- `use-time` for modern time handling that complements geologic chronology labels
- `use-text` for text normalization and tokenization outside the geology domain model
- `use-validate` for additional validation layers in application code
- `rustuse` when you want a single top-level RustUse facade across multiple sibling sets

## Example

```rust
use use_geology::{
	fault, formation, fossil, geologic_process, geologic_time, mineral, rock, sediment, stratum,
	tectonic_plate,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let quartz = mineral::MineralName::new("Quartz")?;
let hardness = mineral::MohsHardness::new(7.0)?;
let basalt = rock::RockName::new("Basalt")?;
let composition = rock::RockComposition::describe("mafic", ["Pyroxene", "Plagioclase"])?;
let layer = stratum::StratumName::new("Brushy Basin")?;
let thickness = stratum::StratumThickness::new(12.5)?;
let formation = formation::FormationName::new("Morrison Formation")?;
let member = formation::FormationMember::new("Brushy Basin Member")?;
let fault_name = fault::FaultName::new("Wasatch Fault")?;
let plate = tectonic_plate::TectonicPlateName::new("Pacific Plate")?;
let epoch = geologic_time::GeologicEpoch::new("Holocene")?;
let age = geologic_time::GeologicAge::new(145.0)?;
let fossil_name = fossil::FossilName::new("Trilobite trace")?;
let occurrence = fossil::FossilOccurrence::new(
	Some("Burgess Shale".to_string()),
	Some("Cambrian".to_string()),
)?;
let process = geologic_process::GeologicProcess::new("delta progradation")?;
let rate = geologic_process::ProcessRate::new(0.2, "mm/yr")?;
let sediment_name = sediment::SedimentName::new("Alluvial sand")?;
let grain_size = sediment::GrainSize::new(0.25)?;

assert_eq!(quartz.as_str(), "Quartz");
assert_eq!(mineral::MineralClass::Silicate.to_string(), "silicate");
assert_eq!(mineral::CrystalSystem::Trigonal.to_string(), "trigonal");
assert_eq!(hardness.value(), 7.0);

assert_eq!(basalt.as_str(), "Basalt");
assert_eq!(rock::RockKind::Igneous.to_string(), "igneous");
assert_eq!(rock::RockTexture::FineGrained.to_string(), "fine-grained");
assert_eq!(composition.to_string(), "mafic [Pyroxene, Plagioclase]");

assert_eq!(layer.as_str(), "Brushy Basin");
assert_eq!(stratum::StratumKind::Layer.to_string(), "layer");
assert_eq!(stratum::StratumOrder::new(3).position(), 3);
assert_eq!(thickness.meters(), 12.5);

assert_eq!(formation.as_str(), "Morrison Formation");
assert_eq!(formation::FormationKind::Formation.to_string(), "formation");
assert_eq!(member.as_str(), "Brushy Basin Member");
assert_eq!(formation::FormationGroup::new("Chinle Group")?.as_str(), "Chinle Group");

assert_eq!(fault_name.as_str(), "Wasatch Fault");
assert_eq!(fault::FaultKind::Normal.to_string(), "normal");
assert_eq!(fault::FaultMovement::new("dip-slip")?.as_str(), "dip-slip");
assert_eq!(fault::FaultActivity::Active.to_string(), "active");

assert_eq!(plate.as_str(), "Pacific Plate");
assert_eq!(tectonic_plate::PlateKind::Oceanic.to_string(), "oceanic");
assert_eq!(tectonic_plate::PlateBoundaryKind::Divergent.to_string(), "divergent");
assert_eq!(tectonic_plate::PlateMotion::new("northwest")?.as_str(), "northwest");

assert_eq!(geologic_time::GeologicEon::Phanerozoic.to_string(), "phanerozoic");
assert_eq!(geologic_time::GeologicEra::Mesozoic.to_string(), "mesozoic");
assert_eq!(geologic_time::GeologicPeriod::Jurassic.to_string(), "jurassic");
assert_eq!(epoch.as_str(), "Holocene");
assert_eq!(age.millions_of_years_before_present(), 145.0);

assert_eq!(fossil_name.as_str(), "Trilobite trace");
assert_eq!(fossil::FossilKind::TraceFossil.to_string(), "trace-fossil");
assert_eq!(fossil::FossilPreservation::Compressed.to_string(), "compressed");
assert_eq!(occurrence.to_string(), "Burgess Shale @ Cambrian");

assert_eq!(process.as_str(), "delta progradation");
assert_eq!(geologic_process::ProcessKind::Deposition.to_string(), "deposition");
assert_eq!(rate.to_string(), "0.2 mm/yr");

assert_eq!(sediment_name.as_str(), "Alluvial sand");
assert_eq!(sediment::SedimentKind::Sand.to_string(), "sand");
assert_eq!(grain_size.millimeters(), 0.25);
assert_eq!(sediment::Sorting::WellSorted.to_string(), "well-sorted");
assert_eq!(sediment::Roundness::SubRounded.to_string(), "sub-rounded");
# Ok(())
# }
```

## Status

This first version stays intentionally small. It focuses on composable vocabulary and validated
labels instead of deep scientific modeling or application framework behavior.
