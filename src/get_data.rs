use crate::UserInputData;
use std::io::stdin;

pub fn from_stdin() -> Result<UserInputData, String> {
    println!("Wprowadz dzien wyplaty: ");
    let mut payout_day = String::new();
    stdin().read_line(&mut payout_day).map_err(|_| {
        format!("Failed parsing the line {payout_day}, expected non-negative number.")
    })?;

    let payout_day_of_month = payout_day.trim().parse().map_err(|_| {
        format!("{payout_day} should be a non-negative integer representing day of month.")
    })?;

    println!("Wprowadz stan konta");
    let mut money_amount = String::new();
    stdin().read_line(&mut money_amount).map_err(|_| {
        format!("Failed parsing the line {money_amount}, expected non-negative number.")
    })?;

    let money_amount = money_amount.trim().parse().map_err(|_| {
        format!(
        "{payout_day} should be a non-negative integer representing the amount of money you have."
    )
    })?;

    Ok(UserInputData {
        payout_day_of_month,
        money_amount,
    })
}
