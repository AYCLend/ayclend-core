use crate::events::{GroupEventHeader, AyclendGroupCreateEvent};
use crate::{state::ayclend_group::AyclendGroup, AyclendResult};
use anchor_lang::prelude::*;

pub fn initialize_group(ctx: Context<AyclendGroupInitialize>) -> AyclendResult {
    let ayclend_group = &mut ctx.accounts.ayclend_group.load_init()?;

    ayclend_group.set_initial_configuration(ctx.accounts.admin.key());

    emit!(AyclendGroupCreateEvent {
        header: GroupEventHeader {
            ayclend_group: ctx.accounts.ayclend_group.key(),
            signer: Some(*ctx.accounts.admin.key)
        },
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AyclendGroupInitialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + std::mem::size_of::<AyclendGroup>(),
    )]
    pub ayclend_group: AccountLoader<'info, AyclendGroup>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
