#![feature(proc_macro_hygiene, decl_macro)]

mod money_per_day;
use crate::money_per_day::app::App;
use crate::money_per_day::data_access::UserInputData;

#[macro_use]
extern crate rocket;
use money_per_day::app::AppResponse;
use rocket_contrib::json::Json;

#[get("/pay_per_day?<day>&<amount>")]
fn pay_per_day(day: u32, amount: u32) -> Option<Json<AppResponse>> {
    let data = UserInputData {
        payout_day_of_month: day,
        money_amount: amount,
    };

    let res = App::run(data).ok()?;

    Some(Json(res))
}

fn main() {
    rocket::ignite().mount("/", routes![pay_per_day]).launch();
}
