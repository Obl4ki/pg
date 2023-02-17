use crate::money_per_day::locale::Lang;
use std::io::stdin;
#[derive(Clone, Copy, Debug)]
pub struct UserInputData {
    pub payout_day_of_month: u32,
    pub money_amount: u32,
}

pub fn from_stdin<L: Lang>() -> Result<UserInputData, String> {
    println!("{}", L::input_payday_number());
    let mut payout_day = String::new();
    stdin().read_line(&mut payout_day).map_err(|_| {
        format!("Failed parsing the line {payout_day}, expected non-negative number.")
    })?;

    let payout_day_of_month = payout_day.trim().parse().map_err(|_| {
        format!("{payout_day} should be a non-negative integer representing day of month.")
    })?;

    println!("{}", L::input_account_balance());
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
