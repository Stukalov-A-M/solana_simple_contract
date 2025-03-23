use crate::state::Vault;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{msg, system_instruction};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instructions {
    Deposit(u64),
    CheckBalance,
    Withdraw(u64),
    Initialize,
}

impl Instructions {
    pub fn initialize(
        program_id: &Pubkey,
        admin: Pubkey,
        accounts: &[AccountInfo],
    ) -> Result<(), ProgramError> {
        Vault::initialize(program_id, admin, accounts)
    }

    pub fn deposit<'a>(
        program_id: &Pubkey,
        from: &AccountInfo<'a>,
        vault: &AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        if !from.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if vault.owner != program_id {
            return Err(ProgramError::IllegalOwner);
        }
        let instruction = system_instruction::transfer(from.key, vault.key, amount);
        invoke(&instruction, &[from.clone(), vault.clone()])?;
        msg!("💰 Deposit {} lamports from {}", amount, from.key);
        Ok(())
    }

    pub fn check_balance(program_id: &Pubkey, vault: &AccountInfo) -> Result<(), ProgramError> {
        if vault.owner != program_id {
            return Err(ProgramError::IllegalOwner);
        }
        msg!("🏦 Vault balance: {} lamports", vault.lamports());
        Ok(())
    }

    pub fn withdraw<'a>(
        program_id: &Pubkey,
        vault: &AccountInfo<'a>,
        recipient: &AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        if !recipient.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if vault.owner != program_id {
            return Err(ProgramError::IllegalOwner);
        }
        let vault_struct = Vault::try_from_slice(&vault.data.borrow())?;
        if let Some(deposited_amount) = vault_struct.participants.get(recipient.key) {
            if deposited_amount < &amount {
                msg!(
                    "User with the public key {} haven't enough balance",
                    recipient.key
                );
                return Err(ProgramError::Custom(12));
            };
            let instruction = system_instruction::transfer(vault.key, recipient.key, amount);
            invoke(&instruction, &[vault.clone(), recipient.clone()])?;
            msg!("🚀 Withdraw {} lamports to {}", amount, recipient.key);
            Ok(())
        } else {
            msg!(
                "User with the public key {} haven't fount in the depositors pool",
                recipient.key
            );
            Err(ProgramError::Custom(13))
        }
    }
}
