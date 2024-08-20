use anchor_lang::prelude::*;

use crate::{
    check,
    prelude::*,
    state::{
        ayclend_account::{BankAccountWrapper, AyclendAccount, DISABLED_FLAG},
        ayclend_group::Bank,
    },
};

pub fn lending_account_close_balance(ctx: Context<LendingAccountCloseBalance>) -> AyclendResult {
    let LendingAccountCloseBalance {
        ayclend_account,
        bank: bank_loader,
        ..
    } = ctx.accounts;

    let mut ayclend_account = ayclend_account.load_mut()?;
    let mut bank = bank_loader.load_mut()?;

    check!(
        !ayclend_account.get_flag(DISABLED_FLAG),
        AyclendError::AccountDisabled
    );

    bank.accrue_interest(
        Clock::get()?.unix_timestamp,
        #[cfg(not(feature = "client"))]
        bank_loader.key(),
    )?;

    let mut bank_account = BankAccountWrapper::find(
        &bank_loader.key(),
        &mut bank,
        &mut ayclend_account.lending_account,
    )?;

    bank_account.close_balance()?;

    Ok(())
}

#[derive(Accounts)]
pub struct LendingAccountCloseBalance<'info> {
    pub ayclend_group: AccountLoader<'info, AyclendGroup>,

    #[account(
        mut,
        constraint = ayclend_account.load()?.group == ayclend_group.key(),
    )]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,

    #[account(
        address = ayclend_account.load()?.authority,
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = bank.load()?.group == ayclend_group.key(),
    )]
    pub bank: AccountLoader<'info, Bank>,
}
