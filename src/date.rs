use std::fmt::Display;

use crate::{
    cal,
    month::Month,
    numeral,
    weekday::{Complimentary, Ordinary, Weekday},
};

pub trait FrcDate {
    fn month_int(&self) -> u8;
    fn day(&self) -> u8;
    fn year(&self) -> i32;
    fn is_leap_year(&self) -> bool;

    fn weekday(&self) -> Weekday {
        if self.month_int() == 13 {
            Weekday::Complimentary(Complimentary::day_of_week(self.day().into()))
        } else {
            let day_of_week = self.day() % 11;
            Weekday::Ordinary(Ordinary::day_of_week(day_of_week.into()))
        }
    }

    fn month(&self) -> Month {
        Month::nth(self.month_int())
    }

    fn day_of_year(&self) -> u8 {
        (self.month_int() - 1) * 30 + self.day()
    }

    fn rural_day(&self) -> Option<(&str, &str)> {
        cal::RURAL_DAYS.get(self.day_of_year() as usize).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date<D: FrcDate> {
    date: D,
}

impl<D: FrcDate> Display for Date<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = if self.date.year() < 0 || self.date.year() > (numeral::MAX_NUMERAL_VALUE as i32)
        {
            format!("{}", self.date.year())
        } else {
            format!("{}", numeral::to_numeral(self.date.year() as u16).unwrap())
        };

        write!(f, "{} {} An {}", self.date.day(), self.date.month(), year)
    }
}
