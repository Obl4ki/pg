use crate::traits::GetToday;

use chrono::prelude::*;

pub struct NowFromChrono;

impl GetToday for NowFromChrono {
    fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }
}
