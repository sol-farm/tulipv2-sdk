//! a catch all farm type indicating an unknown farm / error
use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum Unknown {
    Uknown,
}
