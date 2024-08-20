use crate::{
    events::{AccountEventHeader, AyclendAccountCreateEvent},
    prelude::*,
    state::ayclend_account::AyclendAccount,
};
use anchor_lang::prelude::*;
use solana_program::sysvar::Sysvar;

pub fn initialize_account(ctx: Context<AyclendAccountInitialize>) -> AyclendResult {
    let AyclendAccountInitialize {
        authority,
        ayclend_group,
        ayclend_account: ayclend_account_loader,
        ..
    } = ctx.accounts;

    let mut ayclend_account = ayclend_account_loader.load_init()?;

    ayclend_account.initialize(ayclend_group.key(), authority.key());

    emit!(AyclendAccountCreateEvent {
        header: AccountEventHeader {
            signer: Some(authority.key()),
            ayclend_account: ayclend_account_loader.key(),
            ayclend_account_authority: ayclend_account.authority,
            ayclend_group: ayclend_account.group,
        }
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AyclendAccountInitialize<'info> {
    pub ayclend_group: AccountLoader<'info, AyclendGroup>,

    #[account(
        init,
        payer = fee_payer,
        space = 8 + std::mem::size_of::<AyclendAccount>()
    )]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
