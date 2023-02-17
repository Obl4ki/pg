use chrono::prelude::*;

pub trait GetToday {
    fn today(&self) -> NaiveDate;
}

pub struct NowFromChrono;

impl GetToday for NowFromChrono {
    fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }
}
