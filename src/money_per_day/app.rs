use crate::money_per_day::domain::{calculate_money_per_day, days_until_payout, next_payout_date};
use crate::money_per_day::locale::Lang;

use super::data_access::from_stdin;

pub struct App;

#[derive(Clone, Copy, Debug)]
pub struct AppResponse {
    pub days_until_payout: u32,
    pub amount_per_day: f32,
}

impl AppResponse {
    pub fn print<L: Lang>(&self) {
        println!("{}", L::how_many_days_left(self.days_until_payout));

        println!("{}", L::how_much_money_per_day(self.amount_per_day));
    }
}

impl App {
    pub fn run() -> Result<AppResponse, String> {
        let data = from_stdin()?;

        let payout_date = next_payout_date(data.payout_day_of_month)?;

        let days_until_payout = days_until_payout(payout_date)?;

        let amount_per_day = calculate_money_per_day(data.money_amount, days_until_payout);

        Ok(AppResponse {
            days_until_payout,
            amount_per_day,
        })
    }
}
