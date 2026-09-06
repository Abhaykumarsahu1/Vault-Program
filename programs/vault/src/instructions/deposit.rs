use anchor::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_SEED, VAULT_STATE_SEED},
    state::VaultState,
}

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,

    //vaultstae we do not require here
}

pub fn deposit(ctx: Context<Deposit>, amount:u64)->Result<()>{

    let cpi_accounts = Transfer{
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.key(),
        cpi_accounts,
    );

    Transfer(cpi_ctx, amount)?; //if the transfer fails then error will propogate 

    Ok(())

}

