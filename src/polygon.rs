use reqwest::Error;
use serde::Deserialize;
use time::OffsetDateTime;

pub struct Polygon {
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerResponse {
    pub results: Vec<Ticker>,
}
#[derive(Debug, Deserialize)]
pub struct Ticker {
    #[serde(rename = "c")]
    pub close: f32,
    #[serde(rename = "h")]
    pub height: f32,
    #[serde(rename = "l")]
    pub low: f32,
    #[serde(rename = "n")]
    pub count: i32,
    #[serde(rename = "o")]
    pub open: f32,
    #[serde(rename = "t", with = "time::serde::timestamp::milliseconds")]
    pub time: OffsetDateTime,
    #[serde(rename = "v")]
    pub volume: f32,
    #[serde(rename = "vw")]
    pub volume_weight: f32,
}

impl Polygon {
    pub fn new(api_key: &str) -> Polygon {
        Polygon { api_key: api_key.to_string() }
    }

    pub async fn ticker(&self, symbol: &str, start: &str, end: &str) -> Result<TickerResponse, Error> {
        // https://api.polygon.io/v2/aggs/ticker/AAPL/range/1/day/2023-01-09/2023-02-10?adjusted=true&sort=asc&apiKey=
        let url = format!("https://api.polygon.io/v2/aggs/ticker/{}/range/1/day/{}/{}?adjusted=true&sort=asc&apiKey={}", symbol, start, end, self.api_key);
        let resp = reqwest::get(url).await?;
        Ok(resp.json::<TickerResponse>().await?)
    }
}