#[derive(BorshSerialize, BorshDeserialize, Debug)]
use solana_program::{
    account_info::AccountInfo,
    next_account_info,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
}
entrypoint!(counter_contract);
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}
pub fn counter_contract( 
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: [u8],
)-> ProgramResult {
    let acc=next_account_info(accounts.iter())?;


    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter:: try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
         
            counter_data.value += value;
        
            msg!("Incrementing counter by {}", value);
            
        },
        InstructionType::Decrement(value) => {
            counter_data.value -= value;
            
            msg!("Decrementing counter by {}", value);
        },
    }
    counter_data.serialize(&mut &mut *acc.data.borrow_mut());
    msg!("Counter value is now {}", counter_data.value);
    msg!("contract executed successfully");
    Ok(())

}
