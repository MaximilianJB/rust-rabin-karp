use std::io;

fn main() {
    let mut passage = String::new();
    let mut target = String::new();

    println!("\nEnter a phrase to search:");
    io::stdin()
        .read_line(&mut passage)
        .expect("Failed to read line");

    println!("\nEnter a target phrase:");
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    let passage = passage.trim();
    let target_phrase = target.trim();

    println!("\nSearching for '{target_phrase}' in '{passage}'...");

}
