use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn increase(counter_data: &mut Counter) -> ProgramResult {
    // Place your custom code here...
    counter_data.number += 1;
    Ok(())
}
