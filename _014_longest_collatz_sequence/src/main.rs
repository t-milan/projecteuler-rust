fn collatz_length(start: u64) -> u64 {
    if start == 1 {
        return 1;
    } else if start % 2 == 0 {
        return collatz_length(start/2) + 1;
    } else {
        return collatz_length(3*start + 1) + 1;
    }
}

fn main() {
    let mut largest: (u64, u64) = (0,0);
    for i in 1..1_000_000 {
        let length = collatz_length(i);
        if length > largest.1 {
            largest = (i, length);
        }
    }
    println!("{} has length {}", largest.0, largest.1);
}
