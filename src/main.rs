use std::collections::HashMap;
use std::env;
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
    fn from(sym: &str, ticker: polygon::Ticker) -> Record {
        Self {
            symbol: sym.to_string(),
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

    let polygon = Polygon::new(env::var("API_KEY")?.as_str());
    let mut records: Vec<Record> = vec![];

    let symbols = "NVDA,TSM";
    for sym in symbols.split(",").collect::<Vec<&str>>() {
        let ticker = polygon.ticker(sym, "2024-01-01", "2024-09-01").await?;
        for tick in ticker.results {
            records.push(Record::from(sym, tick));
        }
    }

    print_csv(records);
    Ok(())
}

fn print_csv(records: Vec<Record>) {
    println!("symbol,date,open,close,low,height,count,volume,volume_weight");
    for record in records {
        println!("{},{},{},{},{},{},{},{},{}",
                 record.symbol,
                 record.time.date().to_string(),
                 record.open,
                 record.close,
                 record.low,
                 record.height,
                 record.count,
                 record.volume,
                 record.volume_weight);
    }
}