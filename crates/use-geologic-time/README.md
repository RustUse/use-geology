# use-geologic-time

Small geologic time vocabulary primitives for `RustUse`.

## Example

```rust
use use_geologic_time::{GeologicAge, GeologicEon, GeologicEpoch, GeologicEra, GeologicPeriod};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let epoch = GeologicEpoch::new("Holocene")?;
let age = GeologicAge::new(145.0)?;

assert_eq!(GeologicEon::Phanerozoic.to_string(), "phanerozoic");
assert_eq!(GeologicEra::Mesozoic.to_string(), "mesozoic");
assert_eq!(GeologicPeriod::Jurassic.to_string(), "jurassic");
assert_eq!(epoch.as_str(), "Holocene");
assert_eq!(age.millions_of_years_before_present(), 145.0);
# Ok(())
# }
```
