pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("9EWA6r8FyQGX6X9eGvj74jsnM21VyGvsuWWvvrJBDVLa");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        instructions::close(ctx)
    }
  
}
