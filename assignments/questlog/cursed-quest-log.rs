// cursed_quest_log.rs
// DO NOT SUBMIT THIS VERSION. Fix every error.

use std::fs::File;
use std::io::Read;

struct QuestReport<'a> {
    hero: &'a String,          // lifetime & type issues lurking
    item: &'a str,
}

fn get_item_weight(name: String) -> i32 {          // sentinel + ownership
    if name == "Sword of Debugging" {
        12
    } else if name == "Potion of Compilation" {
        3
    } else {
        -1                                          // sentinel value instead of Option
    }
}

fn create_report(hero_name: &String, item_name: &str) -> &str {  // returns local reference
    let report = format!("{} bravely carries the {}", hero_name, item_name);
    &report                                         // dangling reference
}

fn add_item(inventory: &String, item: String) {    // unnecessary &String + ownership
    // pretends to add but does nothing useful
    println!("Trying to add {} to {}", item, inventory);
}

fn total_weight(items: Vec<String>) -> i32 {
    let mut total = 0;
    let items_clone = items.clone();                // overusing clone
    for item in items_clone {
        total += get_item_weight(item);
    }
    total
}

fn load_secret_quest() -> String {
    let mut file = File::open("secret_quest.txt").unwrap();  // unwrap on Result
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();             // another unwrap
    contents
}

fn main() {
    let mut hero_name = String::from("Aria the Compiler");
    let mut inventory = vec![
        String::from("Sword of Debugging"),
        String::from("Potion of Compilation"),
    ];

    // Move then use
    let name_copy = hero_name;
    println!("Hero: {}", hero_name);                // use after move

    // Mutable borrow conflict
    let inv_ref1 = &mut inventory;
    let inv_ref2 = &mut inventory;                  // second mutable borrow
    inv_ref1.push(String::from("Shield of Ownership"));
    println!("{:?}", inv_ref2);

    // String vs &str confusion + unnecessary clone
    let quest_item = String::from("Sword of Debugging");
    add_item(&inventory[0].clone(), quest_item.clone());  // clone abuse + &String

    // Lifetime / report creation
    let report = create_report(&name_copy, "Sword of Debugging");
    println!("Report: {}", report);

    // More unwrap and sentinel usage
    let weight = get_item_weight(String::from("Unknown Artifact")).unwrap(); // wrong type + would panic
    println!("Weight of unknown: {}", weight);

    let total = total_weight(inventory.clone());
    println!("Total inventory weight: {}", total);

    // Final status that tries to use moved values
    println!("{}'s final inventory: {:?}", name_copy, inventory);
}