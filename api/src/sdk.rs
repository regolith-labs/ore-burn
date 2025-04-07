use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn update_score(signer: Pubkey, creator: Pubkey, new_score: u64) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_pda().0, false),
            AccountMeta::new(creator, false),
        ],
        data: UpdateScore {
            new_score: new_score.to_le_bytes(),
        }
        .to_bytes(),
    }
}
