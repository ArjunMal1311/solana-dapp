#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("4aop5ewTwAiKttJndNbNpXhRbx2RN6vaJQ66mSRikJDg");

#[program]
pub mod journal {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
      msg!("Journal Entry Created");
      msg!("Title: {}", title);
      msg!("Message: {}", message);

      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.owner = ctx.accounts.owner.key();
      journal_entry.title = title;
      journal_entry.message = message;

      Ok(())
  }

  pub fn update_journal_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
    msg!("Journal Entry Updated");
    msg!("Title: {}", title);
    msg!("Message: {}", message);

    let journal_entry = &mut ctx.accounts.journal_entry;
    journal_entry.message = message;

    Ok(())
  }

  pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, title: String) -> Result<()> {
    msg!("Journal entry titled {} deleted", title);
    Ok(())
  }
}


// I always want to start with program state and the state is where we gonna hold all the data
// and solana contract is stateless i.e. all state is stored inside program accounts
// we will create accounts


#[account]
pub struct JournalEntryState {
    pub owner: Pubkey,
    pub title: String,
    pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],   // suppose title = "first_entry", owner = 6hz5DK14m9RLtutvhN4CPaJEdkWq6bGsZT8X7wE1rHGq, seeds will be
                                                            // ["first_entry" (as bytes), 6hz5DK14m9RLtutvhN4CPaJEdkWq6bGsZT8X7wE1rHGq (as bytes)]
                                                            // pda: something like 9hUrY3u5Rpwh4HLu2Eivq7KGVrS3e1dzPAZ8j5Ajg6fj
                                                            // bump: the bump seed automatically calculated (between 0-255) to make the address valid (i.e., off the elliptic curve).
        bump, 
        payer = owner, 
        space = 8 + 32 + 4 + title.len() + 4 + message.len()
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        realloc = 8 + 32 + 4 + title.len() + 4 + message.len(),
        realloc::payer = owner, 
        realloc::zero = true, 
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account( 
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        close = owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}