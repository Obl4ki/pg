use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = piggy::get_data::from_stdin()?;

    let payout_date = piggy::next_payout_date(data.payout_day_of_month)?;

    let days_until_payout = piggy::days_until_payout(payout_date)?;

    let amount_per_day = piggy::calculate_money_per_day(data.money_amount, days_until_payout);

    println!("Zostalo {days_until_payout} dni.");

    println!("{amount_per_day}");
    Ok(())
}
