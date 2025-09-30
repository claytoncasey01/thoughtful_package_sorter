/// Sorts packages based on their dimensions and mass.
///
/// # Arguments
/// * `width` - Width in centimeters
/// * `height` - Height in centimeters
/// * `length` - Length in centimeters
/// * `mass` - Mass in kilograms
///
/// # Returns
/// * `"STANDARD"` - Package is neither bulky nor heavy
/// * `"SPECIAL"` - Package is either bulky or heavy (but not both)
/// * `"REJECTED"` - Package is both bulky and heavy
///
/// # Rules
/// * A package is **bulky** if:
///   - Its volume (width × height × length) >= 1,000,000 cm³, OR
///   - Any dimension >= 150 cm
/// * A package is **heavy** if:
///   - Its mass >= 20 kg
pub fn sort(width: f64, height: f64, length: f64, mass: f64) -> &'static str {
    let volume = width * height * length;
    let is_bulky = volume >= 1_000_000.0 || width >= 150.0 || height >= 150.0 || length >= 150.0;
    let is_heavy = mass >= 20.0;

    match (is_bulky, is_heavy) {
        (true, true) => "REJECTED",
        (true, false) | (false, true) => "SPECIAL",
        (false, false) => "STANDARD",
    }
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
        let result = sort(width, height, length, mass);
        println!(
            "{}: {}x{}x{} cm, {} kg -> {}",
            description, width, height, length, mass, result
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
}
