# Mole

A stupid 3D model format that is literally a zstandard messagepack struct. Useful for ingest directly from a game or engine without 5000 dependencies and a 6 hour build. GLTF is too hard :(

## Usage

```rust
use mole::from_bytes;
let file = std::fs::read("hello.mol").unwrap();
let hill = mole::from_bytes(file);
println!("{:?}", hill);
std::fs::write("hello.mol", mole::to_bytes(&hill));
```
