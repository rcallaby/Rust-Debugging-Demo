// solution_nebula_cargo_tracker.rs
// Error-free version – all 10 common pitfalls fixed

use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct ShippingManifest {
    ship: String,     // owned data – no lifetime parameters needed
    package: String,
}

impl ShippingManifest {
    fn new(ship: &str, package: &str) -> Self {
        ShippingManifest {
            ship: ship.to_string(),
            package: package.to_string(),
        }
    }

    fn display(&self) -> String {
        format!("Manifest for {} carrying {}", self.ship, self.package)
    }
}

/// Returns the mass of a known package, or None if the package ID is unknown.
/// (Replaces the old sentinel value approach)
fn get_package_mass(id: &str) -> Option<i32> {
    match id {
        "PKG-7781" => Some(45),
        "PKG-9920" => Some(12),
        "PKG-3344" => Some(28),
        _ => None,
    }
}

/// Adds a package to the cargo hold (takes &str for maximum flexibility)
fn add_package(hold: &mut Vec<String>, package: &str) {
    hold.push(package.to_string());
    println!("Added package \"{}\" to the cargo hold", package);
}

/// Calculates total mass using a slice – no unnecessary cloning
fn total_mass(packages: &[String]) -> i32 {
    packages
        .iter()
        .filter_map(|pkg| get_package_mass(pkg))
        .sum()
}

/// Attempts to load a flight plan. Proper error handling – no unwrap.
fn load_flight_plan() -> Result<String, io::Error> {
    let mut file = File::open("flight_plan.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let ship_name = String::from("Nebula Express");

    let mut cargo_hold = vec![
        String::from("PKG-7781"),
        String::from("PKG-9920"),
    ];

    // Safe mutable access – no overlapping borrows
    {
        add_package(&mut cargo_hold, "PKG-3344");
    }

    // Create a manifest using owned data (no dangling references)
    let manifest = ShippingManifest::new(&ship_name, "PKG-7781");
    println!("Manifest: {}", manifest.display());

    // Look up mass safely with Option
    match get_package_mass("PKG-0000") {
        Some(mass) => println!("Mass of unknown package: {}", mass),
        None => println!("Package PKG-0000 not found in the manifest database"),
    }

    // Calculate total mass from a slice – no clone needed
    let total = total_mass(&cargo_hold);
    println!("Total cargo mass: {} kg", total);

    // Optional: try to load a flight plan (handles missing file gracefully)
    match load_flight_plan() {
        Ok(plan) => println!("Flight plan loaded: {}", plan),
        Err(e) => println!(
            "Could not load flight plan (expected if the file is missing): {}",
            e
        ),
    }

    // Final status – everything is still valid and usable
    println!("{} final cargo hold: {:?}", ship_name, cargo_hold);
}