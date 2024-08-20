pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod macros;
pub mod prelude;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;
use prelude::*;
use state::ayclend_group::{BankConfigCompact, BankConfigOpt};

cfg_if::cfg_if! {
    if #[cfg(feature = "mainnet-beta")] {
        declare_id!("MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA");
    } else if #[cfg(feature = "devnet")] {
        declare_id!("B8MRNdMjeRE19EbpUL9BdAmFDSvxmfyBBcnfdEKB17cK");
    } else if #[cfg(feature = "staging")] {
        declare_id!("stag8sTKds2h4KzjUw3zKTsxbqvT4XKHdaR9X9E6Rct");
    } else {
        declare_id!("Mfi1111111111111111111111111111111111111111");
    }
}

#[program]
pub mod ayclend {
    use super::*;

    pub fn ayclend_group_initialize(ctx: Context<AyclendGroupInitialize>) -> AyclendResult {
        ayclend_group::initialize_group(ctx)
    }

    pub fn ayclend_group_configure(
        ctx: Context<AyclendGroupConfigure>,
        config: GroupConfig,
    ) -> AyclendResult {
        ayclend_group::configure(ctx, config)
    }

    pub fn lending_pool_add_bank(
        ctx: Context<LendingPoolAddBank>,
        bank_config: BankConfigCompact,
    ) -> AyclendResult {
        ayclend_group::lending_pool_add_bank(ctx, bank_config.into())
    }

    /// A copy of lending_pool_add_bank with an additional bank seed.
    /// This seed is used to create a PDA for the bank's signature.
    /// lending_pool_add_bank is preserved for backwards compatibility.
    pub fn lending_pool_add_bank_with_seed(
        ctx: Context<LendingPoolAddBankWithSeed>,
        bank_config: BankConfigCompact,
        bank_seed: u64,
    ) -> AyclendResult {
        ayclend_group::lending_pool_add_bank_with_seed(ctx, bank_config.into(), bank_seed)
    }

    pub fn lending_pool_configure_bank(
        ctx: Context<LendingPoolConfigureBank>,
        bank_config_opt: BankConfigOpt,
    ) -> AyclendResult {
        ayclend_group::lending_pool_configure_bank(ctx, bank_config_opt)
    }

    pub fn lending_pool_setup_emissions(
        ctx: Context<LendingPoolSetupEmissions>,
        flags: u64,
        rate: u64,
        total_emissions: u64,
    ) -> AyclendResult {
        ayclend_group::lending_pool_setup_emissions(ctx, flags, rate, total_emissions)
    }

    pub fn lending_pool_update_emissions_parameters(
        ctx: Context<LendingPoolUpdateEmissionsParameters>,
        emissions_flags: Option<u64>,
        emissions_rate: Option<u64>,
        additional_emissions: Option<u64>,
    ) -> AyclendResult {
        ayclend_group::lending_pool_update_emissions_parameters(
            ctx,
            emissions_flags,
            emissions_rate,
            additional_emissions,
        )
    }

    /// Handle bad debt of a bankrupt ayclend account for a given bank.
    pub fn lending_pool_handle_bankruptcy<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingPoolHandleBankruptcy<'info>>,
    ) -> AyclendResult {
        ayclend_group::lending_pool_handle_bankruptcy(ctx)
    }

    // User instructions

    /// Initialize a ayclend account for a given group
    pub fn ayclend_account_initialize(ctx: Context<AyclendAccountInitialize>) -> AyclendResult {
        ayclend_account::initialize_account(ctx)
    }

    pub fn lending_account_deposit<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountDeposit<'info>>,
        amount: u64,
    ) -> AyclendResult {
        ayclend_account::lending_account_deposit(ctx, amount)
    }

    pub fn lending_account_repay<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountRepay<'info>>,
        amount: u64,
        repay_all: Option<bool>,
    ) -> AyclendResult {
        ayclend_account::lending_account_repay(ctx, amount, repay_all)
    }

    pub fn lending_account_withdraw<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountWithdraw<'info>>,
        amount: u64,
        withdraw_all: Option<bool>,
    ) -> AyclendResult {
        ayclend_account::lending_account_withdraw(ctx, amount, withdraw_all)
    }

    pub fn lending_account_borrow<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountBorrow<'info>>,
        amount: u64,
    ) -> AyclendResult {
        ayclend_account::lending_account_borrow(ctx, amount)
    }

    pub fn lending_account_close_balance(
        ctx: Context<LendingAccountCloseBalance>,
    ) -> AyclendResult {
        ayclend_account::lending_account_close_balance(ctx)
    }

    pub fn lending_account_withdraw_emissions<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountWithdrawEmissions<'info>>,
    ) -> AyclendResult {
        ayclend_account::lending_account_withdraw_emissions(ctx)
    }

    pub fn lending_account_settle_emissions(
        ctx: Context<LendingAccountSettleEmissions>,
    ) -> AyclendResult {
        ayclend_account::lending_account_settle_emissions(ctx)
    }

    /// Liquidate a lending account balance of an unhealthy ayclend account
    pub fn lending_account_liquidate<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountLiquidate<'info>>,
        asset_amount: u64,
    ) -> AyclendResult {
        ayclend_account::lending_account_liquidate(ctx, asset_amount)
    }

    pub fn lending_account_start_flashloan(
        ctx: Context<LendingAccountStartFlashloan>,
        end_index: u64,
    ) -> AyclendResult {
        ayclend_account::lending_account_start_flashloan(ctx, end_index)
    }

    pub fn lending_account_end_flashloan<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingAccountEndFlashloan<'info>>,
    ) -> AyclendResult {
        ayclend_account::lending_account_end_flashloan(ctx)
    }

    // Operational instructions
    pub fn lending_pool_accrue_bank_interest(
        ctx: Context<LendingPoolAccrueBankInterest>,
    ) -> AyclendResult {
        ayclend_group::lending_pool_accrue_bank_interest(ctx)
    }

    pub fn lending_pool_collect_bank_fees<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingPoolCollectBankFees<'info>>,
    ) -> AyclendResult {
        ayclend_group::lending_pool_collect_bank_fees(ctx)
    }

    pub fn lending_pool_withdraw_fees<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingPoolWithdrawFees<'info>>,
        amount: u64,
    ) -> AyclendResult {
        ayclend_group::lending_pool_withdraw_fees(ctx, amount)
    }

    pub fn lending_pool_withdraw_insurance<'info>(
        ctx: Context<'_, '_, 'info, 'info, LendingPoolWithdrawInsurance<'info>>,
        amount: u64,
    ) -> AyclendResult {
        ayclend_group::lending_pool_withdraw_insurance(ctx, amount)
    }

    pub fn set_account_flag(ctx: Context<SetAccountFlag>, flag: u64) -> AyclendResult {
        ayclend_group::set_account_flag(ctx, flag)
    }

    pub fn unset_account_flag(ctx: Context<UnsetAccountFlag>, flag: u64) -> AyclendResult {
        ayclend_group::unset_account_flag(ctx, flag)
    }

    pub fn set_new_account_authority(
        ctx: Context<AyclendAccountSetAccountAuthority>,
    ) -> AyclendResult {
        ayclend_account::set_account_transfer_authority(ctx)
    }

    pub fn ayclend_account_close(ctx: Context<AyclendAccountClose>) -> AyclendResult {
        ayclend_account::close_account(ctx)
    }
}

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "ayclend v2",
    project_url: "https://app.ayclend.com/",
    contacts: "email:security@mrgn.group",
    policy: "https://github.com/mrgnlabs/ayclend-v2/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/mrgnlabs/ayclend-v2"
}
