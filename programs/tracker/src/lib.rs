use anchor_lang::prelude::*;
// use counter_project::{InitializeMaster, AddMasterCount};

declare_id!("8n6kfBMxG815x9VQsqdGRpNqWf6CFpJHQsPSLdDpGrqq");

#[program]
pub mod tracker {
    use super::*;
    // Initialize tracker if no tracker count
    pub fn initialize(ctx: Context<InitializeTracker>) -> ProgramResult {
        let tracker = &mut ctx.accounts.tracker;
        let authority = &ctx.accounts.authority;

        tracker.counter = 0;
        tracker.authority = *authority.key;
        Ok(())
    }
    // Counter += 1
    pub fn add_tracker_count(ctx: Context<AddTracker>) -> ProgramResult {
        let tracker = &mut ctx.accounts.tracker;
        tracker.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTracker<'info> {
    #[account(init, payer = authority, space = Tracker::LEN)]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddTracker<'info> {
    #[account(mut, has_one = authority)]
    pub tracker: Account<'info, Tracker>,
    pub authority: Signer<'info>
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const MAX_COUNT_LENGTH: usize = 2; // u16

#[account]
pub struct Tracker {
    counter: u32,
    authority: Pubkey
}

impl Tracker {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + MAX_COUNT_LENGTH + PUBLIC_KEY_LENGTH;
}