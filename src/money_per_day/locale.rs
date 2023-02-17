pub trait Lang {
    fn how_many_days_left(n: u32) -> String;
    fn how_much_money_per_day(n: f32) -> String;
}

pub struct English;
impl Lang for English {
    fn how_many_days_left(n: u32) -> String {
        format!("{n} days are left for payday.")
    }

    fn how_much_money_per_day(n: f32) -> String {
        format!("You have {n:2}$ per day to spend.")
    }
}

pub struct Polish;
impl Lang for Polish {
    fn how_many_days_left(n: u32) -> String {
        let days_declension = if n != 1 { "dni" } else { "dzień" };

        format!("Do wypłaty pozostało {n} {days_declension}.")
    }

    fn how_much_money_per_day(n: f32) -> String {
        format!("Masz do wydania {n:2}zł.")
    }
}
