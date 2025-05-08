use ore_api::state::proof_pda;
use ore_boost_api::state::{boost_pda, config_pda, stake_pda};
use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey) -> Instruction {
    let authority = authority_pda().0;
    let boost = boost_pda(crate::consts::NFT_MINT_ADDRESS).0;
    let boost_config = config_pda().0;
    let boost_deposits = spl_associated_token_account::get_associated_token_address(
        &boost,
        &crate::consts::NFT_MINT_ADDRESS,
    );
    let boost_proof = proof_pda(boost_config).0;
    let boost_rewards = spl_associated_token_account::get_associated_token_address(
        &boost_config,
        &ore_api::consts::MINT_ADDRESS,
    );
    let rewards = spl_associated_token_account::get_associated_token_address(
        &authority,
        &ore_api::consts::MINT_ADDRESS,
    );
    let sender = spl_associated_token_account::get_associated_token_address(
        &authority,
        &crate::consts::NFT_MINT_ADDRESS,
    );
    let stake = stake_pda(authority, boost).0;

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(boost, false),
            AccountMeta::new(boost_config, false),
            AccountMeta::new(boost_deposits, false),
            AccountMeta::new(boost_proof, false),
            AccountMeta::new(boost_rewards, false),
            AccountMeta::new(crate::consts::NFT_MINT_ADDRESS, false),
            AccountMeta::new(ore_api::consts::MINT_ADDRESS, false),
            AccountMeta::new(rewards, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(stake, false),
            AccountMeta::new(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(ore_boost_api::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn bury(signer: Pubkey) -> Instruction {
    let authority = authority_pda().0;
    let boost = boost_pda(crate::consts::NFT_MINT_ADDRESS).0;
    let boost_config = config_pda().0;
    let boost_proof = proof_pda(boost_config).0;
    let boost_rewards = spl_associated_token_account::get_associated_token_address(
        &boost_config,
        &ore_api::consts::MINT_ADDRESS,
    );
    let rewards = spl_associated_token_account::get_associated_token_address(
        &authority,
        &ore_api::consts::MINT_ADDRESS,
    );
    let stake = stake_pda(authority, boost).0;

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(boost, false),
            AccountMeta::new(boost_config, false),
            AccountMeta::new(boost_proof, false),
            AccountMeta::new(boost_rewards, false),
            AccountMeta::new(ore_api::consts::MINT_ADDRESS, false),
            AccountMeta::new(rewards, false),
            AccountMeta::new(stake, false),
            AccountMeta::new(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::ID, false),
            AccountMeta::new_readonly(ore_boost_api::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Bury {}.to_bytes(),
    }
}
