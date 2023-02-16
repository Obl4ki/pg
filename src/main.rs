use std::error::Error;

use chrono::prelude::*;
use piggy::get_data;
use piggy::traits;

struct NowFromChrono;

impl traits::GetToday for NowFromChrono {
    fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_data::from_stdin()?;

    let payout_date = piggy::next_payout_date(data.payout_day_of_month, NowFromChrono)?;

    let days_until_payout = piggy::days_until_payout(payout_date, NowFromChrono)?;

    let amount_per_day = piggy::calculate_money_per_day(data.money_amount, days_until_payout);

    println!("Zostalo {days_until_payout} dni.");

    println!("{amount_per_day}");
    Ok(())
}
