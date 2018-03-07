fn main() {
    let mut range: Vec<u32> = (1..21).collect();

    for i in 1..21 {
        for j in (i+1)..21 {
            if j % i == 0 {
                range[j-1] = range[j-1]/range[i-1];
            }
        }
    }

    println!("{:?}", range.iter().product());
}
