// Complete the code by bringing the required items into scope.

mod days {
    pub enum WeekDay {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    pub fn is_holiday(day: &WeekDay) -> bool {
        match day {
            WeekDay::Sunday | WeekDay::Saturday => true,
            _ => false,
        }
    }
}

use days::{is_holiday, WeekDay};

pub fn main() {
    let today = WeekDay::Friday;
    if is_holiday(&today) {
        println!("I can go out!");
    } else {
        println!("I have to work today!");
    }
}
