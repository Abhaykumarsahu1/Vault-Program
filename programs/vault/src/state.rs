use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump:u8, //this is the bump for vault pda
    pub bump: u8, //this is the bump for vaultstate pda
}
