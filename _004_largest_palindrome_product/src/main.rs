fn main() {
    let (mut max, mut it, mut jt) = (0, 0, 0);
    for i in 100..999 {
        for j in 100..i{
            if palindrome(i*j) {
                if i*j > max {
                    max = i*j;
                    it = i;
                    jt = j;
                }
            }
        }
    }
    println!("{} = {}*{}", max, it, jt);
    return;
}

fn palindrome(int: i32) -> bool {
    let s = int.to_string();
    if s.chars().rev().collect::<String>() == s {
        return true;
    }
    return false;
}
