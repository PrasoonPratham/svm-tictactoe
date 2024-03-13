use anchor_lang::prelude::*;

declare_id!("CKkkguDzG9hyiAHDwkB3qw8wT2pgBUwuQtcuwUqAPD2V");

#[program]
pub mod tictactoe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
