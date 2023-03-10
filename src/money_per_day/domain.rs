use crate::money_per_day::dates::GetToday;
use crate::money_per_day::dates::NowFromChrono;
use chrono::prelude::*;
use chrono::Duration;
use chronoutil::RelativeDuration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DateError {
    #[error("Day {day} is not correct")]
    DayNumberNotCorrect { day: u32 },
    #[error("{now:?} date is before {date_of_payroll:?}")]
    PaydayOccuredInThePast {
        now: NaiveDate,
        date_of_payroll: Duration,
    },
}

pub fn next_payout_date(day: u32) -> Result<NaiveDate, DateError> {
    _next_payout_date(day, NowFromChrono)
}

pub(crate) fn _next_payout_date<T: GetToday>(
    day: u32,
    today_provider: T,
) -> Result<NaiveDate, DateError> {
    let now = today_provider.today();
    let date_of_payroll = now
        .with_day(day)
        .ok_or(DateError::DayNumberNotCorrect { day })?;

    if now.day() >= day {
        let month_after_payroll = date_of_payroll + RelativeDuration::months(1);

        Ok(month_after_payroll)
    } else {
        Ok(date_of_payroll)
    }
}

pub fn days_until_payout(payout_date: NaiveDate) -> Result<u32, DateError> {
    _days_until_payout(payout_date, NowFromChrono)
}

pub(crate) fn _days_until_payout<T: GetToday>(
    payout_date: NaiveDate,
    today_provider: T,
) -> Result<u32, DateError> {
    let now = today_provider.today();
    let date_of_payroll = payout_date.signed_duration_since(now);
    if date_of_payroll.num_days() >= 0 {
        // Add 1 because the current day is also important
        Ok(1 + date_of_payroll.num_days() as u32)
    } else {
        Err(DateError::PaydayOccuredInThePast {
            now,
            date_of_payroll,
        })
    }
}

pub fn calculate_money_per_day(money_amount: u32, days_until_payout: u32) -> f32 {
    money_amount as f32 / days_until_payout as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockNow;
    impl GetToday for MockNow {
        fn today(&self) -> NaiveDate {
            NaiveDate::from_ymd_opt(2023, 2, 10).unwrap()
        }
    }

    #[test]
    fn test_next_payout_date_day_after() {
        let payout_date = super::_next_payout_date(11, MockNow).unwrap();

        assert_eq!(payout_date.day(), 11);
        assert_eq!(payout_date.month(), 2);
    }

    #[test]
    fn test_next_payout_date_same_day() {
        let payout_date = super::_next_payout_date(10, MockNow).unwrap();

        assert_eq!(payout_date.day(), 10);
        assert_eq!(payout_date.month(), 3);
    }

    #[test]
    fn test_next_payout_date_day_before() {
        let payout_date = super::_next_payout_date(9, MockNow).unwrap();

        assert_eq!(payout_date.day(), 9);
        assert_eq!(payout_date.month(), 3);
    }

    #[test]
    fn test_get_days_until_payout() {
        let days_until_payout =
            super::_days_until_payout(NaiveDate::from_ymd_opt(2023, 2, 20).unwrap(), MockNow);

        assert_eq!(days_until_payout.unwrap(), 11); // also count the current day!

        let days_until_payout =
            super::_days_until_payout(NaiveDate::from_ymd_opt(2023, 2, 7).unwrap(), MockNow);
        assert!(days_until_payout.is_err())
    }

    #[test]
    fn test_calculate_money_per_day() {
        let amount = 100;
        let days = 10;

        assert!(10. - calculate_money_per_day(amount, days).abs() < 0.001);

        let amount = 0;
        let days = 10;

        assert!(0. - calculate_money_per_day(amount, days).abs() < 0.001);
    }
}
