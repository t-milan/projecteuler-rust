const DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

struct MonthData {
    year: u16,
    month: u8,
    first_day: u8, //Sunday == 0
}

impl MonthData {
    fn next(&mut self) {
        if self.month < 12 {
            self.month += 1;
        } else {
            self.year += 1;
            self.month = 1;
        }
        self.first_day = (self.first_day + self.days()) % 7;
    }
    fn days(&self) -> u8 {
        if self.month == 2 && is_leap_year(self.year) {
            return 29;
        } else {
            return DAYS[(self.month - 1) as usize];
        }
    }
}

fn is_leap_year(year: u16) -> bool {
    if year % 400 == 0 { return true; }
    if year % 100 == 0 { return false; }
    if year % 4   == 0 { return true; }
    false
}

fn main() {
    let mut month = MonthData { year: 1900, month: 1, first_day: 1 };

    while month.year < 1901 { month.next(); }

    let mut sundays = 0;
    while month.year <= 2000 {
        if month.first_day == 0 { sundays += 1; }
        month.next();
    }

    println!("{} Sundays!", sundays);
}
