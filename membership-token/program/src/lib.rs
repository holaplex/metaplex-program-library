mod utils;

use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::token::{self, Token};

pub const STRING_DEFAULT_SIZE: usize = 20;
pub const HOLDER_PREFIX: &str = "holder";
pub const HISTORY_PREFIX: &str = "history";
pub const VAULT_OWNER_PREFIX: &str = "mt_vault";

/// Return `treasury_owner` Pubkey and bump seed.
pub fn find_treasury_owner_address(
    treasury_mint: &Pubkey,
    selling_resource: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            HOLDER_PREFIX.as_bytes(),
            treasury_mint.as_ref(),
            selling_resource.as_ref(),
        ],
        &id(),
    )
}

/// Return `vault_owner` Pubkey and bump seed.
pub fn find_vault_owner_address(resource_mint: &Pubkey, store: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            VAULT_OWNER_PREFIX.as_bytes(),
            resource_mint.as_ref(),
            store.as_ref(),
        ],
        &id(),
    )
}

/// Return `TradeHistory` Pubkey and bump seed.
pub fn find_trade_history_address(wallet: &Pubkey, market: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[HISTORY_PREFIX.as_bytes(), wallet.as_ref(), market.as_ref()],
        &id(),
    )
}

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod membership_token {
    use super::*;

    pub fn init_selling_resource<'info>(
        ctx: Context<'_, '_, '_, 'info, InitSellingResource<'info>>,
        _master_edition_bump: u8,
        _vault_owner_bump: u8,
        max_supply: Option<u64>,
    ) -> ProgramResult {
        let store = &ctx.accounts.store;
        let admin = &ctx.accounts.admin;
        let selling_resource = &mut ctx.accounts.selling_resource;
        let selling_resource_owner = &ctx.accounts.selling_resource_owner;
        let resource_mint = &ctx.accounts.resource_mint;
        let master_edition_info = &ctx.accounts.master_edition.to_account_info();
        let vault = &ctx.accounts.vault;
        let vault_owner = &ctx.accounts.vault_owner;
        let resource_token = &ctx.accounts.resource_token;
        let _rent = &ctx.accounts.rent;
        let token_program = &ctx.accounts.token_program;
        let _system_program = &ctx.accounts.system_program;

        // Check `MasterEdition` derivation
        utils::assert_derivation(
            &mpl_token_metadata::id(),
            master_edition_info,
            &[
                mpl_token_metadata::state::PREFIX.as_bytes(),
                mpl_token_metadata::id().as_ref(),
                resource_mint.key().as_ref(),
                mpl_token_metadata::state::EDITION.as_bytes(),
            ],
        )?;

        let master_edition =
            mpl_token_metadata::state::MasterEditionV2::from_account_info(master_edition_info)?;

        let mut actual_max_supply = max_supply;

        // Ensure, that provided `max_supply` is under `MasterEditionV2::max_supply` bounds
        if let Some(me_max_supply) = master_edition.max_supply {
            let x = if let Some(max_supply) = max_supply {
                let available_supply = me_max_supply - master_edition.supply;
                if max_supply > available_supply {
                    return Err(ErrorCode::SupplyIsGtThanAvailable.into());
                } else {
                    max_supply
                }
            } else {
                return Err(ErrorCode::SupplyIsNotProvided.into());
            };

            actual_max_supply = Some(x);
        }

        // Transfer `MasterEdition` ownership
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: resource_token.to_account_info(),
            to: vault.to_account_info(),
            authority: admin.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        selling_resource.store = store.key();
        selling_resource.owner = selling_resource_owner.key();
        selling_resource.resource = resource_mint.key();
        selling_resource.vault = vault.key();
        selling_resource.vault_owner = vault_owner.key();
        selling_resource.supply = 0;
        selling_resource.max_supply = actual_max_supply;
        selling_resource.state = SellingResourceState::Created;

        Ok(())
    }

    pub fn create_store<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateStore<'info>>,
        name: String,
        description: String,
    ) -> ProgramResult {
        let admin = &ctx.accounts.admin;
        let store = &mut ctx.accounts.store;

        if !admin.to_account_info().is_signer || !store.to_account_info().is_signer {
            return Err(ErrorCode::NoValidSignerPresent.into());
        }

        if name.len() > STRING_DEFAULT_SIZE || description.len() > STRING_DEFAULT_SIZE {
            return Err(ErrorCode::StringIsTooLong.into());
        }

        store.admin = admin.key();
        store.name = name;
        store.description = description;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(master_edition_bump:u8, vault_owner_bump: u8, max_supply: Option<u64>)]
pub struct InitSellingResource<'info> {
    #[account(has_one=admin)]
    store: Account<'info, Store>,
    #[account(mut)]
    admin: Signer<'info>,
    #[account(init, payer=admin, space=SellingResource::LEN)]
    selling_resource: Account<'info, SellingResource>,
    selling_resource_owner: UncheckedAccount<'info>,
    resource_mint: UncheckedAccount<'info>,
    #[account(owner=mpl_token_metadata::id())]
    master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    vault: Signer<'info>,
    #[account(seeds=[VAULT_OWNER_PREFIX.as_bytes(), resource_mint.key().as_ref(), store.key().as_ref()], bump=vault_owner_bump)]
    vault_owner: UncheckedAccount<'info>,
    #[account(mut)]
    resource_token: UncheckedAccount<'info>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateStore<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(init, space=Store::LEN, payer=admin)]
    store: Account<'info, Store>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Store {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
}

impl Store {
    pub const LEN: usize = 8 + 32 + STRING_DEFAULT_SIZE * 4 + STRING_DEFAULT_SIZE * 4;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub enum SellingResourceState {
    Uninitialized,
    Created,
    InUse,
    Exhausted,
    Stopped,
}

#[account]
pub struct SellingResource {
    pub store: Pubkey,
    pub owner: Pubkey,
    pub resource: Pubkey,
    pub vault: Pubkey,
    pub vault_owner: Pubkey,
    pub supply: u64,
    pub max_supply: Option<u64>,
    pub state: SellingResourceState,
}

impl SellingResource {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 9 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum MarketState {
    Uninitialized,
    Created,
    Active,
    Ended,
}

#[account]
pub struct Market {
    pub store: Pubkey,
    pub selling_resource: Pubkey,
    pub treasury_mint: Pubkey,
    pub treasury_holder: Pubkey,
    pub treasury_owner: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub mutable: bool,
    pub price: u64,
    pub pieces_in_one_wallet: Option<u64>,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub state: MarketState,
}

impl Market {
    pub const LEN: usize = 8
        + 32
        + 32
        + 32
        + 32
        + 32
        + 32
        + STRING_DEFAULT_SIZE * 4
        + STRING_DEFAULT_SIZE * 4
        + 1
        + 8
        + 9
        + 8
        + 9
        + 1;
}

#[account]
pub struct TradeHistory {
    pub market: Pubkey,
    pub wallet: Pubkey,
    pub already_bought: u64,
}

impl TradeHistory {
    pub const LEN: usize = 8 + 32 + 32 + 8;
}

#[error]
pub enum ErrorCode {
    #[msg("No valid signer present")]
    NoValidSignerPresent,
    #[msg("Some string variable is longer than allowed")]
    StringIsTooLong,
    #[msg("Provided supply is gt than available")]
    SupplyIsGtThanAvailable,
    #[msg("Supply is not provided")]
    SupplyIsNotProvided,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
}
