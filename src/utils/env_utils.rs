use std::env;
use std::fmt::Debug;
use std::str::FromStr;

pub fn as_parsed<T: FromStr>(key: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    as_string(key)
        .parse::<T>()
        .expect(format!("{} should be a valid u16", key).as_str())
}

pub fn as_string(key: &str) -> String {
    String::from(
        env::var(key)
            .expect(format!("{} should be set", key).as_str())
            .as_str(),
    )
}
