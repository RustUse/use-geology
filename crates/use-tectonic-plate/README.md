# use-tectonic-plate

Small tectonic plate vocabulary primitives for RustUse.

## Example

```rust
use use_tectonic_plate::{PlateBoundaryKind, PlateKind, PlateMotion, TectonicPlateName};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = TectonicPlateName::new("Pacific Plate")?;
let motion = PlateMotion::new("northwest")?;

assert_eq!(name.as_str(), "Pacific Plate");
assert_eq!(PlateKind::Oceanic.to_string(), "oceanic");
assert_eq!(PlateBoundaryKind::Divergent.to_string(), "divergent");
assert_eq!(motion.as_str(), "northwest");
# Ok(())
# }
```
