mod money_per_day;

use crate::money_per_day::app::App;
use crate::money_per_day::data_access;
use crate::money_per_day::locale;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = data_access::from_stdin()?;

    let app = App::<locale::Polish>::from_data(data);

    let res = app.run()?;

    app.print_response(res);
    Ok(())
}
