use crate::instruction::Instructions;
use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instructions::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let account_info_iter = &mut accounts.iter();

    match instruction {
        Instructions::Initialize => {
            let admin = next_account_info(account_info_iter)?.key;
            Instructions::initialize(program_id, *admin, accounts)?;
        }
        Instructions::Deposit(amount) => {
            let from = next_account_info(account_info_iter)?;
            let vault = next_account_info(account_info_iter)?;
            Instructions::deposit(program_id, from, vault, amount)?;
        }
        Instructions::CheckBalance => {
            let vault = next_account_info(account_info_iter)?;
            Instructions::check_balance(program_id, vault)?
        }
        Instructions::Withdraw(amount) => {
            let vault = next_account_info(account_info_iter)?;
            let recipient = next_account_info(account_info_iter)?;
            Instructions::withdraw(program_id, vault, recipient, amount)?;
        }
    }

    Ok(())
}
