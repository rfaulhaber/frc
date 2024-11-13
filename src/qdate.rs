use std::fmt::Display;

use crate::{
    cal,
    date::{DateResult, FrcDate},
    numeral,
};

type JDN = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QDate {
    jdn: JDN,
    year: i32,
    month: u8,
    day: u8,
}

impl Display for QDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = if self.year < 0 || self.year > (numeral::MAX_NUMERAL_VALUE as i32) {
            format!("{}", self.year)
        } else {
            format!("{}", numeral::to_numeral(self.year as u16).unwrap())
        };

        write!(f, "{} {} An {}", self.day, self.month, year)
    }
}

impl FrcDate for QDate {
    fn month_int(&self) -> u8 {
        self.month
    }

    fn day(&self) -> u8 {
        self.day
    }

    fn year(&self) -> i32 {
        self.year
    }

    fn is_leap_year(&self) -> bool {
        QDate::leap_year(self.year)
    }
}

impl QDate {
    pub fn today_utc() -> Self {
        let today = time::OffsetDateTime::now_utc();

        let jdn = today.to_julian_day();

        QDate::new(jdn)
    }

    pub fn today_local() -> DateResult<Self> {
        let today = time::OffsetDateTime::now_local()?;

        let jdn = today.to_julian_day();

        Ok(QDate::new(jdn))
    }

    pub fn from_gregorian_date(year: i32, month: u8, day: u8) -> DateResult<Self> {
        let month = match month {
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
            _ => return Err(crate::date::DateError::InvalidDate),
        };

        let date = time::Date::from_calendar_date(year, month as time::Month, day)?;

        Ok(QDate::new(date.to_julian_day()))
    }

    /// Returns the Julian Day for this date
    pub fn jdn(&self) -> JDN {
        self.jdn
    }

    fn new(jdn: i32) -> Self {
        let (year, month, day) = QDate::from_julian_day(jdn);

        Self {
            jdn,
            year,
            month,
            day,
        }
    }

    fn from_julian_day(day: JDN) -> (i32, u8, u8) {
        let mut low: i32 = 0;
        let mut high: i32 = 30391; // hard-coded length of leap year slice length

        let leaps_to_date = cal::leaps_to_date();

        while low + 1 < high {
            let mid = (low + high) / 2;
            if cal::START_JD + 365 * mid + (leaps_to_date[mid as usize] as i32) <= day {
                low = mid;
            } else {
                high = mid;
            }
        }

        let dd = day
            - ((cal::START_JD as i32)
                + (365 as i32) * (low as i32)
                + (leaps_to_date[low as usize] as i32));

        let year = cal::START_YEAR + (low as i32);
        let month = (dd / 30 + 1) as u8;
        let day = (dd % 30 + 1) as u8;

        (year, month, day)
    }

    fn to_julian_day(year: i32, month: u8, day: u8) -> JDN {
        let dy = year - cal::START_YEAR;
        let dd: i32 = ((month as u32) * 30 + (day as u32) - 31) as i32;

        let leaps_to_date = cal::leaps_to_date();

        cal::START_JD + 365 * dy + (leaps_to_date[dy as usize] as i32) + (dd as i32)
    }

    fn leap_year(year: i32) -> bool {
        cal::LEAP_YEARS[(year - cal::START_YEAR) as usize] == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::month::Month;

    use super::*;

    #[test]
    fn julian_day_conversion() {
        let dates = vec![
            (2375840, (1, 1, 1)),
            (2378444, (8, 2, 18)),
            (2416017, (111, 1, 1)),
            (2450715, (206, 1, 1)),
            (3284926, (2490, 1, 1)),
            (2416016, (110, 13, 6)),
            (2450714, (205, 13, 6)),
            (3284925, (2489, 13, 5)),
        ];

        for (jdn, frc_date) in dates {
            assert_eq!(QDate::from_julian_day(jdn), frc_date);
        }
    }

    #[test]
    fn date_to_jdn() {
        let dates = vec![
            ((233, 2, 1), 2460606),
            ((1, 1, 1), 2375840),
            ((8, 2, 18), 2378444),
            ((111, 1, 1), 2416017),
            ((206, 1, 1), 2450715),
            ((2490, 1, 1), 3284926),
        ];

        for ((year, month, day), jdn) in dates {
            assert_eq!(QDate::to_julian_day(year, month, day), jdn);
        }
    }

    #[test]
    fn leap_years() {
        let years = &[
            (1, false),
            (8, false),
            (3, true),
            (7, true),
            (11, true),
            (110, true),
            (205, true),
            (2489, false),
            (111, false),
            (206, false),
            (2490, true),
        ];

        for &(year, is_leap) in years {
            let res = QDate::leap_year(year);
            assert_eq!(
                res, is_leap,
                "year {} was {} but should be {}",
                year, res, is_leap
            );
        }
    }

    #[test]
    fn constructs_end_of_year_correctly() {
        let leap_year_date = QDate::from_gregorian_date(1795, 9, 22).unwrap();

        assert_eq!(leap_year_date.month(), Month::Complémentaires);
        assert_eq!(leap_year_date.year(), 3);
        assert_eq!(leap_year_date.day(), 6);
        assert_eq!(leap_year_date.day_of_year(), 366);
        assert!(leap_year_date.is_leap_year());

        let regular_date = QDate::from_gregorian_date(1793, 9, 21).unwrap();
        assert_eq!(regular_date.month(), Month::Complémentaires);
        assert_eq!(regular_date.year(), 1);
        assert_eq!(regular_date.day(), 5);
        assert_eq!(regular_date.day_of_year(), 365);
        assert!(!regular_date.is_leap_year());
    }

    #[test]
    fn date_properties() {
        let date = QDate::new(2460606);

        assert_eq!(date.month(), Month::Brumaire);
        assert_eq!(date.year(), 233);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_of_year(), 31);
    }
}
