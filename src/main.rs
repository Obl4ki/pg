use std::error::Error;

use chrono::prelude::*;
use piggy::UserInputData;

fn main() -> Result<(), Box<dyn Error>> {
    let data = UserInputData {
        payout_day_of_month: 17,
        money_amount: 1900,
    };
    let payout_date: DateTime<Utc> = piggy::calculate_next_payout_date(data.payout_day_of_month)
        .ok_or(format!(
            "Payroll date calculation failed with the following data: {:?}. Check your inputs.",
            &data
        ))?;

    println!("{payout_date:?}");

    Ok(())
}
