extern crate num_bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::cast::FromPrimitive;

fn main() {
    let mut total = BigUint::from_i32(1).unwrap();
    for i in 2..101 {
        total *= BigUint::from_usize(i).unwrap();
    }
    let digit_sum = total.to_radix_be(10)
                         .iter().map(|&d| d as u32)
                         .sum::<u32>();
    println!("{}", digit_sum);
}
