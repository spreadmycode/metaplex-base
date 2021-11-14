mod whitelist;

use {
    anchor_lang::{
        prelude::*, AnchorDeserialize, AnchorSerialize,
    },
    whitelist::WHITELIST,
};

#[program]
pub mod nft_candy_machine {

    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    ///                          Smart Contract                             ///
    ///////////////////////////////////////////////////////////////////////////

    /**
     *  Initialize contract data
     */
    pub fn initialize_contract(
        ctx: Context<Initialize>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;

        data.period_status = PeriodStatus::PreSale as u8;

        Ok(())
    }

    /**
     *  Check mint possible
     */
    pub fn check_mint_possible(
        ctx: Context<Status>,
        pub_key: String,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;

        if data.period_status == PeriodStatus::PendingSale as u8 {               // Pending-sale period
            data.check_status = ResultCode::NotAvailable as u8;
            return Ok(());
        }

        if data.period_status == PeriodStatus::PreSale as u8 {                   // Pre-sale period
            for x in &WHITELIST {
                if pub_key == *x {
                    data.check_status = ResultCode::Available as u8;
                    return Ok(());
                }
            }
            data.check_status = ResultCode::NotExistInWhiteList as u8;
            return Ok(());
        } 
        
        if data.period_status == PeriodStatus::PostSale as u8 {                 // Post-sale period
            data.check_status = ResultCode::Available as u8;
            return Ok(());
        }

        return Ok(())
    }

    /**
     *  Set sale is pending
     */
    pub fn set_pending(
        ctx: Context<Status>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;
        data.period_status = PeriodStatus::PendingSale as u8;

        Ok(())
    }

    /**
     *  Toggle pre/post sale period
     */
    pub fn toggle_period(
        ctx: Context<Status>
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;

        if data.period_status == PeriodStatus::PreSale as u8 {
            data.period_status = PeriodStatus::PostSale as u8;
        }

        if data.period_status == PeriodStatus::PostSale as u8 {
            data.period_status = PeriodStatus::PreSale as u8;
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub data: ProgramAccount<'info, Data>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Status<'info> {
    #[account(mut)]
    pub data: ProgramAccount<'info, Data>,
    pub minter: AccountInfo<'info>,
}

#[account]
pub struct Data {
	pub check_status: u8,
    pub period_status: u8,
}

pub enum ResultCode {
    NotAvailable        = 0,
    Available           = 1,
    NotExistInWhiteList = 2,
}

pub enum PeriodStatus {
    PendingSale = 0,
    PreSale     = 1,
    PostSale    = 2
}