use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SwapInfo {
    pub label: String,

    #[serde(rename = "inAmount")]
    pub in_amount: String,

    #[serde(rename = "outAmount")]
    pub out_amount: String,
}

#[derive(Debug, Deserialize)]
pub struct RoutePlan {
    pub percent: u32,

    #[serde(rename = "swapInfo")]
    pub swap_info: SwapInfo,
}

#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "inAmount")]
    pub in_amount: String,

    #[serde(rename = "outAmount")]
    pub out_amount: String,

    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,

    #[serde(rename = "routePlan")]
    pub route_plan: Vec<RoutePlan>,

    #[serde(rename = "inUsdValue")]
    pub swap_usd_value: Option<String>,
}

pub async fn jup_price(
    client: &reqwest::Client,
    inputMint: &str,
    outputMint: &str,
    amount: u64,
    dex: &str,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let api_key = std::env::var("JUP_API_KEY")?;
    let url = format!(
        "https://api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}&dexes={}",
        inputMint, outputMint, amount, dex,
    );

    let response = client.get(url).header("x-api-key", api_key).send().await?;

    let quote: QuoteResponse = response.json().await?;

    println!("{:#?}", quote);

    let sol_price_in_usdc = quote.out_amount.parse::<f64>()? / 1_000_000.0;

    println!("1 SOL = {:6} USDC", sol_price_in_usdc);
    Ok(quote)
}
