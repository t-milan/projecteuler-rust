extern crate time;
use time::{Duration, PreciseTime};

struct Primes {
    current: u64,
    so_far: Vec<u64>,
}

impl Primes {
    fn new() -> Primes {
        Primes { current: 1, so_far: vec!(2) }
    }
    fn copy_vec(&self) -> &[u64] {
        &self.so_far.as_slice()
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut isprime: bool;
        let mut result: u64 = 0;

        for i in (self.current+1).. { 
            isprime = true;
            for &p in &self.so_far {
                if p > f32::sqrt(i as f32) as u64{
                    break;
                }
                if i % p == 0 {
                    isprime = false;
                    break;
                }
            }
            if isprime {
                self.so_far.push(i);
                self.current = i;
                result = i;
                break;
            }
        }
        Some(result)
    }
}

fn is_prime(test: u64, so_far: &mut Primes) -> bool {
    if test <= so_far.current {
        for &p in so_far.copy_vec() {
            if p > test { return false; }
            if p == test { return true; }
        }
    } else {
        for p in so_far {
            if p > test { return false; }
            if p == test { return true; }
        }
    }
    false
}

fn main() {
    let start = PreciseTime::now();
    let mut so_far = Primes::new();
    for p in Primes::new() {
        let p_string = p.to_string();
        for to_replace in "012".chars() {
            if !p_string.contains(to_replace) {
                continue;
            }
            let mut total = 1;
            for (index, replace_with) in "0123456789".chars().enumerate() {
                if to_replace == replace_with { continue; }
                if index as i8 - total > 2 { break; }
                let new_string = p_string.replace(&to_replace.to_string(), &replace_with.to_string());
                let test: u64 = new_string.parse()
                                          .expect("Could not parse string");
                if is_prime(test, &mut so_far) && !new_string.starts_with('0') {
                    total += 1;
                }
            }
            if total == 8 {
                println!("{}", p);
                println!("Took {} ms.", start.to(PreciseTime::now()).num_milliseconds());
                return;
            }
        }
    }
}
