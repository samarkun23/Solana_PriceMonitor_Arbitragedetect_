use crate::price::calculate_profit::calculate_net_profit;

mod price;

use chrono::Utc;
use std::fs::OpenOptions;
use std::time::Duration;

async fn check_opportunity(
    client: &reqwest::Client,
    input_mint: &str,
    output_mint: &str,
    amount: u64,
    csv_writer: &mut csv::Writer<std::fs::File>,
) -> Result<(), Box<dyn std::error::Error>> {
    let raydium_quote =
        price::jup_price(client, input_mint, output_mint, amount, "Raydium").await?;
    let orca_quote = price::jup_price(client, input_mint, output_mint, amount, "Orca+V2").await?;

    let raydium_usdc = raydium_quote.out_amount.parse::<f64>()? / 1_000_000.0;
    let orca_usdc = orca_quote.out_amount.parse::<f64>()? / 1_000_000.0;

    let raydium_impact = raydium_quote.price_impact_pct.parse::<f64>()?;
    let orca_impact = orca_quote.price_impact_pct.parse::<f64>()?;

    let sol_price_usd = (raydium_usdc + orca_usdc) / 2.0;

    let net_profit = calculate_net_profit(
        raydium_usdc,
        orca_usdc,
        raydium_impact,
        orca_impact,
        sol_price_usd,
    );

    let timestamp = Utc::now().to_rfc3339();
    let is_profitable = net_profit > 0.0;

    println!(
        "[{}] Raydium: {:.4} | Orca: {:.4} | Net profit: {:.6} | Profitable: {}",
        timestamp, raydium_usdc, orca_usdc, net_profit, is_profitable
    );

    csv_writer.write_record(&[
        timestamp,
        raydium_usdc.to_string(),
        orca_usdc.to_string(),
        raydium_impact.to_string(),
        orca_impact.to_string(),
        net_profit.to_string(),
        is_profitable.to_string(),
    ])?;
    csv_writer.flush()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let sol = "So11111111111111111111111111111111111111112";
    let usdc = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let amount = 1_000_000_000;

    let file_exists = std::path::Path::new("opportunities.csv").exists();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("opportunities.csv")?;
    let mut csv_writer = csv::Writer::from_writer(file);

    if !file_exists {
        csv_writer.write_record(&[
            "timestamp",
            "raydium_price",
            "orca_price",
            "raydium_impact",
            "orca_impact",
            "net_profit",
            "is_profitable",
        ])?;
        csv_writer.flush()?;
    }

    let mut interval = tokio::time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        if let Err(e) = check_opportunity(&client, sol, usdc, amount, &mut csv_writer).await {
            eprintln!("Error: {}", e);
        }
    }
}
