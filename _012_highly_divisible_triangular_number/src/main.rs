fn num_divisors(input:u32) -> u32 {
    let limit = f32::sqrt(input as f32) as u32;
    let mut total = 2;
    for i in 2..(limit+1) {
        if input%i == 0 {
            total += 2;
        }
    }
    if limit*limit == input { total -= 1; }
    total
}

fn main() {
    let mut triangle_number = 0;

    for i in 1.. {
        triangle_number += i;
        let divisors = num_divisors(triangle_number);
        if divisors > 500 {
            println!("{} has {} divisors!", triangle_number, divisors);
            break;
        }
    }
}
