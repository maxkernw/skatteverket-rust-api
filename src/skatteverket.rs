use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub results: Vec<TestPerson>,
}

#[derive(Debug, Deserialize)]
pub struct TestPerson {
    pub testpersonnummer: String,
}

pub async fn fetch_data_from_api(year: String, amount: String) -> Result<ApiResponse> {
    // Construct the API URL
    let url = format!(
        "https://skatteverket.entryscape.net/rowstore/dataset/b4de7df7-63c0-4e7e-bb59-1f156a591763?testpersonnummer={}.*&_limit={}&_offset=0",
        year, amount
    );

    let client = Client::new();

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?;
    if response.status().is_success() {
        let api_response = response.json::<ApiResponse>().await?;
        Ok(api_response)
    } else {
        Err(anyhow!(
            "API request failed with status code: {}",
            response.status()
        ))
    }
}
