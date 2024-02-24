use anchor_lang::prelude::*;
use coral_multisig::Multisig;
use crate::errors::HolonicFutarchyErrors;

#[account]
pub struct Holarchy {
    pub authority: Vec<Pubkey>, // allows for up to 10 multisig owners
    pub metadata: HolarchyMetadata
}

impl Holarchy {
    pub const SEED: &'static str = "holarchy";
    pub const MAX_SIZE: usize = (32 + (4 + 20)) + (4 + (10 * 32));

    pub fn new(&mut self, multisig: Vec<Pubkey>, holarchy_metadata: HolarchyMetadata) {
        self.authority = multisig;
        self.metadata = holarchy_metadata;
    }
    pub fn assert_multisig(&self, multi_sig: Vec<Pubkey>) -> Result<()> {
        if &self.authority != multi_sig {
            return err!(HolonicFutarchyErrors::NotMultisig);
        }
        Ok(())
    }
}

#[account]
pub struct Holon {
    pub holon_metadata: HolonMetadata
}

impl Holon {
    pub const SEED: &'static str = "holon";
    pub const MAX_SIZE: usize = (32 + (4 + 20)) + (4 + 20) + 32 + 1;

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
    pub name: String, // 4 + 20 (length of string in bytes)
    pub key: Pubkey, // 32
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolonMetadata {
    pub name: String, // 4 + 20 (length of string in bytes)
    pub root_holon: Pubkey, // 32
    pub holarchy_metadata: HolarchyMetadata, // 32 + 4 + 20
    pub futarchy: bool // 1
}
