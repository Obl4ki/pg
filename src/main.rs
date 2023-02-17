mod money_per_day;

use money_per_day::app::{App, AppResponse};
use money_per_day::get_data;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_data::from_stdin()?;

    let AppResponse {
        days_until_payout,
        amount_per_day,
    } = App::from_data(data).run()?;

    println!("Zostalo {days_until_payout} dni.");

    println!("{amount_per_day}");
    Ok(())
}
