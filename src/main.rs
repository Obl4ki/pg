mod money_per_day;

use crate::money_per_day::app::App;
use crate::money_per_day::data_access::from_stdin;
use crate::money_per_day::locale;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = from_stdin::<locale::English>()?;
    let res = App::run(data)?;
    res.print::<locale::English>();
    Ok(())
}
