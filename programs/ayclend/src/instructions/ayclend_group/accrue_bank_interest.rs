use crate::{
    state::ayclend_group::{Bank, AyclendGroup},
    AyclendResult,
};
use anchor_lang::prelude::*;

pub fn lending_pool_accrue_bank_interest(
    ctx: Context<LendingPoolAccrueBankInterest>,
) -> AyclendResult {
    let clock = Clock::get()?;
    let mut bank = ctx.accounts.bank.load_mut()?;

    bank.accrue_interest(
        clock.unix_timestamp,
        #[cfg(not(feature = "client"))]
        ctx.accounts.bank.key(),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct LendingPoolAccrueBankInterest<'info> {
    pub ayclend_group: AccountLoader<'info, AyclendGroup>,

    #[account(
        mut,
        constraint = bank.load()?.group == ayclend_group.key(),
    )]
    pub bank: AccountLoader<'info, Bank>,
}
