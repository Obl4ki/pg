mod money_per_day;

use crate::money_per_day::app::{App, AppResponse};
use crate::money_per_day::data_access;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_access::from_stdin()?;

    let AppResponse {
        days_until_payout,
        amount_per_day,
    } = App::from_data(data).run()?;

    println!("Zostalo {days_until_payout} dni.");

    println!("{amount_per_day}");
    Ok(())
}
