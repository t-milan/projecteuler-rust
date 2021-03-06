fn main() {
    let mut primes: Vec<u64> = Vec::new();
    let mut isprime: bool;

    for i in 2..2_000_000 { 
        isprime = true;
        for &p in &primes {
            let sqrt_i = f32::sqrt(i as f32) as u64;
            if p > sqrt_i{
                break;
            }
            if i % p == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            primes.push(i);
        }
    }

    println!("{}", primes.iter().sum::<u64>());
}
