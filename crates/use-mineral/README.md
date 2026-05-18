# use-mineral

Small mineral vocabulary primitives for RustUse.

## Example

```rust
use use_mineral::{CrystalSystem, MineralClass, MineralName, MohsHardness};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = MineralName::new("Quartz")?;
let hardness = MohsHardness::new(7.0)?;

assert_eq!(name.as_str(), "Quartz");
assert_eq!(MineralClass::Silicate.to_string(), "silicate");
assert_eq!(CrystalSystem::Trigonal.to_string(), "trigonal");
assert_eq!(hardness.value(), 7.0);
# Ok(())
# }
```
