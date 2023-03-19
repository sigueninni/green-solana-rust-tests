use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::{
    instruction::{initialize_account, initialize_mint, mint_to},
    state::{Account, Mint},
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Decode the instruction
    let instruction = TokenInstruction::unpack(instruction_data)?;

    // Match the instruction and call the corresponding function
    match instruction {
        TokenInstruction::InitializeMint { decimals, mint_authority } => {
            process_initialize_mint(accounts, decimals, mint_authority, program_id)?;
        }
        TokenInstruction::InitializeAccount => {
            process_initialize_account(accounts, program_id)?;
        }
        TokenInstruction::MintTo { amount } => {
            process_mint_to(accounts, amount, program_id)?;
        }
    }

    Ok(())
}

// Instruction enum for the token program
#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
    InitializeMint {
        decimals: u8,
        mint_authority: Pubkey,
    },
    InitializeAccount,
    MintTo {
        amount: u64,
    },
}

impl Sealed for TokenInstruction {}
impl IsInitialized for TokenInstruction {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for TokenInstruction {
    const LEN: usize = 33;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        match self {
            TokenInstruction::InitializeMint { decimals, mint_authority } => {
                dst[0] = 0;
                dst[1..9].copy_from_slice(&decimals.to_le_bytes());
                dst[9..].copy_from_slice(&mint_authority.to_bytes());
            }
            TokenInstruction::InitializeAccount => {
                dst[0] = 1;
            }
            TokenInstruction::MintTo { amount } => {
                dst[0] = 2;
                dst[1..].copy_from_slice(&amount.to_le_bytes());
            }
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let tag = src[0];
        match tag {
            0 => {
                let decimals = u8::from_le_bytes(src[1..9].try_into().unwrap());
                let mint_authority = Pubkey::new_from_array(src[9..].try_into().unwrap());
                Ok(TokenInstruction::InitializeMint {
                    decimals,
                    mint_authority,
                })
            }
            1 => Ok(TokenInstruction::InitializeAccount),
            2 => {
                let amount = u64::from_le_bytes(src[1..9].try_into().unwrap());
                Ok(TokenInstruction::MintTo { amount })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

fn process_initialize_mint(
    accounts: &[AccountInfo],
    decimals: u8,
    mint_authority: Pubkey,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account
