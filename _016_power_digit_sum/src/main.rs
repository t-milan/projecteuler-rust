extern crate num;

use num::BigUint;
use num::pow;

fn main() {
    let two = BigUint::new(vec![2]);
    let total = pow(two, 1000);

    let sum: u32 = total.to_str_radix(10)
                        .chars()
                        .map(|x| char::to_digit(x, 10).unwrap())
                        .sum();

    println!("{}", sum);
}
