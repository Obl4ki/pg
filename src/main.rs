mod money_per_day;

use crate::money_per_day::app::App;
use crate::money_per_day::locale;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let res = App::run()?;

    App::print_response::<locale::English>(res);
    Ok(())
}
