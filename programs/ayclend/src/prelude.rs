use anchor_lang::prelude::*;

pub type AyclendResult<G = ()> = Result<G>;

pub use crate::{
    errors::AyclendError,
    state::ayclend_group::{GroupConfig, AyclendGroup},
};
