use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program,
    program_pack::IsInitialized,
    sysvar::{rent::Rent, Sysvar},
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

const USER_STAKE_SIZE: usize = 1 + 8;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserStake {
    pub is_initialized: bool,
    pub lamports: u64,
}

impl IsInitialized for UserStake {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Create PDA example program_id {}", program_id);

    let accounts_iter = &mut accounts.iter();
    let user = next_account_info(accounts_iter)?;
    let user_derived_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if !system_program::check_id(system_program_account.key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(&[user.key.as_ref()], program_id);
    if pda != *user_derived_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let rent_lamports = Rent::get()?.minimum_balance(USER_STAKE_SIZE);
    msg!(
        "user have to pay {} lamports for rent exemption of {} bytes",
        rent_lamports,
        USER_STAKE_SIZE
    );

    invoke_signed(
        &system_instruction::create_account(user.key, user_derived_account.key, rent_lamports, USER_STAKE_SIZE.try_into().unwrap(), program_id),
        &[user.clone(), user_derived_account.clone(), system_program_account.clone()], &[&[user.key.as_ref(), &[bump_seed]]],
    )?;
    Ok(())
}
