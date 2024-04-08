use anchor_lang::prelude::*;

declare_id!("C2Bj5sizbDg41KzZj1EnCrC3Dg8yCmZVvazQJzWDY8U8");

#[program]
pub mod pump {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
