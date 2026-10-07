// solution_quest_log.rs
// Error-free version – all 10 common pitfalls fixed

use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct QuestReport {
    hero: String,   // owned data – no lifetime issues
    item: String,
}

impl QuestReport {
    fn new(hero: &str, item: &str) -> Self {
        QuestReport {
            hero: hero.to_string(),
            item: item.to_string(),
        }
    }

    fn display(&self) -> String {
        format!("{} bravely carries the {}", self.hero, self.item)
    }
}

/// Returns the weight of a known item, or None if the item is unknown.
/// (Replaces the old sentinel -1 approach)
fn get_item_weight(name: &str) -> Option<i32> {
    match name {
        "Sword of Debugging" => Some(12),
        "Potion of Compilation" => Some(3),
        "Shield of Ownership" => Some(8),
        _ => None,
    }
}

/// Adds an item to the inventory (takes &str for flexibility)
fn add_item(inventory: &mut Vec<String>, item: &str) {
    inventory.push(item.to_string());
    println!("Added \"{}\" to inventory", item);
}

/// Calculates total weight using references – no unnecessary clones
fn total_weight(items: &[String]) -> i32 {
    items
        .iter()
        .filter_map(|item| get_item_weight(item))
        .sum()
}

/// Attempts to load a secret quest file. Proper error handling – no unwrap.
fn load_secret_quest() -> Result<String, io::Error> {
    let mut file = File::open("secret_quest.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let hero_name = String::from("Aria the Compiler");

    let mut inventory = vec![
        String::from("Sword of Debugging"),
        String::from("Potion of Compilation"),
    ];

    // Safe mutable access – no overlapping borrows
    {
        add_item(&mut inventory, "Shield of Ownership");
    }

    // Create a report using owned data (no dangling references)
    let report = QuestReport::new(&hero_name, "Sword of Debugging");
    println!("Report: {}", report.display());

    // Look up weight safely with Option
    match get_item_weight("Unknown Artifact") {
        Some(weight) => println!("Weight of unknown: {}", weight),
        None => println!("Unknown Artifact not found in the catalog"),
    }

    // Calculate total weight from a slice – no clone needed
    let total = total_weight(&inventory);
    println!("Total inventory weight: {}", total);

    // Optional: try to load a secret quest (handles missing file gracefully)
    match load_secret_quest() {
        Ok(quest) => println!("Secret quest loaded: {}", quest),
        Err(e) => println!("Could not load secret quest (this is expected if the file is missing): {}", e),
    }

    // Final status – everything is still valid
    println!("{}'s final inventory: {:?}", hero_name, inventory);
}