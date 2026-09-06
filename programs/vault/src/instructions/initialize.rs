use anchor_lang::{
    prelude::*;
    system_program::{transfer, Transfer},
};
use crate::{
    constants::{VAULT_STATE_SEED, VAULT_SEED},
    state::VaultState
};

#[derive(Accounts)]
pub struct Initialize<'info>{
    //user will be intialized as signer to sign the transaction,also will be modified as they will pay for creation and all stuff
    #[account(mut)]
    pub user: Signer<'info>,

    //vault_state will have the data
    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_LENGTH + VaultState::INIT_SPACE,//init space is basically how many bytes  i m intializing
        seed = [VAULT_STATE_SEED, user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    //vault will be the system account that will be more like pda
    #[account(
        mut,
        seed = [VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>, 
}


pub fn initialize(ctx : Context<Initialize>)->Result<()>{
    msg!("intializing vault for user: {}",ctx.accounts.user.key());
    
    //preparing user and vault for system program transfer
    let cpi_accounts = anchor_lang::system_program::Transfer{
        from: ctx.accounts.user.to_account_info(),
        to:ctx.accounts.vault.to_account_info(),
    };

    //here we are preparing the cpi calls for the transfer
    let cpi_ctx = CpiContext::new(
        ctx.account.system_program.key(),
        cpi_accounts,
    );

    let rent = Rent::get()?.minimum_balance(ctx.accounts.vault.data_len());

    //cpi_ctx tells the sysprogram which accounts are involved. then we are basically here transfering the rent to make the vault onchain
    transfer(cpi_ctx,rent)?;

    ctx.accounts.vault_state.set_inner(VaultState {
    vault_bump: ctx.bumps.vault,
    bump: ctx.bumps.vault_state,
});

    Ok(());
}
