use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub status: String,
    pub result: Option<Vec<TheResult>>,
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub hash: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "isError")]
    pub is_error: String,
    pub input: String,
    #[serde(rename = "methodId")]
    pub method_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
}
