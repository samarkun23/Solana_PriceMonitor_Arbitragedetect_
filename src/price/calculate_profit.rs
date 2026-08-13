pub fn calculate_net_profit(
    radium_out: f64,
    orca_out: f64,
    radium_price_impact_pct: f64,
    orca_price_impact_pact: f64,
    sol_price_usdc: f64,
) -> f64 {
    let gross_diff = (radium_out - orca_out).abs();

    let impact_cost = (radium_price_impact_pct.abs() + orca_price_impact_pact.abs()) * radium_out;

    // trx fee
    let tx_fee_sol = 0.000005 * 2.0;
    let tx_fee_usdc = tx_fee_sol * sol_price_usdc;

    // fee estimation TODO: we need to fetch real value
    let priority_fee_sol = 0.0005;
    let priority_fee_usd = priority_fee_sol * sol_price_usdc;

    let net_profit = gross_diff - impact_cost - tx_fee_usdc - priority_fee_usd;
    net_profit
}
