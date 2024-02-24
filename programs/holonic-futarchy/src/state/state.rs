use anchor_lang::prelude::*;
use crate::errors::HolonicFutarchyErrors;

#[account]
pub struct Holarchy {
    pub authority: Vec<Pubkey>,
    pub metadata: HolarchyMetadata
}

impl Holarchy {
    pub const SEED: &'static str = "holarchy";
    pub const MAX_SIZE: usize = 32 + (8 + 32) ; // TODO: update this

    pub fn new(&mut self, multisig: Vec<Pubkey>, holarchy_metadata: HolarchyMetadata) {
        self.authority = multisig;
        self.metadata = holarchy_metadata;
    }
}

#[account]
pub struct Holon {
    pub holon_metadata: HolonMetadata
}

impl Holon {
    pub const SEED: &'static str = "holon";
    pub const MAX_SIZE: usize = 4; // TODO: update this

    pub fn new(&mut self, holon_metadata: HolonMetadata) {
        self.holon_metadata = holon_metadata;
    }

    pub fn assert_from_holarchy(&self, holarchy: &Pubkey) -> Result<()> {
        if &self.holon_metadata.holarchy_metadata.key != holarchy {
            return err!(HolonicFutarchyErrors::IncorrectHolarchy);
        }
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolarchyMetadata {
    pub name: String,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolonMetadata {
    pub name: String,
    pub root_holon: Pubkey,
    pub holarchy_metadata: HolarchyMetadata,
    pub futarchy: bool
}
