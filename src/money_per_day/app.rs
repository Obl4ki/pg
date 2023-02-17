use crate::money_per_day::domain::{calculate_money_per_day, days_until_payout, next_payout_date};
use crate::money_per_day::locale::Lang;

use super::data_access::from_stdin;

pub struct App;

#[derive(Clone, Copy, Debug)]
pub struct AppResponse {
    pub days_until_payout: u32,
    pub amount_per_day: f32,
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

    pub fn print_response<L: Lang>(res: AppResponse) {
        println!("{}", L::how_many_days_left(res.days_until_payout));

        println!("{}", L::how_much_money_per_day(res.amount_per_day));
    }
}
