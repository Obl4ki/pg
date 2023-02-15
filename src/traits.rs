use chrono::*;

pub trait GetToday {
    fn today(&self) -> NaiveDate;
}

