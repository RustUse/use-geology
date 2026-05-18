# Releasing

This repository uses a first-wave RustUse multi-crate release flow.

1. Publish the focused crates first.
2. Wait for the published versions to become visible on crates.io.
3. Run the facade dry-run once the focused crates are indexed.
4. Publish `use-geology` last.

Suggested focused-crate order:

1. `use-mineral`
2. `use-rock`
3. `use-stratum`
4. `use-formation`
5. `use-fault`
6. `use-tectonic-plate`
7. `use-geologic-time`
8. `use-fossil`
9. `use-geologic-process`
10. `use-sediment`
11. `use-geology`
