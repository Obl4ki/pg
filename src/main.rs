use std::error::Error;

use chrono::prelude::*;
use piggy::traits;
use piggy::UserInputData;
struct NowFromChrono;

impl traits::GetToday for NowFromChrono {
    fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = UserInputData {
        payout_day_of_month: 23,
        money_amount: 1900,
    };
    let payout_date =
        piggy::next_payout_date(data.payout_day_of_month, NowFromChrono).ok_or(format!(
            "Payroll date calculation failed with the following data: {:?}. Check your inputs.",
            &data
        ))?;

    let days_until_payout = piggy::days_until_payout(payout_date, NowFromChrono);

    println!("{days_until_payout:?} days left");

    Ok(())
}
