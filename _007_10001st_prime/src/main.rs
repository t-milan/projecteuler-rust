//extern crate time;
//use time::{Duration, PreciseTime};

fn main() {
    let start = PreciseTime::now();

    let nth = 10001;
    let mut primes: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    let mut isprime: bool;

    for i in 2.. { 
        isprime = true;
        for &p in &primes {
            if p > f32::sqrt(i as f32) as u32{
                break;
            }
            if i % p == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            primes.push(i);
            count = count + 1;
            if count == nth {
                println!("{}", i);
                break;
            }
        }
    }

    //println!("Took {} ms.", start.to(PreciseTime::now()).num_milliseconds());
}
