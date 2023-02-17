use std::marker::PhantomData;

use crate::money_per_day::data_access::UserInputData;
use crate::money_per_day::domain::{calculate_money_per_day, days_until_payout, next_payout_date};
use crate::money_per_day::locale::Lang;

#[derive(Clone, Copy, Debug)]
pub struct App<L: Lang> {
    data: UserInputData,
    lang_marker: PhantomData<L>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppResponse {
    pub days_until_payout: u32,
    pub amount_per_day: f32,
}

impl<L: Lang> App<L> {
    pub fn from_data(data: UserInputData) -> Self {
        Self {
            data,
            lang_marker: PhantomData,
        }
    }

    pub fn run(&self) -> Result<AppResponse, String> {
        let payout_date = next_payout_date(self.data.payout_day_of_month)?;

        let days_until_payout = days_until_payout(payout_date)?;

        let amount_per_day = calculate_money_per_day(self.data.money_amount, days_until_payout);

        Ok(AppResponse {
            days_until_payout,
            amount_per_day,
        })
    }

    pub fn print_response(&self, res: AppResponse) {
        println!("{}", L::how_many_days_left(res.days_until_payout));

        println!("{}", L::how_much_money_per_day(res.amount_per_day));
    }
}
