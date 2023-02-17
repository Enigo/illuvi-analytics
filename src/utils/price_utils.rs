pub fn get_price(quantity: &String, decimals: i32) -> f32 {
    let index_of_comma = quantity.chars().count() as i32 - decimals;

    return match index_of_comma {
        -1 => (format!("{}{}", "0.0", quantity)).parse().unwrap(),
        0 => (format!("{}{}", "0.", quantity)).parse().unwrap(),
        _ => {
            let mut quantity_clone = quantity.clone();
            quantity_clone.insert(index_of_comma as usize, '.');
            quantity_clone.parse().unwrap()
        }
    };
}
