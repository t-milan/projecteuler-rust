fn main() {

    let total: u32 = (1..101).sum();

    let answer: u32 = (1..101).map(|x| x*(total-x)).sum();

    println!("Answer = {}", answer);
}
