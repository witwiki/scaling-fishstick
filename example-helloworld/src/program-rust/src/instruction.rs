use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32)
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //  First split our input and make sure we're not getting a ProgramError
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        // Use switch case to parse through the instruction data set
        match tag {
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            // remaining instruction_data is a set of four enum types
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => {
                        return Ok(HelloInstruction::Set(u32::from_le_bytes(i)))
                    },
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
        
    }
}