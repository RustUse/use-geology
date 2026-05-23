# use-formation

Small geologic formation vocabulary primitives for `RustUse`.

## Example

```rust
use use_formation::{FormationGroup, FormationKind, FormationMember, FormationName};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = FormationName::new("Morrison Formation")?;
let member = FormationMember::new("Brushy Basin Member")?;
let group = FormationGroup::new("Chinle Group")?;

assert_eq!(name.as_str(), "Morrison Formation");
assert_eq!(FormationKind::Formation.to_string(), "formation");
assert_eq!(member.as_str(), "Brushy Basin Member");
assert_eq!(group.as_str(), "Chinle Group");
# Ok(())
# }
```
