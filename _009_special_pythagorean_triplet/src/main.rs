fn main() {
    let sum: u64 = 10000;

    for a in 1 .. (sum/3) {
        for b in (a+1) .. (sum/2 - a/2 + 1) {
            let c = sum - a - b;
            if a*a + b*b == c*c {
                println!("abc = {}", a*b*c);
                return;
            }
        }
    }
}
