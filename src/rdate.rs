use crate::{date::FrcDate, month::Month};
use thiserror::Error;

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
}
