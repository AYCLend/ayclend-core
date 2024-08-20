use crate::{prelude::*, state::ayclend_account::AyclendAccount};
use anchor_lang::prelude::*;

pub fn set_account_transfer_authority(
    ctx: Context<AyclendAccountSetAccountAuthority>,
) -> AyclendResult {
    // Ensure ayclend_account is dropped out of scope to not exceed stack frame limits
    {
        let mut ayclend_account = ctx.accounts.ayclend_account.load_mut()?;
        let new_account_authority = ctx.accounts.new_authority.key();
        ayclend_account.set_new_account_authority_checked(new_account_authority)?;
    }

    // TODO: add back event (dropped for memory reasons)

    Ok(())
}

#[derive(Accounts)]
pub struct AyclendAccountSetAccountAuthority<'info> {
    #[account(mut)]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,

    /// CHECK: The group is confirmed by the address macro
    #[account(
        address = ayclend_account.load()?.group,
    )]
    pub ayclend_group: AccountInfo<'info>,

    #[account(
        address = ayclend_account.load()?.authority,
    )]
    pub signer: Signer<'info>,

    /// CHECK: The new account authority doesn't need explicit checks
    pub new_authority: AccountInfo<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,
}
