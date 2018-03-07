fn main() {
    let mut characters = 0;
    for i in 1..1001 {
        characters += written(i).chars()
                                .filter(|&x| char::is_alphabetic(x))
                                .count();
    }
    println!("{}", characters);
}

fn hundreds(number: usize) -> String {
    let mut written = String::new();
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight",
                "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", 
                "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "ten", "twenty", "thirty", "forty", "fifty", 
                "sixty", "seventy", "eighty", "ninety"];

    if number %100 < 20 {
        written.insert_str(0, ones[number % 100]);
    } else {
        written.insert_str(0, &format!("{} {}", tens[number/10 % 10], ones[number % 10]));
    }

    if number >= 100 {
        if number % 100 != 0 {
            written.insert_str(0, "and ");
        }
        if number/100 % 10 != 0 {
            written.insert_str(0, &format!("{} hundred ", ones[number/100 % 10]));
        }
    }
    written
}

fn written(number: usize) -> String {
    assert!(number > 0 && number < 1_000_000_000_000_000, "Invalid number: {}", number);

    let orders = ["thousand", "million", "billion", "trillion"];
    let mut written = String::new();
    written.insert_str(0, &hundreds(number));

    for i in 1..4 {
        let order = 10u32.pow(i*3) as usize;
        if number >= order && number/order % 1000 != 0 {
            written.insert_str(0, &format!("{} {} ", hundreds(number/order % 1000), orders[(i as usize)-1]));
        }
    }
    written
}
