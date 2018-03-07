use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("C:\\Users\\Tom\\Dev\\Rust\\ProjectEuler\\_067_maximum_path_sum_ii\\p067_triangle.txt")
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("unable to read file");

    let mut triangle: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        let elements = line.split_whitespace();
        for element in elements {
            row.push(element.parse().unwrap());
        }
        triangle.push(row);
    }

    let height = triangle.len();
    for level in 1..height {
        let width = level + 1;
        for element in 0..width {
            let max;
            if element == 0 {
                max = triangle[level - 1][element];
            } else if element == width - 1 {
                max = triangle[level - 1][element - 1];
            } else {
                max = std::cmp::max(triangle[level - 1][element - 1], triangle[level - 1][element]);
            }
            triangle[level][element] += max;
        }
    }
    println!("{}", triangle[height - 1].iter().max().unwrap());
}
