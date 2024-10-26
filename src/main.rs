use std::{env, error};
use std::fs::{OpenOptions};
use std::io::Write;
use dotenv::dotenv;
use time::OffsetDateTime;
use crate::polygon::Polygon;

mod polygon;

#[derive(Debug)]
struct Record {
    symbol: String,
    time: OffsetDateTime,
    open: f32,
    close: f32,
    low: f32,
    height: f32,
    count: i32,
    volume: f32,
    volume_weight: f32,
}

impl Record {
    fn from(sym: String, ticker: polygon::Ticker) -> Record {
        Self {
            symbol: sym,
            close: ticker.close,
            height: ticker.height,
            low: ticker.low,
            count: ticker.count,
            open: ticker.open,
            time: ticker.time,
            volume: ticker.volume,
            volume_weight: ticker.volume_weight,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let filename = env::var("OUTPUT_FILENAME")?;
    let api_key = env::var("API_KEY")?;
    let start = env::var("START")?;
    let end = env::var("END")?;

    let mut records: Vec<Record> = vec![];

    let symbols = env::var("SYMBOLS")?;
    for sym in symbols.split(",").collect::<Vec<&str>>() {
        println!("{}", sym);
        let record_or_error = get_ticker(api_key.to_owned(), sym.to_string(), start.to_owned(), end.to_owned()).await;
        if record_or_error.is_err() {
            println!("Error getting symbol: {}\n{}", sym, record_or_error.unwrap_err());
            continue;
        }

        let record_from_symbol = record_or_error.unwrap_or_else(|error| {
            println!("error: {} | {:?}", sym, error);
            return vec![]
        });

        records.extend(record_from_symbol);
    }

    let ok_or_error = write_csv(filename, records);
    if ok_or_error.is_err() {
        println!("Error writing csv\n{}", ok_or_error.unwrap_err());
    }
    Ok(())
}

async fn get_ticker(api_key: String, symbol: String, start: String, end: String) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let polygon = Polygon::new(api_key.as_str());
    let ticker_response = polygon.ticker(symbol.as_str(), start.as_str(), end.as_str()).await?;
    if ticker_response.error.is_some() {
        return Err(ticker_response.error.unwrap().into())
    }

    let mut records: Vec<Record> = vec![];
    for tick in ticker_response.results.unwrap_or(vec![]) {
        records.push(Record::from(symbol.to_owned(), tick));
    }

    Ok(records)
}

fn write_csv(filename: String, records: Vec<Record>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .append(false)
        .write(true)
        .create(true)
        .open(filename)?;

    writeln!(file, "symbol,date,open,close,low,height,count,volume,volume_weight")?;

    for record in records {
        let line = format!("{},{},{},{},{},{},{},{},{}",
                           record.symbol,
                           record.time.date().to_string(),
                           record.open,
                           record.close,
                           record.low,
                           record.height,
                           record.count,
                           record.volume,
                           record.volume_weight);
        let error = writeln!(file, "{}", line);
        if error.is_err() {
            println!("error: {:?}", error);
        }
    }

    Ok(())
}