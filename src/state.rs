use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
// Костыльная реализация мапы вкладчиков.
// Не смог толком понять поддерживает ли Solana HashMap через Borsh или все же нет.
// Порисерчу отдельно если тестовое зайдет.
pub struct Participants {
    pub participants: Vec<(Pubkey, u64)>,
}

impl Participants {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &Pubkey) -> Option<&u64> {
        self.participants
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: Pubkey, value: u64) {
        if let Some((_, v)) = self.participants.iter_mut().find(|(k, _)| k == &key) {
            *v = value;
        } else {
            self.participants.push((key, value));
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, key: &Pubkey) {
        self.participants.retain(|(k, _)| k != key);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vault {
    pub admin: Pubkey,
    pub participants: Participants,
}

impl Vault {
    pub fn initialize(
        program_id: &Pubkey,
        admin: Pubkey,
        vault: &[AccountInfo],
    ) -> Result<(), ProgramError> {
        let account_info_iter = &mut vault.iter();
        let vault_account = next_account_info(account_info_iter)?;
        if !vault_account.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        if vault_account.owner != program_id {
            return Err(ProgramError::IllegalOwner);
        }
        let vault = Vault {
            admin,
            participants: Participants::new(),
        };
        vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;
        msg!("✅ Vault initialized with admin: {}", admin);
        Ok(())
    }
}
