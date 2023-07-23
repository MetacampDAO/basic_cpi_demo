use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    associated_token::{create, AssociatedToken, Create},
    token::{initialize_mint, mint_to, transfer, InitializeMint, Mint, MintTo, Token, Transfer},
};

declare_id!("FxxMt1Cxu35N2MM9e4vDAGW2NaV78oF6KyjUn6WRwmQv");

#[program]
pub mod global_counter {
    use super::*;

    pub fn create_mint_and_transfer_to(
        ctx: Context<CreateMintAndTransferTo>,
        mint_amount: u64,
        transfer_amount: u64,
    ) -> Result<()> {
        // EASIER REFERENCE
        let token_program = ctx.accounts.token_program.to_account_info();

        // CREATE AN ACCOUNT FOR MINT
        let create_account_cpi_accounts = CreateAccount {
            from: ctx.accounts.initializer.to_account_info(),
            to: ctx.accounts.mint.to_account_info(),
        };
        let create_account_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            create_account_cpi_accounts,
        );
        create_account(
            create_account_context,
            ctx.accounts.rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &token_program.key(),
        )?;

        // INITIALIZER NEW MINT
        let init_mint_cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let create_mint_context = CpiContext::new(token_program.clone(), init_mint_cpi_accounts);
        initialize_mint(
            create_mint_context,
            2,
            &ctx.accounts.initializer.key(),
            None,
        )?;

        // CREATE NEW ATA FOR INITIALIZER
        let init_initializer_ata_cpi_accounts = Create {
            payer: ctx.accounts.initializer.to_account_info(),
            associated_token: ctx.accounts.initializer_ata.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let create_ata_context_1 =
            CpiContext::new(token_program.clone(), init_initializer_ata_cpi_accounts);
        create(create_ata_context_1)?;

        // CREATE NEW ATA FOR RECEIVER
        let init_receiver_ata_cpi_accounts = Create {
            payer: ctx.accounts.initializer.to_account_info(),
            associated_token: ctx.accounts.receiver_ata.to_account_info(),
            authority: ctx.accounts.receiver.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let create_ata_context_2 =
            CpiContext::new(token_program.clone(), init_receiver_ata_cpi_accounts);
        create(create_ata_context_2)?;

        // MINT mint_amount TO INITIALIZER
        let mint_to_cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.initializer_ata.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info(),
        };
        let mint_to_context = CpiContext::new(token_program.clone(), mint_to_cpi_accounts);

        mint_to(mint_to_context, mint_amount)?;

        // TRANSFER transfer_amount FROM INITIALIZER TO RECEVIER
        let transfer_cpi_accounts = Transfer {
            from: ctx.accounts.initializer_ata.to_account_info(),
            to: ctx.accounts.receiver_ata.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info(),
        };
        let transfer_context = CpiContext::new(token_program.clone(), transfer_cpi_accounts);
        transfer(transfer_context, transfer_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMintAndTransferTo<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// CHECK: For ATA Reference
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: To be initialize
    pub initializer_ata: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: To be initialize
    pub receiver_ata: AccountInfo<'info>,
    /// CHECK: Not Mint Yet
    #[account(mut)]
    pub mint: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Counter {
    owner: Pubkey,
    is_initialized: bool,
    count: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Use the current count")]
    IncorrectCounterOwner,
}
