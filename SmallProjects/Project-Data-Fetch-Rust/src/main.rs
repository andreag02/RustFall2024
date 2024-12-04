use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use ureq;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct BitcoinPrice {
    bitcoin: BitcoinCurrency,
}

#[derive(Debug, Deserialize)]
struct BitcoinCurrency {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct EthereumPrice {
    ethereum: EthereumCurrency,
}

#[derive(Debug, Deserialize)]
struct EthereumCurrency {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct SP500 {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct SP500Response {
    chart: ChartData,
}

#[derive(Debug, Deserialize)]
struct ChartData {
    result: Vec<SP500Result>,
}

#[derive(Debug, Deserialize)]
struct SP500Result {
    indicators: Indicators,
}

#[derive(Debug, Deserialize)]
struct Indicators {
    quote: Vec<QuoteData>,
}

#[derive(Debug, Deserialize)]
struct QuoteData {
    close: Option<Vec<Option<f64>>>,
}

trait Pricing {
    fn fetch_price(&self) -> f64;
    fn save_to_file(&self, price: f64);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let json: BitcoinPrice = response.into_json().unwrap();
        json.bitcoin.usd
    }

    fn save_to_file(&self, price: f64) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .unwrap();
        writeln!(file, "Bitcoin: ${}", price).unwrap();
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let json: EthereumPrice = response.into_json().unwrap();
        json.ethereum.usd
    }

    fn save_to_file(&self, price: f64) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .unwrap();
        writeln!(file, "Ethereum: ${}", price).unwrap();
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let json: SP500Response = response.into_json().unwrap();

        if let Some(close_prices) = json.chart.result.first()
            .and_then(|r| r.indicators.quote.first())
            .and_then(|q| q.close.as_ref()) {
            
            let last_price = close_prices.last()
                .and_then(|&price| price)
                .unwrap_or(0.0);
            last_price
        } else {
            0.0
        }
    }

    fn save_to_file(&self, price: f64) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .unwrap();
        writeln!(file, "S&P 500: ${}", price).unwrap();
    }
}

fn main() {
    let bitcoin = Bitcoin {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
            .to_string(),
        file_name: "bitcoin.txt".to_string(),
    };
    let ethereum = Ethereum {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd"
            .to_string(),
        file_name: "ethereum.txt".to_string(),
    };
    let sp500 = SP500 {
        api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d"
            .to_string(),
        file_name: "sp500.txt".to_string(),
    };

    let assets: Vec<(&str, &dyn Pricing)> = vec![
        ("Bitcoin", &bitcoin),
        ("Ethereum", &ethereum),
        ("SP500", &sp500),
    ];

    loop {
        for (name, asset) in &assets {
            let price = asset.fetch_price();
            asset.save_to_file(price);
            println!("{} price saved: ${}", name, price);
        }
        thread::sleep(Duration::from_secs(10));
    }
}
