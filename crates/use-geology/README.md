# use-geology

Thin facade crate for the RustUse geology workspace.

`use-geology` reexports the focused geology crates under topic modules so application code can use
one dependency while the underlying implementation stays split across the smaller crates.

## Reexports

- `use_fault` as `fault`
- `use_formation` as `formation`
- `use_fossil` as `fossil`
- `use_geologic_process` as `geologic_process`
- `use_geologic_time` as `geologic_time`
- `use_mineral` as `mineral`
- `use_rock` as `rock`
- `use_sediment` as `sediment`
- `use_stratum` as `stratum`
- `use_tectonic_plate` as `tectonic_plate`

## Example

```rust
use use_geology::{
	fault, formation, fossil, geologic_process, geologic_time, mineral, rock, sediment, stratum,
	tectonic_plate,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let quartz = mineral::MineralName::new("Quartz")?;
let basalt = rock::RockName::new("Basalt")?;
let composition = rock::RockComposition::describe("mafic", ["Pyroxene", "Plagioclase"])?;
let layer = stratum::StratumName::new("Brushy Basin")?;
let formation = formation::FormationName::new("Morrison Formation")?;
let fault_name = fault::FaultName::new("Wasatch Fault")?;
let plate = tectonic_plate::TectonicPlateName::new("Pacific Plate")?;
let period = geologic_time::GeologicPeriod::Jurassic;
let fossil_name = fossil::FossilName::new("Trilobite trace")?;
let process = geologic_process::GeologicProcess::new("delta progradation")?;
let sediment_name = sediment::SedimentName::new("Alluvial sand")?;

assert_eq!(quartz.as_str(), "Quartz");
assert_eq!(mineral::MineralClass::Silicate.to_string(), "silicate");
assert_eq!(mineral::MohsHardness::new(7.0)?.value(), 7.0);

assert_eq!(basalt.as_str(), "Basalt");
assert_eq!(rock::RockKind::Igneous.to_string(), "igneous");
assert_eq!(rock::RockTexture::FineGrained.to_string(), "fine-grained");
assert_eq!(composition.to_string(), "mafic [Pyroxene, Plagioclase]");

assert_eq!(layer.as_str(), "Brushy Basin");
assert_eq!(formation.as_str(), "Morrison Formation");
assert_eq!(fault_name.as_str(), "Wasatch Fault");
assert_eq!(fault::FaultKind::Normal.to_string(), "normal");
assert_eq!(tectonic_plate::PlateBoundaryKind::Divergent.to_string(), "divergent");
assert_eq!(plate.as_str(), "Pacific Plate");
assert_eq!(period.to_string(), "jurassic");
assert_eq!(fossil_name.as_str(), "Trilobite trace");
assert_eq!(fossil::FossilKind::TraceFossil.to_string(), "trace-fossil");
assert_eq!(process.as_str(), "delta progradation");
assert_eq!(geologic_process::ProcessKind::Deposition.to_string(), "deposition");
assert_eq!(sediment_name.as_str(), "Alluvial sand");
assert_eq!(sediment::SedimentKind::Sand.to_string(), "sand");
# Ok(())
# }
```
