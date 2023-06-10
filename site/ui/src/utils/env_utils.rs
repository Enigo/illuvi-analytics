use dotenvy_macro::dotenv;

pub fn get_api_endpoint() -> String {
    // loaded at compile time
    let backend_url = dotenv!("BACKEND_ENDPOINT");
    String::from(backend_url)
}
