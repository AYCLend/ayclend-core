use anchor_lang::prelude::*;

use crate::{check, state::ayclend_account::AyclendAccount, AyclendError, AyclendResult};

pub fn close_account(ctx: Context<AyclendAccountClose>) -> AyclendResult {
    let ayclend_account = &ctx.accounts.ayclend_account.load()?;

    check!(
        ayclend_account.can_be_closed(),
        AyclendError::IllegalAction,
        "Account cannot be closed"
    );

    Ok(())
}

#[derive(Accounts)]
pub struct AyclendAccountClose<'info> {
    #[account(mut, close = fee_payer)]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,
    #[account(address = ayclend_account.load()?.authority)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
}
