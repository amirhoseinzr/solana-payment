use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
    msg,
};
use solana_program::program_error::ProgramError;

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer_account = next_account_info(accounts_iter)?;
    let recipient_account = next_account_info(accounts_iter)?;


    let recipient_pubkey = Pubkey::new(&instruction_data[0..32]);

    if *recipient_account.key != recipient_pubkey {
        msg!("Recipient account does not match the provided public key.");
        return Err(ProgramError::InvalidArgument);
    }

    let transfer_instruction = system_instruction::transfer(
        &payer_account.key,
        &recipient_account.key,
        1_000_000_000, // 1 sol is eq this value lamport
    );

    invoke(
        &transfer_instruction,
        &[payer_account.clone(), recipient_account.clone()],
    )?;

    Ok(())
}
