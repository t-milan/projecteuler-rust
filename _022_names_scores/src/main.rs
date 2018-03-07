use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("p022_names.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("unable to read file");

    let mut names = contents.split(',').collect::<Vec<&str>>();
    names.sort();

    let total: u32 = names
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * alpha_value(x.trim_matches('"')))
        .sum();

    println!("Total is {}", total);
}

fn alpha_value(name: &str) -> u32 {
    // Assumes uppercase alphanumeric characters.
    name.chars().map(|x| x as u32 - 64).sum()
}
