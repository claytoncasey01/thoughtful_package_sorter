[![Rust](https://github.com/claytoncasey01/thoughtful_package_sorter/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/claytoncasey01/thoughtful_package_sorter/actions/workflows/rust.yml)

# Package Sorter

A Rust implementation of an automated package sorting system for Thoughtful's robotic automation factory.

## Overview

This program sorts packages into three categories based on their dimensions and mass:

- **STANDARD**: Packages that can be handled normally
- **SPECIAL**: Packages requiring special handling (either bulky or heavy)
- **REJECTED**: Packages that cannot be processed (both bulky and heavy)

## Sorting Rules

### Bulky Package

A package is considered **bulky** if:

- Its volume (width × height × length) ≥ 1,000,000 cm³, **OR**
- Any single dimension ≥ 150 cm

### Heavy Package

A package is considered **heavy** if:

- Its mass ≥ 20 kg

### Dispatch Logic

- **STANDARD**: Not bulky AND not heavy
- **SPECIAL**: Bulky XOR heavy (one but not both)
- **REJECTED**: Bulky AND heavy (both conditions met)

## Installation

Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

### Run the Example Program

```bash
cargo run
```

This will execute example test cases and display the sorting results.

### Run Tests

```bash
cargo test
```

To see detailed test output:

```bash
cargo test -- --nocapture
```

### Using the `sort` Function

**Simple API (for quick usage):**

```rust
use sorter::sort;

fn main() {
    let category = sort(100.0, 100.0, 100.0, 25.0);
    println!("Package should go to: {}", category); // Output: REJECTED
}
```

**Type-Safe API (recommended):**

```rust
use sorter::{Package, Centimeters, Kilograms, SortCategory};

fn main() {
    let package = Package::new(
        Centimeters::new(100.0),
        Centimeters::new(100.0),
        Centimeters::new(100.0),
        Kilograms::new(25.0),
    );

    println!("Volume: {} cm³", package.volume());
    println!("Is bulky: {}", package.is_bulky());
    println!("Is heavy: {}", package.is_heavy());
    println!("Category: {}", package.sort_category()); // Output: REJECTED
}
```

## API Reference

### Simple Function

```rust
pub fn sort(width: f64, height: f64, length: f64, mass: f64) -> &'static str
```

**Parameters:**

- `width`: Width in centimeters (cm)
- `height`: Height in centimeters (cm)
- `length`: Length in centimeters (cm)
- `mass`: Mass in kilograms (kg)

**Returns:**

- A string: `"STANDARD"`, `"SPECIAL"`, or `"REJECTED"`

### Type-Safe Structures

**`SortCategory` enum:**

```rust
pub enum SortCategory {
    Standard,  // Not bulky or heavy
    Special,   // Either bulky or heavy
    Rejected,  // Both bulky and heavy
}
```

**`Package` struct:**

```rust
pub struct Package {
    pub width: Centimeters,
    pub height: Centimeters,
    pub length: Centimeters,
    pub mass: Kilograms,
}

impl Package {
    pub fn new(width: Centimeters, height: Centimeters, length: Centimeters, mass: Kilograms) -> Self;
    pub fn volume(&self) -> f64;
    pub fn is_bulky(&self) -> bool;
    pub fn is_heavy(&self) -> bool;
    pub fn sort_category(&self) -> SortCategory;
}
```

**Newtype wrappers for type safety:**

```rust
pub struct Centimeters(f64);  // Prevents mixing up dimensions
pub struct Kilograms(f64);    // Prevents mixing up mass with dimensions
```

## Examples

| Width | Height | Length | Mass | Result   | Reason                  |
| ----- | ------ | ------ | ---- | -------- | ----------------------- |
| 50    | 50     | 50     | 10   | STANDARD | Not bulky, not heavy    |
| 100   | 100    | 100    | 10   | SPECIAL  | Bulky (volume = 1M cm³) |
| 160   | 50     | 50     | 10   | SPECIAL  | Bulky (dimension ≥ 150) |
| 50    | 50     | 50     | 25   | SPECIAL  | Heavy (mass ≥ 20 kg)    |
| 160   | 50     | 50     | 25   | REJECTED | Both bulky and heavy    |

## Project Structure

```
.
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    └── main.rs         # Implementation and tests
```

## License

This is a coding challenge submission for Thoughtful AI.
