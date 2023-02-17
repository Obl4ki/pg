use piggy::app::{App, AppResponse};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = piggy::get_data::from_stdin()?;

    let AppResponse {
        days_until_payout,
        amount_per_day,
    } = App::from_data(data).run()?;

    println!("Zostalo {days_until_payout} dni.");

    println!("{amount_per_day}");
    Ok(())
}
