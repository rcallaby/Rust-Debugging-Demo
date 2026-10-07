// nebula_cargo_tracker.rs
// DO NOT SUBMIT THIS VERSION. Fix every error.

use std::fs::File;
use std::io::Read;

struct ShippingManifest<'a> {
    ship: &'a String,
    package: &'a str,
}

fn get_package_mass(id: String) -> i32 {
    if id == "PKG-7781" {
        45
    } else if id == "PKG-9920" {
        12
    } else {
        -1                                          // sentinel value
    }
}

fn create_manifest(ship_name: &String, package_id: &str) -> &str {
    let text = format!("Manifest for {} carrying {}", ship_name, package_id);
    &text                                           // returns reference to local
}

fn add_package(hold: &String, package: String) {
    println!("Attempting to add {} into {}", package, hold);
}

fn total_mass(packages: Vec<String>) -> i32 {
    let mut sum = 0;
    let packages_clone = packages.clone();          // unnecessary clone
    for pkg in packages_clone {
        sum += get_package_mass(pkg);
    }
    sum
}

fn load_flight_plan() -> String {
    let mut file = File::open("flight_plan.txt").unwrap();  // unwrap
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();            // another unwrap
    contents
}

fn main() {
    let mut ship_name = String::from("Nebula Express");
    let mut cargo_hold = vec![
        String::from("PKG-7781"),
        String::from("PKG-9920"),
    ];

    // Use after move
    let name_backup = ship_name;
    println!("Ship: {}", ship_name);

    // Mutable borrow conflict
    let hold_ref1 = &mut cargo_hold;
    let hold_ref2 = &mut cargo_hold;
    hold_ref1.push(String::from("PKG-3344"));
    println!("{:?}", hold_ref2);

    // String vs &str + unnecessary clone + &String parameter
    let new_pkg = String::from("PKG-7781");
    add_package(&cargo_hold[0].clone(), new_pkg.clone());

    // Lifetime / dangling reference
    let manifest = create_manifest(&name_backup, "PKG-7781");
    println!("Manifest: {}", manifest);

    // Sentinel + unwrap misuse
    let mass = get_package_mass(String::from("PKG-0000")).unwrap();
    println!("Mass of unknown package: {}", mass);

    let total = total_mass(cargo_hold.clone());
    println!("Total cargo mass: {}", total);

    // Final status using moved values
    println!("{} final cargo hold: {:?}", name_backup, cargo_hold);
}