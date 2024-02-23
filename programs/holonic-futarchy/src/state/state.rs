use anchor_lang::prelude::*;

#[account]
pub struct Holarchy {
    pub metadata: HolarchyMetadata
}

impl Holarchy {
    pub const SEED: &'static str = "holarchy";
    pub const MAX_SIZE: usize = 4;

    pub fn new(&mut self, holarchy_metadata: HolarchyMetadata) {
        self.metadata = holarchy_metadata;
    }
}

#[account]
pub struct Holon {
    pub metadata: HolarchyMetadata
}

impl Holon {
    pub const SEED: &'static str = "holon";
    pub const MAX_SIZE: usize = 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolarchyMetadata {
    pub name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolonMetadata {
    pub name: String,
    pub root_holon: Pubkey,
    pub holarchy_metadata: HolarchyMetadata,
    pub futarchy: bool
}
