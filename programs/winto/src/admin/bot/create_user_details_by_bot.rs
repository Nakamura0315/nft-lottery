use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use std::mem::size_of;

/////////////////////////////////////
/// Create User reserved PDA by Bot
///
/// claim-airdrop case
///
/////////////////////////////////////
#[derive(Accounts)]
pub struct CreateUserDetailsByBot<'info> {
    /// bot
    #[account(mut)]
    pub bot_wallet: Signer<'info>,

    /// user wallet
    /// CHECK: Safe account
    pub user_wallet: UncheckedAccount<'info>,

    /// UserDetails PDA
    #[account(
        init,
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump,
        payer = bot_wallet,
        space = 8 + size_of::<UserDetails>(),
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
        constraint = win.bot_wallet == bot_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserDetailsByBot<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        self.user_details.user_wallet = self.user_wallet.key();
        self.user_details.win_airdrop_reward_amount = 0;
        self.user_details.win_game_reward_amount = 0;
        self.user_details.win_dao_reward_amount = 0;
        self.user_details.win_contributors_reward_amount = 0;
        self.user_details.win_last_reward_claimed_date = 0;
        self.user_details.freely_nft_staked_time = 0;
        self.user_details.freely_ticket_amount = 0;
        self.user_details.last_game_distribute_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        self.user_details.freely_nft_type = 0;
        self.user_details.wings_locked_count = 0;
        self.user_details.is_freely_nft_staked = false;
        self.user_details.bump = bump;
        Ok(())
    }
}
