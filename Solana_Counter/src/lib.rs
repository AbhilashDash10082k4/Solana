use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
//A struct from the account_info module allowing us to access account information.
//solana_program=crate, entrypoint=module,
//return value type from the entrypoint module.
//A struct from the pubkey module allowing us to access addresses as public keys.
//A macro allowing us to print messages to the program log

/// Definition of the structure for a data account
/// for structs only derive
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u32,
}
// Declare and export the program's entrypoint
//accounts = to extract all the accounts that the program needs to access. Calling a solana program needs a specific data accnt to be specified
//the order of params of process_instruction matters!!! 
entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    //handling the contract accnt logic

    //s1. get the caller accnt
    //iterating over accnts- looping through accnts to get the specific accnt they need
    let accounts_iter = &mut accounts.iter();

    // The program is doing something that modifies or reads from this data account (like updating a counter, balance, etc.).
    //getting the caller accnt-

    //getting the accnt of teh program caller
    let caller_accnt = next_account_info(accounts_iter)?;

    // s2. Validate the caller's identity, the ownership of the data accnt is given to the current accnt and is verified
    if caller_accnt.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    //s3. read into the data accnt -deserialize the CountAccount struct and get a mutable borrow
    // &caller_acct.data - immutable reference to the data associated to the accnt. This data is binary for program accnt and stored data for data accnt
    let mut counter = CounterAccount::try_from_slice(&caller_accnt.data.borrow())?;

    //CountAccount ->a struct from a data accnt -this is a binary for the program accnt, try_from_slice -> a method from BorshDeserialize trait to deserialize an instance of CountAccount struct from (byte sequence) data accnt to program accnt. &caller_accnt.data.borrow() -borrowable reference of data field from caller_accnt

    //s4 modifying data accnt
    //*- derefernce to obtain the val of caller_accnt.data.borrow_mut() (which is the currently updated value),
    //the value is mutable referenced so that it can be updated repeatitively
    //after changing the val, serialize the struct value into byte array. then write it into the mutable data field of the Solana account -update the field.

    // caller_accnt.data = data field of CounterAccount struct,  borrow_mut() = mutable refernce to the DATA Field, *caller_accnt.data.borrow_mut() = value of the data field,  &mut *caller_accnt.data.borrow_mut() = mutable reference to the VALUE itself
    counter.count += 1;
    counter.serialize(&mut *caller_accnt.data.borrow_mut())?;
    Ok(())
}

//data account
//The actual (de)serialization operations are implemented through the BorshSerialize and BorshDeserialize derive macros. The macros' definitions process metadata of Rust code represented as TokenStream and return the processed metadata.
