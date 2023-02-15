use std::ops::Add;

use chrono::prelude::*;
use chronoutil::RelativeDuration;

#[derive(Clone, Copy, Debug)]
pub struct UserInputData {
    pub payout_day_of_month: u32,
    pub money_amount: u32,
}

pub fn calculate_next_payout_date(day: u32) -> Option<DateTime<Utc>> {
    let now = Utc::now();
    let date_of_payroll = now.with_day(day)?;

    if now.day() >= day {
        let month_after_payroll = date_of_payroll + RelativeDuration::months(1);

        Some(month_after_payroll)
    } else {
        now.with_day(day)
    }
}
