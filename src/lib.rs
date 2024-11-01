use std::{
    fmt::Display,
    time::{Duration, SystemTime},
};

use thiserror::Error;

pub(crate) mod numeral;

#[derive(Debug, Error)]
pub enum DateError {
    #[error("Parameters passed create an invalid FRC date")]
    InvalidDate,

    #[error("Invalid Georgian calendar date provided")]
    InvalidGeorgianCalendarDate,

    #[error("Cannot determine time zone")]
    IndeterminateTimezone(#[from] time::error::IndeterminateOffset),

    #[error("Specified parameter was out of range")]
    ComponentRange(#[from] time::error::ComponentRange),
}

pub type DateResult = Result<Date, DateError>;

// TODO
const DAYS_PER_400_YEARS: u32 = 365 * 400 + 97;
const DAYS_PER_100_YEARS: u32 = 365 * 100 + 24;
const DAYS_PER_4_YEARS: u32 = 365 * 4 + 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Vendémiaire,
    Brumaire,
    Frimaire,
    Nivôse,
    Pluviôse,
    Ventôse,
    Germinal,
    Floréal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
    Complémentaires,
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Month::Vendémiaire => "Vendémiaire",
                Month::Brumaire => "Brumaire",
                Month::Frimaire => "Frimaire",
                Month::Nivôse => "Nivôse",
                Month::Pluviôse => "Pluviôse",
                Month::Ventôse => "Ventôse",
                Month::Germinal => "Germinal",
                Month::Floréal => "Floréal",
                Month::Prairial => "Prairial",
                Month::Messidor => "Messidor",
                Month::Thermidor => "Thermidor",
                Month::Fructidor => "Fructidor",
                Month::Complémentaires => "Complémentaires",
            }
        )
    }
}

impl Month {
    pub fn value(&self) -> u8 {
        match self {
            Month::Vendémiaire => 1,
            Month::Brumaire => 2,
            Month::Frimaire => 3,
            Month::Nivôse => 4,
            Month::Pluviôse => 5,
            Month::Ventôse => 6,
            Month::Germinal => 7,
            Month::Floréal => 8,
            Month::Prairial => 9,
            Month::Messidor => 10,
            Month::Thermidor => 11,
            Month::Fructidor => 12,
            Month::Complémentaires => 13,
        }
    }

    pub fn next_month(&self) -> Month {
        match self {
            Month::Vendémiaire => Month::Brumaire,
            Month::Brumaire => Month::Frimaire,
            Month::Frimaire => Month::Nivôse,
            Month::Nivôse => Month::Pluviôse,
            Month::Pluviôse => Month::Ventôse,
            Month::Ventôse => Month::Germinal,
            Month::Germinal => Month::Floréal,
            Month::Floréal => Month::Prairial,
            Month::Prairial => Month::Messidor,
            Month::Messidor => Month::Thermidor,
            Month::Thermidor => Month::Fructidor,
            Month::Fructidor => Month::Complémentaires,
            Month::Complémentaires => Month::Vendémiaire,
        }
    }

    pub fn previous_month(&self) -> Month {
        match self {
            Month::Brumaire => Month::Vendémiaire,
            Month::Frimaire => Month::Brumaire,
            Month::Nivôse => Month::Frimaire,
            Month::Pluviôse => Month::Nivôse,
            Month::Ventôse => Month::Pluviôse,
            Month::Germinal => Month::Ventôse,
            Month::Floréal => Month::Germinal,
            Month::Prairial => Month::Floréal,
            Month::Messidor => Month::Prairial,
            Month::Thermidor => Month::Messidor,
            Month::Fructidor => Month::Thermidor,
            Month::Complémentaires => Month::Fructidor,
            Month::Vendémiaire => Month::Complémentaires,
        }
    }

    fn nth(n: u8) -> Month {
        match n {
            1 => Month::Vendémiaire,
            2 => Month::Brumaire,
            3 => Month::Frimaire,
            4 => Month::Nivôse,
            5 => Month::Pluviôse,
            6 => Month::Ventôse,
            7 => Month::Germinal,
            8 => Month::Floréal,
            9 => Month::Prairial,
            10 => Month::Messidor,
            11 => Month::Thermidor,
            12 => Month::Fructidor,
            13 => Month::Complémentaires,
            _ => unreachable!("invalid month: {}", n),
        }
    }
}

