/***********
 * Imports *
 ***********/
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use chainlink_solana as chainlink;

/**************
 * Program ID *
 **************/
declare_id!("9cYRL79dw32gpoimWApf8B1apvs6esD9nnExSFKWiioK");

/***********
 * Program *
 ***********/
// Macro
#[program]
// Module
pub mod solana_chainlink {
    use super::*;

    pub fn execute(context: Context<Execute>) -> ProgramResult {
        let round = chainlink::latest_round_data(
            // Chainlink Program created by Chainlink
            context.accounts.chainlink_program.to_account_info(),
            // Chainlink Data Feed
            context.accounts.chainlink_feed.to_account_info()
        );

        let result_account = &mut context.accounts.result_account;

        result_account.value = round.unwrap().answer;

        Ok(())
    }

}

/*
 * An account is like a file that is stored on the blockchain that contains
 * data, and a context is a list of accounts that a function needs to run
 * successfully.
 */
#[derive(Accounts)]
pub struct Execute<'info> {
    /*
     * 'init' macro:
     *   - 'init' means that we are initializing a new account;
     *   - 'payer=user' means that the user who creates this account would have
     *   to pay for it to be created;
     *   - 'space=100' means that we would allocate 100 bytes of space for this
     *   account on the Solana blockchain.
     */
    #[account(init, payer=user, space=100)]
    pub result_account: Account<'info, ResultAccount>,
    /*
     * Since we are running this 'init' macro, we will need bothe the 'user' and
     * 'system_program' accounts;
     */
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub chainlink_program: AccountInfo<'info>,
    /// CHECK:
    pub chainlink_feed: AccountInfo<'info>,
}

/*
 * The result account is the account where we will record the data that
 * Chainlink provides to our Solana program, and the user will then be able to
 * access the result account andi check the data.
 */
#[account]
pub struct ResultAccount {
    pub value: i128,
}
