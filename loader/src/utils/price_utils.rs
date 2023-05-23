use rust_decimal::prelude::*;

pub fn get_price(quantity: &String, decimals: i32) -> f32 {
    let wei_value = Decimal::from_str(quantity).unwrap();
    let ether_value = wei_value / Decimal::new(10i64.pow(decimals as u32), 0);
    ether_value.to_f32().unwrap()
}
