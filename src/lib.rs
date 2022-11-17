use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InstructionData {
    pub vault_bump_seed: u8,
    pub lamports: u64,
}

pub static VAULT_ACCOUNT_SIZE: u64 = 1024;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;

    let vault = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut instruction_data = instruction_data;
    let instr = InstructionData::deserialize(&mut instruction_data)?;
    let vault_bump_seed = instr.vault_bump_seed;
    let lamports = instr.lamports;
    let vault_size = VAULT_ACCOUNT_SIZE;

    invoke_signed(
        &system_instruction::create_account(payer.key, vault.key, lamports, vault_size, program_id),
        &[payer.clone(), vault.clone(), system_program.clone()],
        &[&[b"vault", payer.key.as_ref(), &[vault_bump_seed]]],
    )?;
    Ok(())
}
