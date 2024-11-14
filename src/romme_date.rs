use crate::date::{DateError, FrcDate};

const DAYS_PER_400_YEARS: i32 = 365 * 400 + 97;
const DAYS_PER_100_YEARS: i32 = 365 * 100 + 24;
const DAYS_PER_4_YEARS: i32 = 365 * 4 + 1;

pub type DateResult = Result<RommeDate, DateError>;

type RommeDelta = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents an FRC date that adheres to the Romme Rule
pub struct RommeDate {
    days: RommeDelta,
    year: i32,
    month: u8,
    day: u8,
}

impl FrcDate for RommeDate {
    fn month_int(&self) -> u8 {
        self.month
    }

    fn day(&self) -> u8 {
        self.day
    }

    fn year(&self) -> i32 {
        self.year
            .try_into()
            .expect("Cannot convert this year into an i32")
    }

    fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
    }
}

impl RommeDate {
    pub fn today_local() -> DateResult {
        let today = time::OffsetDateTime::now_local()?;

        let delta = (today.date() - RommeDate::epoch()).whole_days();

        Ok(RommeDate::new(delta.try_into().unwrap()))
    }

    pub fn today_utc() -> Self {
        let today = time::OffsetDateTime::now_utc();

        let delta = (today.date() - RommeDate::epoch()).whole_days();

        RommeDate::new(delta.try_into().unwrap())
    }

    pub fn from_georgian_date(year: i32, month: u8, day: u8) -> DateResult {
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

        // TODO deal with i64 type properly
        let delta = (date - RommeDate::epoch()).whole_days();

        Ok(RommeDate::new(delta.try_into().unwrap()))
    }

    fn new(days: RommeDelta) -> Self {
        let (year, month, day) = RommeDate::from_days(days);

        Self {
            days,
            year,
            month,
            day,
        }
    }

    fn from_days(days: RommeDelta) -> (i32, u8, u8) {
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

        // TODO remove unwrap
        (
            years.try_into().unwrap(),
            month.try_into().unwrap(),
            days.try_into().unwrap(),
        )
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
            // these next two values differ from my original implementation:
            // https://github.com/rfaulhaber/fdate/blob/a83c6305b69144f2a5d3b0671302adc8dda3bc93/fdate_test.go#L202-L213
            // I'm not sure what the difference is exactly, but these values seem to be correct when cross-referenced with other tools that use the same method
            (81813, (224, 13, 6)),
            (81810, (224, 13, 3)),
            (82178, (225, 13, 5)),
            (82179, (226, 1, 1)),
            (84789, (233, 2, 24)),
        ];

        for (days, expected) in dates {
            let result = RommeDate::from_days(days);

            assert_eq!(
                result, expected,
                "Expected {:?} for {:?} days but got {:?}",
                expected, days, result
            );
        }
    }
}
