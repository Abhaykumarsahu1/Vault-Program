use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_SEED,VAULT_STATE_SEED},
    state::VaultState,
};

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    //now lets add vault (this gonna be interesting hehee)
    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(
        seeds = [VAULT_STATE_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>, //we are bringing vaultstate as it contains vault.bump

    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount:u64)->Result<()>{

    //preparing the transfer now:
    let cpi_accounts = Transfer{
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };

    //now we will create the signer seeds as vault does not have this privatekey
    let user_key = ctx.accounts.user.key();
    let signer_seeds: &[&[u8]] = &[
        VAULT_SEED,
        user_key.as_ref(),
        &[ctx.accounts.vault_state.vault_bump],
    ];

    let binding = [signer_seeds];
    let cpi_ctx = CpiContext::new_with_signer(//new_with_signer user here coz vault is signing here as pda with no private key so that's why we are using new_with_signer and passing the signer seeds for it to be able to verify and sign
        ctx.accounts.system_program.key(),
        cpi_accounts,
        &binding,
    );

    transfer(cpi_ctx, amount)?;

    Ok(())
}