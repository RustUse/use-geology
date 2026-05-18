# use-sediment

Small sediment vocabulary primitives for RustUse.

## Example

```rust
use use_sediment::{GrainSize, Roundness, SedimentKind, SedimentName, Sorting};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = SedimentName::new("Alluvial sand")?;
let grain_size = GrainSize::new(0.25)?;

assert_eq!(name.as_str(), "Alluvial sand");
assert_eq!(SedimentKind::Sand.to_string(), "sand");
assert_eq!(grain_size.millimeters(), 0.25);
assert_eq!(Sorting::WellSorted.to_string(), "well-sorted");
assert_eq!(Roundness::SubRounded.to_string(), "sub-rounded");
# Ok(())
# }
```
