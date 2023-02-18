use crate::money_per_day::locale::Lang;
use std::io::stdin;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Failed reading from stdin")]
    Stdin,
    #[error("Failed parsing the day number {line} - not a number")]
    DayParse { line: String },
    #[error("Money amount should be a non-negative integer, got {amount} instead")]
    MoneyParse { amount: String },
}

#[derive(Clone, Copy, Debug)]
pub struct UserInputData {
    pub payout_day_of_month: u32,
    pub money_amount: u32,
}

pub fn from_stdin<L: Lang>() -> Result<UserInputData, InputError> {
    println!("{}", L::input_payday_number());
    let mut payout_day_line = String::new();
    stdin()
        .read_line(&mut payout_day_line)
        .map_err(|_| InputError::Stdin)?;

    let payout_day_of_month = payout_day_line
        .trim()
        .parse()
        .map_err(|_| InputError::DayParse {
            line: payout_day_line,
        })?;

    println!("{}", L::input_account_balance());

    let mut money_amount_line = String::new();
    stdin()
        .read_line(&mut money_amount_line)
        .map_err(|_| InputError::Stdin)?;

    let money_amount = money_amount_line
        .trim()
        .parse()
        .map_err(|_| InputError::MoneyParse {
            amount: money_amount_line,
        })?;

    Ok(UserInputData {
        payout_day_of_month,
        money_amount,
    })
}