pub enum Weekday {
    Primidi,
    Duodi,
    Tridi,
    Quartidi,
    Quintidi,
    Sextidi,
    Septidi,
    Octidi,
    Nonidi,
    Décadi,
}

impl Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Weekday::Primidi => "Primidi",
                Weekday::Duodi => "Duodi",
                Weekday::Tridi => "Tridi",
                Weekday::Quartidi => "Quartidi",
                Weekday::Quintidi => "Quintidi",
                Weekday::Sextidi => "Sextidi",
                Weekday::Septidi => "Septidi",
                Weekday::Octidi => "Octidi",
                Weekday::Nonidi => "Nonidi",
                Weekday::Décadi => "Décadi",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date {
    days: u32,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Date {
    pub fn today_utc() -> Date {
        let now = time::OffsetDateTime::now_utc();
        Date::from_gregorian(now.date())
    }

    pub fn today_local() -> DateResult {
        let now = time::OffsetDateTime::now_local()?;
        Ok(Date::from_gregorian(now.date()))
    }

    pub fn from_gregorian_calendar_date(year: u8, month: u8, day: u8) -> DateResult {
        let time_month = match month {
            1 => time::Month::January,
            2 => time::Month::February,
            3 => time::Month::March,
            4 => time::Month::April,
            5 => time::Month::May,
            6 => time::Month::June,
            7 => time::Month::July,
            8 => time::Month::August,
            9 => time::Month::September,
            10 => time::Month::October,
            11 => time::Month::November,
            12 => time::Month::December,
            _ => return Err(DateError::InvalidGeorgianCalendarDate),
        };

        let date = time::Date::from_calendar_date(year as i32, time_month, day)?;

        Ok(Date::from_gregorian(date))
    }

    pub fn month(&self) -> Month {
        self.year_month_day().1
    }

    pub fn day(&self) -> u16 {
        self.year_month_day().2
    }

    pub fn weekday(&self) -> Weekday {
        todo!()
    }

    pub fn year(&self) -> u8 {
        self.year_month_day().0
    }

    fn from_gregorian(date: time::Date) -> Self {
        let epoch = epoch_gregorian();

        let debug = date - epoch;
        let days_since = (date - epoch).whole_days();

        Self {
            days: days_since as u32,
        }
    }

    fn year_month_day(&self) -> (u8, Month, u16) {
        let mut days = self.days;
        let mut years = 0;

        let mut years400 = days / DAYS_PER_400_YEARS;
        years400 -= years400 >> 2;
        years += years400 * 400;
        days -= DAYS_PER_400_YEARS * years400;

        let mut years100 = days / DAYS_PER_100_YEARS;
        years100 -= years100 >> 2;
        years += years100 * 100;
        days -= DAYS_PER_100_YEARS * years100;

        let leap_years = days / DAYS_PER_4_YEARS;
        years += leap_years * 4;
        days -= DAYS_PER_4_YEARS * leap_years;

        let mut y = days / 365;
        y -= y >> 2;
        years += y * 365;
        days -= 365 * y;

        years += 1;

        let n_months = days / 30;
        let date = days - (n_months * 30) + 1;

        let month = Month::nth((n_months + 1) as u8);

        (years as u8, month, date as u16)
    }

    fn is_leap_year(year: u16) -> bool {
        todo!();
    }
}

fn epoch_gregorian() -> time::Date {
    time::Date::from_calendar_date(1792, time::Month::September, 22).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gregorian_conversion() {
        let tests = &[
            (
                time::Date::from_calendar_date(1792, time::Month::September, 22).unwrap(),
                Date { days: 0 },
            ),
            (
                time::Date::from_calendar_date(1795, time::Month::November, 22).unwrap(),
                Date { days: 2604 },
            ),
        ];

        for &(gdate, fdate) in tests {
            assert_eq!(Date::from_gregorian(gdate), fdate)
        }
    }

    #[test]
    fn dates() {
        let tests = &[
            (Date { days: 0 }, (1, Month::Vendémiaire, 1)),
            (Date { days: 2604 }, (8, Month::Brumaire, 18)),
        ];

        for &(date, (year, month, day)) in tests {
            assert_eq!(date.year(), year);
            assert_eq!(date.month(), month);
            assert_eq!(date.day(), day);
        }
    }
}
