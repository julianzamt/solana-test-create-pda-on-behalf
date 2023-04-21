use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum CreatorInstruction {
    CreatePDAInvokeSigned {},
    CreatePDAInvoke {},
}

impl CreatorInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::CreatorError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::CreatePDAInvokeSigned {},
            1 => Self::CreatePDAInvoke {},
            _ => return Err(errors::CreatorError::InvalidInstruction.into()),
        })
    }
}
