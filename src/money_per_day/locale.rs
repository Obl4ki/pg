pub trait Lang {
    fn input_payday_number() -> String;
    fn input_account_balance() -> String;
    fn how_many_days_left(n: u32) -> String;
    fn how_much_money_per_day(n: f32) -> String;
}

pub struct English;
impl Lang for English {
    fn input_payday_number() -> String {
        "Input day of your payday (1-31):".to_string()
    }

    fn input_account_balance() -> String {
        "Input your account balance:".to_string()
    }

    fn how_many_days_left(n: u32) -> String {
        format!("{n} days are left for payday.")
    }

    fn how_much_money_per_day(n: f32) -> String {
        format!("You have {n:2}$ per day to spend.")
    }
}

pub struct Polish;
impl Lang for Polish {
    fn input_payday_number() -> String {
        "Wprowadź dzień wypłaty (1-31)".to_string()
    }

    fn input_account_balance() -> String {
        "Wprowadź swój stan konta".to_string()
    }

    fn how_many_days_left(n: u32) -> String {
        let days_declension = if n != 1 { "dni" } else { "dzień" };

        format!("Do wypłaty pozostało {n} {days_declension}.")
    }

    fn how_much_money_per_day(n: f32) -> String {
        format!("Masz do wydania {n:2}zł.")
    }
}
