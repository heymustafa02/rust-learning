use reqwest;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Coin {
    symbol: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Portfolio {
    coins: Vec<Coin>,
}

impl Portfolio {
    fn new() -> Self {
        Portfolio { coins: Vec::new() }
    }

    fn add(&mut self, symbol: String, amount: f64) {
        self.coins.push(Coin { symbol, amount });
        println!("Coin added.");
    }

    fn remove(&mut self, symbol: &str) {
        self.coins.retain(|c| c.symbol != symbol);
        println!("Coin removed.");
    }

    fn list(&self) {
        for coin in &self.coins {
            println!("{} - {}", coin.symbol, coin.amount);
        }
    }

    async fn price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=inr",
            symbol
        );

        let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
        let price = resp[symbol]["inr"].as_f64().unwrap_or(0.0);
        Ok(price)
    }

    async fn total_value(&self) {
        let mut total = 0.0;

        for coin in &self.coins {
            let price = Portfolio::price(&coin.symbol.to_lowercase()).await.unwrap_or(0.0);
            let value = price * coin.amount;
            println!("{} - {} | ₹{}", coin.symbol, coin.amount, value);
            total += value;
        }

        println!("Total Portfolio Value: ₹{}", total);
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write("portfolio.json", data).unwrap();
    }

    fn load() -> Self {
        let data = fs::read_to_string("portfolio.json").unwrap_or("".to_string());

        if data.trim().is_empty() {
            return Portfolio::new();
        }

        serde_json::from_str(&data).unwrap_or(Portfolio::new())
    }
}

#[tokio::main]
async fn main() {
    let mut portfolio = Portfolio::load();

    println!("Crypto Portfolio Tracker");
    println!("Commands:");
    println!("add <symbol> <amount>");
    println!("list");
    println!("price <symbol>");
    println!("portfolio");
    println!("remove <symbol>");
    println!("exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                if parts.len() < 3 {
                    println!("Usage: add BTC 0.5");
                    continue;
                }
                let symbol = parts[1].to_string();
                let amount: f64 = parts[2].parse().unwrap();
                portfolio.add(symbol, amount);
                portfolio.save();
            }

            "list" => {
                portfolio.list();
            }

            "price" => {
                if parts.len() < 2 {
                    println!("Usage: price BTC");
                    continue;
                }
                let symbol = parts[1].to_lowercase();
                if let Ok(p) = Portfolio::price(&symbol).await {
                    println!("Live Price of {}: ₹{}", symbol, p);
                }
            }

            "remove" => {
                if parts.len() < 2 {
                    println!("Usage: remove BTC");
                    continue;
                }
                portfolio.remove(parts[1]);
                portfolio.save();
            }

            "portfolio" => {
                portfolio.total_value().await;
            }

            "exit" => {
                portfolio.save();
                println!("Saved & exiting...");
                break;
            }

            _ => println!("Unknown command."),
        }
    }
}
