# use-rock

Small rock vocabulary primitives for `RustUse`.

## Example

```rust
use use_rock::{RockComposition, RockKind, RockName, RockTexture};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let name = RockName::new("Basalt")?;
let composition = RockComposition::describe("mafic", ["Pyroxene", "Plagioclase"])?;

assert_eq!(name.as_str(), "Basalt");
assert_eq!(RockKind::Igneous.to_string(), "igneous");
assert_eq!(RockTexture::FineGrained.to_string(), "fine-grained");
assert_eq!(composition.to_string(), "mafic [Pyroxene, Plagioclase]");
# Ok(())
# }
```
