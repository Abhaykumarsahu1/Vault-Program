use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_SEED, VAULT_STATE_SEED},
    state::VaultState,
};

#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close = user,
        seeds = [VAULT_STATE_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

pub fn close(ctx: Context<Close>)->Result<()>{
    //so this is closing logic we are doing
    //to drain the vault we need to know how many are there

    let amount = ctx.accounts.vault.lamports(); //returns the current balance

    let user_key = ctx.accounts.user.key();
    let signer_seeds: &[&[u8]] = &[
        VAULT_SEED,
        user_key.as_ref(),
        &[ctx.accounts.vault_state.vault_bump],
    ];

    //not transfered but preparing the channel
    let cpi_accounts = Transfer{
        from: ctx.accounts.vault.to_account_info(),
        to:ctx.accounts.user.to_account_info(),
    };

    let binding = [signer_seeds];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        cpi_accounts,
        &binding,
    );

    transfer(cpi_ctx, amount)?;
    Ok(())
}