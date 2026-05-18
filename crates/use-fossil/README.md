# use-fossil

Small fossil vocabulary primitives for RustUse.

## Example

```rust
use use_fossil::{FossilKind, FossilName, FossilOccurrence, FossilPreservation};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = FossilName::new("Trilobite trace")?;
let occurrence = FossilOccurrence::new(
	Some("Burgess Shale".to_string()),
	Some("Cambrian".to_string()),
)?;

assert_eq!(name.as_str(), "Trilobite trace");
assert_eq!(FossilKind::TraceFossil.to_string(), "trace-fossil");
assert_eq!(FossilPreservation::Compressed.to_string(), "compressed");
assert_eq!(occurrence.to_string(), "Burgess Shale @ Cambrian");
# Ok(())
# }
```
