use std::{
    fmt::Display,
    time::{Duration, SystemTime},
};

pub(crate) mod numeral;

// TODO
const DAYS_PER_400_YEARS: u64 = 365 * 400 + 97;
const DAYS_PER_100_YEARS: u64 = 365 * 100 + 24;
const DAYS_PER_4_YEARS: u64 = 365 * 4 + 1;

#[derive(Debug, Clone)]
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
    pub fn today() -> Date {
        todo!();
    }

    fn from_gregorian(date: time::Date) -> Self {
        let epoch = epoch_gregorian();

        let days_since = (epoch - date).whole_days();

        Self {
            days: days_since as u32,
        }
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
        let tests = &[(
            time::Date::from_calendar_date(1792, time::Month::September, 22).unwrap(),
            Date { days: 0 },
        )];

        for &(gdate, fdate) in tests {
            assert_eq!(Date::from_gregorian(gdate), fdate)
        }
    }
}
