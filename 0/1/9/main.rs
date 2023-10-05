#[derive(Debug, PartialEq)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

impl Date {
    const M_D: &[i32] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    fn new(y: i32, m: i32, d: i32) -> Date {
        Date {
            year: y,
            month: m,
            day: d,
        }
    }
}

impl Iterator for Date {
    type Item = Date;
    fn next(&mut self) -> Option<Self::Item> {
        self.day += 1;
        if self.month == 2 && self.day == 29 && self.year % 4 == 0 && self.year % 400 != 0 {
            return Some(Date { ..*self });
        } else if self.day > Date::M_D[(self.month - 1) as usize] {
            self.day = 1;
            self.month += 1;
        }
        if self.month > 12 {
            self.month = 1;
            self.year += 1;
        }
        Some(Date { ..*self })
    }
}

use std::iter;

fn main() {
    let d = Date::new(1901, 1, 1);
    let mut week_day = 0;
    let week = iter::repeat_with(|| {
        week_day += 1;
        if week_day == 8 {
            week_day = 1
        };
        week_day
    });
    let date_week = d.zip(week);
    let mut count = 0;
    for i in date_week {
        if i.0.day == 1 && i.1 == 7 {
            count += 1;
        }
        if i.0
            == (Date {
                year: 2000,
                month: 12,
                day: 31,
            })
        {
            break;
        }
    }
    println!("{}", count);
}
