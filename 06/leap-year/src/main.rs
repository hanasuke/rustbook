use std::io;
use std::io::Write;

struct Year {
    year: u32
}

impl Year {
    pub fn new(y: u32) -> Year {
        Year{year: y}
    }
    fn is_leap_year(&self) -> bool {
        self.year%4 == 0 && !(self.year%100 == 0 && self.year%400 != 0)
    }
}

fn main() {
    let mut input = String::new();
    print!("please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let year = Year::new(input.trim().parse::<u32>().unwrap());

    if year.is_leap_year() {
        println!("{} is a leap year", year.year)
    } else {
        println!("{} is not a leap year", year.year)
    }
}
