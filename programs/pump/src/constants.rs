use solana_program::pubkey;

pub const TIME_TO_REFILL_ENERGY: i64 = 60;
pub const MAX_ENERGY: u64 = 100;
pub const MAX_WOOD_PER_TREE: u64 = 100000;

pub const INITIAL_PRICE: f64 = 1_000_000.0; // Initial price of the token in SOL
pub const PRICE_SLOPE: f64 = 33_000_000_000.0; // Price increase per token sold in SOL
pub const DECIMAL: f64 = 1_000.0; // Decimal precision
pub const SCALE: f64 = 1_000_000_000.0; // Scale for division and multiplication




pub const TOKEN_CREATOR: anchor_lang::prelude::Pubkey = pubkey!("devjbkEUcKtEfw3h8nzScA4eS1tyWejcpTzNJmr46Xa");