use anchor_lang::prelude::*;
//prelude module of the Anchor framework, which includes essential functionalities for writing Solana programs, such as AnchorDeserialize and AnchorSerialize (for deserialization and serialization), Accounts (macros for defining and managing program accounts), Context (provides information about the current program execution context, including accounts, system programs, etc.).

declare_id!("HZbtnZC9GRDKhcqboSuCiEE9U17cvi5ci5nmmMba2Usx");
//creates a field to store the program address (program_id) allowing you to access a specific on-chain program with a specified program_id.(onchain program address)

//holds the business logic code of the program. The implementation of the instruction functions will be done under the #[program] macro
/* */
#[program]
pub mod first_anchor {
    use super::*;

    //each public fn is a separate instruction
    //Fn Params - Context - contains contextual info about the program execution, 2nd param represents instruction_data

    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

//This is a program derived macro. The structure (Initialize) decorated with this macro defines the collection of accounts that the program requires.
//the below code does the following-
/*
-This macro implements the serialization and deserialization Trait for the given InstructionAccounts
-lists all accounts for the instruction
-It also implements safety checks to ensure that the accounts meet the requirements for secure program execution (like the caller_accnts.owner != program_id check)
*/
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Custom account type
#[account]
pub struct Counter {
    data: u64,
}
