use anchor_lang::prelude::*;
use instructions::*;

mod state;
mod instructions;
mod error;
mod constants;

// some key eg of defi include decentralized exchanges, stable coins and lending protocols
// decentralized exchanges allow users to trade assets without a central authority
// 2 eg automated market maker and an orderbook, orderbook require matching buyers and sellers
// while AMM use price impact to determine the price of an asset algorithmically

// stable coins are assets that are pegged to a stable value, such as the US dollar
// they are used to provide price stability and reduce volatility in the market

// lending protocols allow users to borrow assets from a pool of lenders
// interest rates are determined by supply and demand

// we are going to make the accounts overcollaterized and calculate the health factor
// if health factor degrades below a certain threshold, the user can be liquidated
// liquidation process involves seizing the user's collateral and repaying the loan
// liquidation bonus is given to the liquidator

declare_id!("CdZeD33fXsAHfZYS8jdxg4qHgXYJwBQ1Bv6GJyETtLST");

#[program]
pub mod lending_protocol {

    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, liquidation_threshold: u64, max_ltv: u64) -> Result<()> {
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }

    pub fn deposit (ctx: Context<Deposit>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }

    pub fn withdraw (ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        process_withdraw(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        process_borrow(ctx, amount)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        process_repay(ctx, amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        process_liquidate(ctx)
    }
}

