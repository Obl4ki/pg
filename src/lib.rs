pub mod traits;

use chrono::prelude::*;
use chronoutil::RelativeDuration;
use traits::GetToday;

#[derive(Clone, Copy, Debug)]
pub struct UserInputData {
    pub payout_day_of_month: u32,
    pub money_amount: u32,
}

pub fn next_payout_date<T: GetToday>(day: u32, today_provider: T) -> Option<NaiveDate> {
    let now = today_provider.today();
    let date_of_payroll = now.with_day(day)?;

    if now.day() >= day {
        let month_after_payroll = date_of_payroll + RelativeDuration::months(1);

        Some(month_after_payroll)
    } else {
        now.with_day(day)
    }
}

pub fn days_until_payout<T: GetToday>(
    payout_date: NaiveDate,
    today_provider: T,
) -> Result<u32, String> {
    let now = today_provider.today();
    let date_of_payroll = payout_date.signed_duration_since(now);
    if date_of_payroll.num_days() >= 0 {
        // Add 1 because the current day is also important
        Ok(1 + date_of_payroll.num_days() as u32)
    } else {
        Err(format!("{now:?} date is before {date_of_payroll:?}"))
    }
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
        let data = UserInputData {
            payout_day_of_month: 11,
            money_amount: 1900,
        };

        let payout_date = super::next_payout_date(data.payout_day_of_month, MockNow)
            .ok_or(format!(
                "Payroll date calculation failed with the following data: {:?}. Check your inputs.",
                &data
            ))
            .unwrap();

        assert_eq!(payout_date.day(), 11);
        assert_eq!(payout_date.month(), 2);
    }

    #[test]
    fn test_next_payout_date_same_day() {
        let data = UserInputData {
            payout_day_of_month: 10,
            money_amount: 1900,
        };

        let payout_date = super::next_payout_date(data.payout_day_of_month, MockNow)
            .ok_or(format!(
                "Payroll date calculation failed with the following data: {:?}. Check your inputs.",
                &data
            ))
            .unwrap();

        assert_eq!(payout_date.day(), 10);
        assert_eq!(payout_date.month(), 3);
    }

    #[test]
    fn test_next_payout_date_day_before() {
        let data = UserInputData {
            payout_day_of_month: 9,
            money_amount: 1900,
        };

        let payout_date = super::next_payout_date(data.payout_day_of_month, MockNow)
            .ok_or(format!(
                "Payroll date calculation failed with the following data: {:?}. Check your inputs.",
                &data
            ))
            .unwrap();

        assert_eq!(payout_date.day(), 9);
        assert_eq!(payout_date.month(), 3);
    }

    #[test]
    fn test_get_days_until_payout() {
        let days_until_payout =
            super::days_until_payout(NaiveDate::from_ymd_opt(2023, 2, 20).unwrap(), MockNow);

        assert_eq!(days_until_payout.unwrap(), 11); // also count the current day!

        let days_until_payout =
            super::days_until_payout(NaiveDate::from_ymd_opt(2023, 2, 7).unwrap(), MockNow);
        //
        assert!(days_until_payout.is_err())
    }
}
