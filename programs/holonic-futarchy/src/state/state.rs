use anchor_lang::prelude::*;

#[account]
pub struct Holarchy {
    pub metadata: HolarchyMetadata
}

impl Holarchy {
    pub const SEED: &'static str = "holarchy";
    pub const MAX_SIZE: usize = 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HolarchyMetadata {}
