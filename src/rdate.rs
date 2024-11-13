use crate::{date::FrcDate, month::Month};
use thiserror::Error;

const DAYS_PER_400_YEARS: i32 = 365 * 400 + 97;
const DAYS_PER_100_YEARS: i32 = 365 * 100 + 24;
const DAYS_PER_4_YEARS: i32 = 365 * 4 + 1;

pub type DateResult = Result<RommeDate, DateError>;

#[derive(Debug, Clone, Error)]
pub enum DateError {}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents an FRC date that adheres to the Romme Rule
pub struct RommeDate {
    days: i32,
    year: u32,
    month: u8,
    day: u8,
}

impl FrcDate for RommeDate {
    fn month_int(&self) -> u8 {
        todo!()
    }

    fn day(&self) -> u8 {
        todo!()
    }

    fn year(&self) -> i32 {
        todo!()
    }

    fn is_leap_year(&self) -> bool {
        todo!()
    }
}

impl RommeDate {
    pub fn today_local() -> DateResult {
        todo!()
    }

    pub fn today_utc() -> Self {
        todo!()
    }

    pub fn from_georgian_date(year: i32, month: u8, day: u8) -> DateResult {
        todo!()
    }

    fn new(days: i32) -> Self {
        todo!()
    }

    fn from_days(days: i32) -> (i32, u8, u8) {
        let mut days = days;
        let mut years = 0;

        let days_per_400_years = days / DAYS_PER_400_YEARS;
        years = 400 * days_per_400_years;
        days -= DAYS_PER_400_YEARS * days_per_400_years;

        let mut days_per_100_years = days / DAYS_PER_100_YEARS;
        days_per_100_years -= days_per_100_years >> 2;
        years += 100 * days_per_100_years;
        days -= DAYS_PER_100_YEARS * days_per_100_years;

        let days_per_4_years = days / DAYS_PER_4_YEARS;
        years += 4 * days_per_4_years;
        days -= DAYS_PER_4_YEARS * days_per_4_years;

        let mut days_per_year = days / 365;
        days_per_year -= days_per_year >> 2;
        years += days_per_year;
        days -= 365 * days_per_year;

        years += 1;

        let mut month = days / 30;

        if days > 360 {
            days = days - 360 + 1;
            month = 13;
        } else {
            let end = 30 * (month + 1);

            let mut begin = 30 * month;

            if days >= end {
                month += 1;
                begin = end;
            }

            month += 1;

            days = days - begin + 1;
        }

        (years, month.try_into().unwrap(), days.try_into().unwrap())
    }

    fn epoch() -> time::Date {
        time::Date::from_calendar_date(1792, time::Month::September, 22).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::RommeDate;

    #[test]
    fn day_conversion() {
        let dates = vec![
            (0, (1, 1, 1)),
            (2603, (8, 2, 18)),
            (673, (2, 11, 9)),
            (81814, (225, 1, 1)),
            (81850, (225, 2, 7)),
            (82215, (226, 2, 7)),
            (82580, (227, 2, 7)),
            (81813, (224, 13, 5)),
            (81810, (224, 13, 2)),
        ];

        for (days, expected) in dates {
            let result = RommeDate::from_days(days);

            assert_eq!(result, expected);
        }
    }
}
