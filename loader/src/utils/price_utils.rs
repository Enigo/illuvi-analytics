use rust_decimal::prelude::*;

pub fn get_price(quantity: &String, decimals: i32) -> Decimal {
    let wei_value = Decimal::from_str(quantity).unwrap();
    wei_value / Decimal::new(10i64.pow(decimals as u32), 0)
}
