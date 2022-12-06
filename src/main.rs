use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let now = std::time::Instant::now();
    let mut elf_calories = Vec::new();
    let mut elf_total = 0;

    // Open the file in read-only mode
    let file = File::open("src/day1/input.in").unwrap();

    // Create a BufReader instance to read the file line by line
    let reader = BufReader::new(file);

    // Read the file, one line at a time
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // An empty line signals the end of an Elf's inventory
            elf_calories.push(elf_total);
            elf_total = 0;
        } else {
            // Add the calories of the current food item to the Elf's total
            elf_total += line.trim().parse::<i32>().unwrap();
        }
    }

    // Find the Elf with the most calories and print the result
    let max_calories = elf_calories.iter().max().unwrap();
    println!("Elf with most calories: {}", max_calories);
    
    println!("time: {:?}", now.elapsed());
}