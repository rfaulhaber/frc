// see: https://github.com/quantum5/qcal/blob/master/common/src/french/index.ts

use std::fmt::Display;

use numeral::to_numeral;
use thiserror::Error;

mod cal;
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
    Complimentary(ComplimentaryWeekday),
    Ordinary(OrdinaryWeekday),
}

pub enum ComplimentaryWeekday {
    Vertu = 1,
    Génie = 2,
    Travail = 3,
    #[allow(non_camel_case_types)]
    lOpinion = 4,
    Récompenses = 5,
    Révolution = 6,
}

impl Display for ComplimentaryWeekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ComplimentaryWeekday::Vertu => "La Fête de la Vertu",
                ComplimentaryWeekday::Génie => "La Fête du Génie",
                ComplimentaryWeekday::Travail => "La Fête du Travail",
                ComplimentaryWeekday::lOpinion => "La Fête de l'Opinion",
                ComplimentaryWeekday::Récompenses => "La Fête des Récompenses",
                ComplimentaryWeekday::Révolution => "La Fête de la Révolution",
            }
        )
    }
}

impl ComplimentaryWeekday {
    fn day_of_week(n: i32) -> ComplimentaryWeekday {
        match n {
            1 => ComplimentaryWeekday::Vertu,
            2 => ComplimentaryWeekday::Génie,
            3 => ComplimentaryWeekday::Travail,
            4 => ComplimentaryWeekday::lOpinion,
            5 => ComplimentaryWeekday::Récompenses,
            6 => ComplimentaryWeekday::Révolution,
            _ => unreachable!("invalid value passed to day of week"),
        }
    }
}

pub enum OrdinaryWeekday {
    Primidi = 1,
    Duodi = 2,
    Tridi = 3,
    Quartidi = 4,
    Quintidi = 5,
    Sextidi = 6,
    Septidi = 7,
    Octidi = 8,
    Nonidi = 9,
    Décadi = 10,
}

impl Display for OrdinaryWeekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrdinaryWeekday::Primidi => "Primidi",
                OrdinaryWeekday::Duodi => "Duodi",
                OrdinaryWeekday::Tridi => "Tridi",
                OrdinaryWeekday::Quartidi => "Quartidi",
                OrdinaryWeekday::Quintidi => "Quintidi",
                OrdinaryWeekday::Sextidi => "Sextidi",
                OrdinaryWeekday::Septidi => "Septidi",
                OrdinaryWeekday::Octidi => "Octidi",
                OrdinaryWeekday::Nonidi => "Nonidi",
                OrdinaryWeekday::Décadi => "Décadi",
            }
        )
    }
}

impl OrdinaryWeekday {
    fn day_of_week(n: i32) -> OrdinaryWeekday {
        match n {
            1 => OrdinaryWeekday::Primidi,
            2 => OrdinaryWeekday::Duodi,
            3 => OrdinaryWeekday::Tridi,
            4 => OrdinaryWeekday::Quartidi,
            5 => OrdinaryWeekday::Quintidi,
            6 => OrdinaryWeekday::Sextidi,
            7 => OrdinaryWeekday::Septidi,
            8 => OrdinaryWeekday::Octidi,
            9 => OrdinaryWeekday::Nonidi,
            10 => OrdinaryWeekday::Décadi,
            _ => unreachable!("invalid value passed {:?}", n),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = if self.year < 0 || self.year > (numeral::MAX_NUMERAL_VALUE as i32) {
            format!("{}", self.year)
        } else {
            format!("{}", to_numeral(self.year as u16).unwrap())
        };

        write!(f, "{} {} An {}", self.day, self.month, year)
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
        self.year_month_day().2 as u16
    }

    pub fn weekday(&self) -> Weekday {
        let (_, month, day) = self.year_month_day();

        if month == Month::Complémentaires {
            Weekday::Complimentary(ComplimentaryWeekday::day_of_week(day))
        } else {
            Weekday::Ordinary(OrdinaryWeekday::day_of_week(day % 11))
        }
    }

    pub fn year(&self) -> u8 {
        self.year_month_day().0 as u8
    }

    pub fn is_leap_year(&self) -> bool {
        let (year, _, _) = self.year_month_day();
        Date::is_int_leap_year(year)
    }

    fn from_gregorian(date: time::Date) -> Self {
        let epoch = epoch_gregorian();

        let days_since = (date - epoch).whole_days();

        Self {
            days: days_since as u32,
        }
    }

    fn year_month_day(&self) -> (i32, Month, i32) {
        let mut days = self.days;
        let mut years = 1;

        let days_400_years = days / DAYS_PER_400_YEARS;
        days -= DAYS_PER_400_YEARS * days_400_years;
        years += days_400_years * 400;

        let mut days_100_years = days / DAYS_PER_100_YEARS;
        days_100_years -= days_100_years >> 2;
        days -= DAYS_PER_100_YEARS * days_100_years;
        years += days_100_years * 100;

        let days_4_years = days / DAYS_PER_4_YEARS;
        days -= DAYS_PER_4_YEARS * days_4_years;
        years += days_4_years * 4;

        let mut remaining_years = days / 365;
        remaining_years -= remaining_years >> 2;
        days -= remaining_years * 365;
        years += remaining_years;

        let mut months = days / 30;

        if days > 360 {
            days = days - 360 + 1;
        } else {
            let end = 30 * (months + 1);

            let mut begin = end;

            if days >= end {
                months += 1;
            } else {
                begin = 30 * months;
            }

            days = days - begin + 1;
        }

        let month = Month::nth(
            (months + 1)
                .try_into()
                .expect("months value is larger than expected"),
        );

        (
            years.try_into().expect("years is larger than expected"),
            month,
            days.try_into().expect("days is larger than expected"),
        )
    }

    const fn is_int_leap_year(year: i32) -> bool {
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
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
                time::Date::from_calendar_date(2024, time::Month::November, 7).unwrap(),
                Date { days: 84782 },
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
            (Date { days: 1 }, (1, Month::Vendémiaire, 2)),
            (Date { days: 2603 }, (8, Month::Brumaire, 18)),
            (Date { days: 673 }, (2, Month::Thermidor, 9)),
            (Date { days: 82215 }, (226, Month::Brumaire, 7)),
            (Date { days: 81850 }, (225, Month::Brumaire, 7)),
            (Date { days: 82580 }, (227, Month::Brumaire, 7)),
            (Date { days: 84782 }, (233, Month::Brumaire, 17)),
            // (Date { days: 81808 }, (224, Month::Fructidor, 30)),
            (Date { days: 81810 }, (224, Month::Complémentaires, 2)),
            (Date { days: 81813 }, (224, Month::Complémentaires, 5)),
        ];

        for &(date, expected) in tests {
            let year = date.year();
            let month = date.month();
            let day = date.day();

            let actual = (year, month, day);
            assert_eq!(actual, expected, "Mismatch for date {:?}", date);
        }
    }
}
