use anchor_client::solana_sdk::{bpf_loader_upgradeable, pubkey::Pubkey};
use anyhow::Result;

use crate::service::MultisigService;

pub fn propose_upgrade(
    service: &MultisigService,
    multisig: &Pubkey,
    program_address: &Pubkey,
    buffer_address: &Pubkey,
) -> Result<Pubkey> {
    let signer = service.program.signer(*multisig).0;
    let instruction = bpf_loader_upgradeable::upgrade(
        program_address,
        buffer_address,
        &signer,
        &service.program.payer.pubkey(),
    );
    service.propose_solana_instruction(multisig, instruction)
}

pub fn propose_set_upgrade_authority(
    service: &MultisigService,
    multisig: &Pubkey,
    program_address: &Pubkey,
    new_authority_address: Option<&Pubkey>,
) -> Result<Pubkey> {
    let current_authority_address = service.program.signer(*multisig).0;
    let instruction = bpf_loader_upgradeable::set_upgrade_authority(
        program_address,
        &current_authority_address,
        new_authority_address,
    );
    service.propose_solana_instruction(multisig, instruction)
}
