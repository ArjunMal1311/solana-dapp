use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // calculate space required for the account (bank)
pub struct Bank {
    pub authority: Pubkey,
    pub mint_address: Pubkey,
    pub total_deposits: u64,
    pub total_deposit_shares: u64,
    pub total_borrowed: u64,
    pub total_borrowed_shares: u64,
    pub liquidation_threshold: u64, // LTV at which the loan is defined as under collateralized and can be liquidated
    pub liquidation_bonus: u64, // Bonus percentage of collateral that can be liquidated
    pub liquidation_close_factor: u64, // Percentage of collateral that can be liquidated
    pub max_ltv: u64, // Max percentage of collateral that can be borrowed
    pub last_updated: i64,
    pub interest_rate: u64,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey,
    pub deposited_sol: u64,
    pub deposited_sol_shares: u64,
    pub borrowed_sol: u64,
    pub borrowed_sol_shares: u64,
    pub deposited_usdc: u64,
    pub deposited_usdc_shares: u64,
    pub borrowed_usdc: u64,
    pub borrowed_usdc_shares: u64,
    pub usdc_address: Pubkey,
    pub health_factor: u64,
    pub last_updated: i64,
}
