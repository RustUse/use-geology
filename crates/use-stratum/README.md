# use-stratum

Small stratum and layer vocabulary primitives for `RustUse`.

## Example

```rust
use use_stratum::{StratumKind, StratumName, StratumOrder, StratumThickness};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = StratumName::new("Brushy Basin")?;
let thickness = StratumThickness::new(12.5)?;

assert_eq!(name.as_str(), "Brushy Basin");
assert_eq!(StratumKind::Layer.to_string(), "layer");
assert_eq!(StratumOrder::new(3).position(), 3);
assert_eq!(thickness.meters(), 12.5);
# Ok(())
# }
```
