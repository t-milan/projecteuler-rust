extern crate primal;
extern crate time;

use time::PreciseTime;

fn proper_divisors(factors: Vec<(usize, usize)>) -> Vec<u32> {
    let mut divisors = vec!(1 as u32);

    for pair in factors {
        let prime = pair.0 as u32;
        let mut temp = Vec::new();
        for exponent in 1..(pair.1 + 1) {
            temp.append(&mut divisors.clone()
                                     .iter()
                                     .map(|&x| x * prime.pow(exponent as u32))
                                     .collect());
        }
        divisors.append(&mut temp);
    }
    divisors.pop();
    divisors
}

fn main() {
    let start = PreciseTime::now();

    let sieve = primal::Sieve::new(2000);
    let mut amicables = Vec::new();
    for i in 2..1000000 {
        let factors = sieve.factor(i).expect("Cannot factor.");
        let divisors_sum: u32 = proper_divisors(factors).iter().sum();
        if divisors_sum == i as u32 { continue; }

        let factors_new = sieve.factor(divisors_sum as usize).expect("Cannot factor.");
        let new_sum: u32 = proper_divisors(factors_new).iter().sum();

        if new_sum == i as u32 {
            amicables.push(i);
        }
    }
    println!("Sum = {}", amicables.iter().sum::<usize>());
    println!("Took {} ms.", start.to(PreciseTime::now()).num_milliseconds());
}
