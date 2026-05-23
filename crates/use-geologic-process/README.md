# use-geologic-process

Small geologic process vocabulary primitives for `RustUse`.

## Example

```rust
use use_geologic_process::{GeologicProcess, ProcessKind, ProcessRate};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let process = GeologicProcess::new("delta progradation")?;
let rate = ProcessRate::new(0.2, "mm/yr")?;

assert_eq!(process.as_str(), "delta progradation");
assert_eq!(ProcessKind::Deposition.to_string(), "deposition");
assert_eq!(rate.to_string(), "0.2 mm/yr");
# Ok(())
# }
```
