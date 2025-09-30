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
