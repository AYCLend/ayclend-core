use anchor_lang::{prelude::*, Discriminator};
use solana_program::{
    instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT},
    sysvar::{self, instructions},
};

use crate::{
    check,
    prelude::*,
    state::ayclend_account::{AyclendAccount, RiskEngine, DISABLED_FLAG, IN_FLASHLOAN_FLAG},
};

pub fn lending_account_start_flashloan(
    ctx: Context<LendingAccountStartFlashloan>,
    end_index: u64,
) -> AyclendResult<()> {
    check_flashloan_can_start(
        &ctx.accounts.ayclend_account,
        &ctx.accounts.ixs_sysvar,
        end_index as usize,
    )?;

    let mut ayclend_account = ctx.accounts.ayclend_account.load_mut()?;
    ayclend_account.set_flag(IN_FLASHLOAN_FLAG);

    Ok(())
}

#[derive(Accounts)]
pub struct LendingAccountStartFlashloan<'info> {
    #[account(mut)]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,
    #[account(address = ayclend_account.load()?.authority)]
    pub signer: Signer<'info>,
    /// CHECK: Instructions sysvar
    #[account(address = sysvar::instructions::ID)]
    pub ixs_sysvar: AccountInfo<'info>,
}

const END_FL_IX_MARGINFI_ACCOUNT_AI_IDX: usize = 0;

/// Checklist
/// 1. `end_flashloan` ix index is after `start_flashloan` ix index
/// 2. Ixs has an `end_flashloan` ix present
/// 3. `end_flashloan` ix is for the ayclend program
/// 3. `end_flashloan` ix is for the same ayclend account
/// 4. Account is not disabled
/// 5. Account is not already in a flashloan
/// 6. Start flashloan ix is not in CPI
/// 7. End flashloan ix is not in CPI
pub fn check_flashloan_can_start(
    ayclend_account: &AccountLoader<AyclendAccount>,
    sysvar_ixs: &AccountInfo,
    end_fl_idx: usize,
) -> AyclendResult<()> {
    // Note: FLASHLOAN_ENABLED_FLAG is now deprecated.
    // Any non-disabled account can initiate a flash loan.
    check!(
        !ayclend_account.load()?.get_flag(DISABLED_FLAG),
        AyclendError::AccountDisabled
    );

    let current_ix_idx: usize = instructions::load_current_index_checked(sysvar_ixs)?.into();

    check!(current_ix_idx < end_fl_idx, AyclendError::IllegalFlashloan);

    // Check current ix is not a CPI
    let current_ix = instructions::load_instruction_at_checked(current_ix_idx, sysvar_ixs)?;

    check!(
        get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT,
        AyclendError::IllegalFlashloan,
        "Start flashloan ix should not be in CPI"
    );

    check!(
        current_ix.program_id.eq(&crate::id()),
        AyclendError::IllegalFlashloan,
        "Start flashloan ix should not be in CPI"
    );

    // Will error if ix doesn't exist
    let unchecked_end_fl_ix = instructions::load_instruction_at_checked(end_fl_idx, sysvar_ixs)?;

    check!(
        unchecked_end_fl_ix.data[..8]
            .eq(&crate::instruction::LendingAccountEndFlashloan::DISCRIMINATOR),
        AyclendError::IllegalFlashloan
    );

    check!(
        unchecked_end_fl_ix.program_id.eq(&crate::id()),
        AyclendError::IllegalFlashloan
    );

    let end_fl_ix = unchecked_end_fl_ix;

    let end_fl_ayclend_account = end_fl_ix
        .accounts
        .get(END_FL_IX_MARGINFI_ACCOUNT_AI_IDX)
        .ok_or(AyclendError::IllegalFlashloan)?;

    check!(
        end_fl_ayclend_account.pubkey.eq(&ayclend_account.key()),
        AyclendError::IllegalFlashloan
    );

    let marginf_account = ayclend_account.load()?;

    check!(
        !marginf_account.get_flag(DISABLED_FLAG),
        AyclendError::AccountDisabled
    );

    check!(
        !marginf_account.get_flag(IN_FLASHLOAN_FLAG),
        AyclendError::IllegalFlashloan
    );

    Ok(())
}

pub fn lending_account_end_flashloan<'info>(
    ctx: Context<'_, '_, 'info, 'info, LendingAccountEndFlashloan<'info>>,
) -> AyclendResult<()> {
    check!(
        get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT,
        AyclendError::IllegalFlashloan,
        "End flashloan ix should not be in CPI"
    );

    let mut ayclend_account = ctx.accounts.ayclend_account.load_mut()?;

    ayclend_account.unset_flag(IN_FLASHLOAN_FLAG);

    RiskEngine::check_account_init_health(&ayclend_account, ctx.remaining_accounts)?;

    Ok(())
}

#[derive(Accounts)]
pub struct LendingAccountEndFlashloan<'info> {
    #[account(mut)]
    pub ayclend_account: AccountLoader<'info, AyclendAccount>,
    #[account(address = ayclend_account.load()?.authority)]
    pub signer: Signer<'info>,
}
