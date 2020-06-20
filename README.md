# Genome

Built using rust, used to create cryptobsj

## Example

Creating a DNA sequence

```rust
use genome::DNA;
let dna = DNA::new(2, 2);
```

Converting DNA to string

```rust
use genome::DNA;
let dna = DNA::new(2, 2);
let dna_string = dna.to_string();
```

## Installation

To use this package, add it in the `[dependencies]` in your `Cargo.toml`

```toml
[dependencies]
genome = "0.1.0"
```
