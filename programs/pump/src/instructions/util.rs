use crate::constants::{INITIAL_PRICE, PRICE_SLOPE, SCALE};

// Function to calculate the price of tokens based on the current token supply
pub fn calculate_token_price(current_supply: f64) -> f64 {
    // Convert INITIAL_PRICE to f64 before multiplication to avoid overflow
    let initial_price = INITIAL_PRICE;
    // Multiply by SCALE before division to maintain precision
    let price_slope_scaled = (current_supply * PRICE_SLOPE) / SCALE;
    return initial_price * SCALE + price_slope_scaled;
}

// Function to calculate SOL needed to buy x amount of tokens
pub fn calculate_sol_needed(current_supply: f64, desired_token_amount: f64, decimal:u64) -> u64 {
    let token_price = calculate_token_price(current_supply);
    // Multiply by SCALE before division to maintain precision
    return ((desired_token_amount * token_price / SCALE).ceil()/ 10u64.pow(decimal as u32) as f64) as u64;
}

// Function to calculate tokens bought with x amount of SOL
pub fn calculate_tokens_bought(current_supply: f64, sol_sent: f64, decimal:u64) -> u64 {
    let token_price = calculate_token_price(current_supply);
    // Multiply by SCALE before division to maintain precision
   return ((sol_sent * SCALE / token_price) * 10u64.pow(decimal as u32) as f64) as u64;
}
