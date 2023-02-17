use crate::UserInputData;

#[derive(Clone, Copy, Debug)]
pub struct App {
    data: UserInputData,
}

impl App {
    pub fn from_data(data: UserInputData) -> Self {
        Self { data }
    }

    pub fn run(&self) -> Result<AppResponse, String> {
        let payout_date = crate::next_payout_date(self.data.payout_day_of_month)?;

        let days_until_payout = crate::days_until_payout(payout_date)?;

        let amount_per_day =
            crate::calculate_money_per_day(self.data.money_amount, days_until_payout);

        Ok(AppResponse {
            days_until_payout,
            amount_per_day,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AppResponse {
    pub days_until_payout: u32,
    pub amount_per_day: f32,
}
