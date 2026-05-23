# use-fault

Small geologic fault vocabulary primitives for `RustUse`.

## Example

```rust
use use_fault::{FaultActivity, FaultKind, FaultMovement, FaultName};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = FaultName::new("Wasatch Fault")?;
let movement = FaultMovement::new("dip-slip")?;

assert_eq!(name.as_str(), "Wasatch Fault");
assert_eq!(FaultKind::Normal.to_string(), "normal");
assert_eq!(movement.as_str(), "dip-slip");
assert_eq!(FaultActivity::Active.to_string(), "active");
# Ok(())
# }
```
