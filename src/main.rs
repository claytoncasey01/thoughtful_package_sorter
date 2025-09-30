use std::fmt;

/// Represents a dimension in centimeters (newtype pattern for type safety)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Centimeters(f64);

impl Centimeters {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Represents mass in kilograms (newtype pattern for type safety)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilograms(f64);

impl Kilograms {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Package sorting category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCategory {
    /// Standard packages (not bulky or heavy) - can be handled normally
    Standard,
    /// Special packages (either bulky or heavy) - require special handling
    Special,
    /// Rejected packages (both bulky and heavy) - cannot be processed
    Rejected,
}

impl fmt::Display for SortCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl SortCategory {
    /// Returns the string representation of the category
    pub fn as_str(&self) -> &'static str {
        match self {
            SortCategory::Standard => "STANDARD",
            SortCategory::Special => "SPECIAL",
            SortCategory::Rejected => "REJECTED",
        }
    }
}

/// Represents a package with dimensions and mass
#[derive(Debug, Clone, Copy)]
pub struct Package {
    pub width: Centimeters,
    pub height: Centimeters,
    pub length: Centimeters,
    pub mass: Kilograms,
}

impl Package {
    /// Creates a new package with the given dimensions and mass
    pub fn new(width: Centimeters, height: Centimeters, length: Centimeters, mass: Kilograms) -> Self {
        Self { width, height, length, mass }
    }

    /// Calculates the volume of the package in cubic centimeters
    pub fn volume(&self) -> f64 {
        self.width.value() * self.height.value() * self.length.value()
    }

    /// Checks if the package is bulky according to the rules:
    /// - Volume >= 1,000,000 cm³, OR
    /// - Any dimension >= 150 cm
    pub fn is_bulky(&self) -> bool {
        const VOLUME_THRESHOLD: f64 = 1_000_000.0;
        const DIMENSION_THRESHOLD: f64 = 150.0;

        self.volume() >= VOLUME_THRESHOLD
            || self.width.value() >= DIMENSION_THRESHOLD
            || self.height.value() >= DIMENSION_THRESHOLD
            || self.length.value() >= DIMENSION_THRESHOLD
    }

    /// Checks if the package is heavy (mass >= 20 kg)
    pub fn is_heavy(&self) -> bool {
        const MASS_THRESHOLD: f64 = 20.0;
        self.mass.value() >= MASS_THRESHOLD
    }

    /// Determines the sort category for this package
    pub fn sort_category(&self) -> SortCategory {
        match (self.is_bulky(), self.is_heavy()) {
            (true, true) => SortCategory::Rejected,
            (true, false) | (false, true) => SortCategory::Special,
            (false, false) => SortCategory::Standard,
        }
    }
}

/// Sorts packages based on their dimensions and mass.
///
/// # Arguments
/// * `width` - Width in centimeters
/// * `height` - Height in centimeters
/// * `length` - Length in centimeters
/// * `mass` - Mass in kilograms
///
/// # Returns
/// * A string representing the stack name: "STANDARD", "SPECIAL", or "REJECTED"
///
/// # Rules
/// * A package is **bulky** if:
///   - Its volume (width × height × length) >= 1,000,000 cm³, OR
///   - Any dimension >= 150 cm
/// * A package is **heavy** if:
///   - Its mass >= 20 kg
///
/// # Example
/// ```
/// let result = sort(100.0, 100.0, 100.0, 25.0);
/// assert_eq!(result, "REJECTED");
/// ```
pub fn sort(width: f64, height: f64, length: f64, mass: f64) -> &'static str {
    let package = Package::new(
        Centimeters::new(width),
        Centimeters::new(height),
        Centimeters::new(length),
        Kilograms::new(mass),
    );
    package.sort_category().as_str()
}

fn main() {
    println!("Package Sorting System\n");

    let test_cases = vec![
        (50.0, 50.0, 50.0, 10.0, "Standard package"),
        (100.0, 100.0, 100.0, 10.0, "Bulky by volume"),
        (160.0, 50.0, 50.0, 10.0, "Bulky by dimension"),
        (50.0, 50.0, 50.0, 25.0, "Heavy package"),
        (160.0, 50.0, 50.0, 25.0, "Bulky and heavy"),
    ];

    for (width, height, length, mass, description) in test_cases {
        let category = sort(width, height, length, mass);
        println!(
            "{}: {}x{}x{} cm, {} kg -> {}",
            description, width, height, length, mass, category
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_package() {
        // Small dimensions, light weight
        assert_eq!(sort(10.0, 10.0, 10.0, 5.0), "STANDARD");
        assert_eq!(sort(50.0, 50.0, 50.0, 10.0), "STANDARD");
        // Just under all thresholds
        assert_eq!(sort(149.0, 149.0, 1.0, 19.9), "STANDARD");
    }

    #[test]
    fn test_bulky_by_volume() {
        // Volume >= 1,000,000 cm³
        assert_eq!(sort(100.0, 100.0, 100.0, 10.0), "SPECIAL");
        assert_eq!(sort(200.0, 100.0, 50.0, 15.0), "SPECIAL");
        // Exactly at threshold
        assert_eq!(sort(100.0, 100.0, 100.0, 5.0), "SPECIAL");
    }

    #[test]
    fn test_bulky_by_dimension() {
        // One dimension >= 150 cm
        assert_eq!(sort(150.0, 10.0, 10.0, 10.0), "SPECIAL");
        assert_eq!(sort(10.0, 150.0, 10.0, 10.0), "SPECIAL");
        assert_eq!(sort(10.0, 10.0, 150.0, 10.0), "SPECIAL");
        assert_eq!(sort(200.0, 50.0, 50.0, 15.0), "SPECIAL");
    }

    #[test]
    fn test_heavy_package() {
        // Mass >= 20 kg, not bulky
        assert_eq!(sort(10.0, 10.0, 10.0, 20.0), "SPECIAL");
        assert_eq!(sort(50.0, 50.0, 50.0, 25.0), "SPECIAL");
        assert_eq!(sort(50.0, 50.0, 50.0, 100.0), "SPECIAL");
    }

    #[test]
    fn test_rejected_package() {
        // Both bulky (by volume) and heavy
        assert_eq!(sort(100.0, 100.0, 100.0, 20.0), "REJECTED");
        assert_eq!(sort(200.0, 100.0, 50.0, 25.0), "REJECTED");

        // Both bulky (by dimension) and heavy
        assert_eq!(sort(150.0, 50.0, 50.0, 20.0), "REJECTED");
        assert_eq!(sort(200.0, 10.0, 10.0, 30.0), "REJECTED");
    }

    #[test]
    fn test_package_struct() {
        // Test using the Package struct directly
        let pkg = Package::new(
            Centimeters::new(100.0),
            Centimeters::new(100.0),
            Centimeters::new(100.0),
            Kilograms::new(25.0),
        );

        assert_eq!(pkg.volume(), 1_000_000.0);
        assert!(pkg.is_bulky());
        assert!(pkg.is_heavy());
        assert_eq!(pkg.sort_category(), SortCategory::Rejected);
    }

    #[test]
    fn test_newtype_safety() {
        // Demonstrate type safety with newtypes
        let width = Centimeters::new(150.0);
        let mass = Kilograms::new(25.0);

        assert_eq!(width.value(), 150.0);
        assert_eq!(mass.value(), 25.0);

        // The types prevent mixing up dimensions and mass at compile time
    }
}
