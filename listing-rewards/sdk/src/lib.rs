pub mod accounts;
pub mod args;

use accounts::{
    CancelListingAccounts, CloseOfferAccounts, CreateListingAccounts, CreateOfferAccounts,
};
use anchor_client::solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};
use anchor_lang::{prelude::*, InstructionData};
use args::{CancelListingData, CloseOfferData, CreateListingData, CreateOfferData};
use mpl_auction_house::pda::{
    find_auctioneer_trade_state_address, find_public_bid_trade_state_address,
};
use mpl_listing_rewards::{
    accounts as rewards_accounts, id, instruction,
    listings::{cancel::CancelListingParams, create::CreateListingParams},
    offers::{close::CloseOfferParams, create::CreateOfferParams},
    pda,
    reward_center::{create::CreateRewardCenterParams, edit::EditRewardCenterParams},
    rewardable_collection::{
        create::CreateRewardableCollectionParams, delete::DeleteRewardableCollectionParams,
    },
};
use spl_associated_token_account::get_associated_token_address;

pub fn create_reward_center(
    wallet: Pubkey,
    mint: Pubkey,
    auction_house: Pubkey,
    create_reward_center_params: CreateRewardCenterParams,
) -> Instruction {
    let (reward_center, _) = pda::find_reward_center_address(&auction_house);
    let associated_token_account = get_associated_token_address(&reward_center, &mint);

    let accounts = rewards_accounts::CreateRewardCenter {
        wallet,
        mint,
        auction_house,
        reward_center,
        associated_token_account,
        token_program: spl_token::id(),
        associated_token_program: spl_associated_token_account::id(),
        rent: sysvar::rent::id(),
        system_program: system_program::id(),
    }
    .to_account_metas(None);

    let data = instruction::CreateRewardCenter {
        create_reward_center_params,
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn edit_reward_center(
    wallet: Pubkey,
    auction_house: Pubkey,
    edit_reward_center_params: EditRewardCenterParams,
) -> Instruction {
    let (reward_center, _) = pda::find_reward_center_address(&auction_house);

    let accounts = rewards_accounts::EditRewardCenter {
        wallet,
        auction_house,
        reward_center,
    }
    .to_account_metas(None);

    let data = instruction::EditRewardCenter {
        edit_reward_center_params,
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn create_rewardable_collection(
    wallet: Pubkey,
    auction_house: Pubkey,
    reward_center: Pubkey,
    collection: Pubkey,
) -> Instruction {
    let (rewardable_collection, _) =
        pda::find_rewardable_collection_address(&reward_center, &collection);

    let accounts = rewards_accounts::CreateRewardableCollection {
        wallet,
        reward_center,
        rewardable_collection,
        auction_house,
        system_program: system_program::id(),
    }
    .to_account_metas(None);

    let data = instruction::CreateRewardableCollection {
        rewardable_collection_params: CreateRewardableCollectionParams { collection },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn delete_rewardable_collection(
    wallet: Pubkey,
    auction_house: Pubkey,
    reward_center: Pubkey,
    collection: Pubkey,
) -> Instruction {
    let (rewardable_collection, _) =
        pda::find_rewardable_collection_address(&reward_center, &collection);

    let accounts = rewards_accounts::DeleteRewardableCollection {
        wallet,
        reward_center,
        rewardable_collection,
        auction_house,
    }
    .to_account_metas(None);

    let data = instruction::DeleteRewardableCollection {
        rewardable_collection_params: DeleteRewardableCollectionParams { collection },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn create_listing(
    CreateListingAccounts {
        wallet,
        listing,
        reward_center,
        rewardable_collection,
        token_account,
        metadata,
        authority,
        auction_house,
        seller_trade_state,
        free_seller_trade_state,
    }: CreateListingAccounts,
    CreateListingData {
        price,
        token_size,
        trade_state_bump,
        free_trade_state_bump,
    }: CreateListingData,
) -> Instruction {
    let (auction_house_fee_account, _) =
        mpl_auction_house::pda::find_auction_house_fee_account_address(&auction_house);
    let (ah_auctioneer_pda, _) =
        mpl_auction_house::pda::find_auctioneer_pda(&auction_house, &reward_center);
    let (program_as_signer, program_as_signer_bump) =
        mpl_auction_house::pda::find_program_as_signer_address();

    let accounts = rewards_accounts::CreateListing {
        auction_house_program: mpl_auction_house::id(),
        listing,
        reward_center,
        rewardable_collection,
        wallet,
        token_account,
        metadata,
        authority,
        auction_house,
        auction_house_fee_account,
        seller_trade_state,
        free_seller_trade_state,
        ah_auctioneer_pda,
        program_as_signer,
        token_program: spl_token::id(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
    }
    .to_account_metas(None);

    let data = instruction::CreateListing {
        sell_params: CreateListingParams {
            price,
            token_size,
            trade_state_bump,
            free_trade_state_bump,
            program_as_signer_bump,
        },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn cancel_listing(
    CancelListingAccounts {
        auction_house,
        listing,
        reward_center,
        rewardable_collection,
        authority,
        metadata,
        token_account,
        token_mint,
        treasury_mint,
        wallet,
    }: CancelListingAccounts,
    CancelListingData { token_size, price }: CancelListingData,
) -> Instruction {
    let (auction_house_fee_account, _) =
        mpl_auction_house::pda::find_auction_house_fee_account_address(&auction_house);
    let (ah_auctioneer_pda, _) =
        mpl_auction_house::pda::find_auctioneer_pda(&auction_house, &reward_center);

    let (seller_trade_state, _) = find_auctioneer_trade_state_address(
        &wallet,
        &auction_house,
        &token_account,
        &treasury_mint,
        &token_mint,
        1,
    );

    println!("reward Center {}", reward_center.to_string());

    let accounts = rewards_accounts::CancelListing {
        ah_auctioneer_pda,
        auction_house,
        auction_house_fee_account,
        authority,
        listing,
        metadata,
        reward_center,
        rewardable_collection,
        token_account,
        token_mint,
        trade_state: seller_trade_state,
        wallet,
        auction_house_program: mpl_auction_house::id(),
        token_program: spl_token::id(),
    }
    .to_account_metas(None);

    let data = instruction::CancelListing {
        cancel_listing_params: CancelListingParams { price, token_size },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn create_offer(
    CreateOfferAccounts {
        auction_house,
        authority,
        metadata,
        payment_account,
        reward_center,
        token_account,
        transfer_authority,
        treasury_mint,
        token_mint,
        wallet,
        rewardable_collection,
    }: CreateOfferAccounts,
    CreateOfferData {
        buyer_price,
        token_size,
    }: CreateOfferData,
) -> Instruction {
    let (auction_house_fee_account, _) =
        mpl_auction_house::pda::find_auction_house_fee_account_address(&auction_house);
    let (ah_auctioneer_pda, _) =
        mpl_auction_house::pda::find_auctioneer_pda(&auction_house, &reward_center);

    let (escrow_payment_account, escrow_payment_bump) =
        mpl_auction_house::pda::find_escrow_payment_address(&auction_house, &wallet);

    let (buyer_trade_state, trade_state_bump) = find_public_bid_trade_state_address(
        &wallet,
        &auction_house,
        &treasury_mint,
        &token_mint,
        buyer_price,
        token_size,
    );

    let (offer, _) = pda::find_offer_address(&wallet, &metadata, &rewardable_collection);

    let accounts = rewards_accounts::CreateOffer {
        ah_auctioneer_pda,
        auction_house,
        auction_house_fee_account,
        authority,
        buyer_trade_state,
        metadata,
        payment_account,
        reward_center,
        token_account,
        transfer_authority,
        treasury_mint,
        escrow_payment_account,
        wallet,
        rewardable_collection,
        offer,
        auction_house_program: mpl_auction_house::id(),
        token_program: spl_token::id(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
    }
    .to_account_metas(None);

    let data = instruction::CreateOffer {
        create_offer_params: CreateOfferParams {
            buyer_price,
            escrow_payment_bump,
            token_size,
            trade_state_bump,
        },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

pub fn close_offer(
    CloseOfferAccounts {
        auction_house,
        authority,
        metadata,
        receipt_account,
        reward_center,
        rewardable_collection,
        token_account,
        token_mint,
        treasury_mint,
        wallet,
    }: CloseOfferAccounts,
    CloseOfferData {
        buyer_price,
        token_size,
    }: CloseOfferData,
) -> Instruction {
    let (auction_house_fee_account, _) =
        mpl_auction_house::pda::find_auction_house_fee_account_address(&auction_house);
    let (ah_auctioneer_pda, _) =
        mpl_auction_house::pda::find_auctioneer_pda(&auction_house, &reward_center);
    let (escrow_payment_account, escrow_payment_bump) =
        mpl_auction_house::pda::find_escrow_payment_address(&auction_house, &wallet);

    let (buyer_trade_state, trade_state_bump) = find_public_bid_trade_state_address(
        &wallet,
        &auction_house,
        &treasury_mint,
        &token_mint,
        buyer_price,
        token_size,
    );

    let (offer, _) = pda::find_offer_address(&wallet, &metadata, &rewardable_collection);

    let accounts = rewards_accounts::CloseOffer {
        wallet,
        ah_auctioneer_pda,
        auction_house,
        auction_house_fee_account,
        authority,
        escrow_payment_account,
        metadata,
        offer,
        receipt_account,
        reward_center,
        rewardable_collection,
        token_account,
        token_mint,
        trade_state: buyer_trade_state,
        treasury_mint,
        auction_house_program: mpl_auction_house::id(),
        ata_program: spl_associated_token_account::id(),
        token_program: spl_token::id(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
    }
    .to_account_metas(None);

    let data = instruction::CloseOffer {
        close_offer_params: CloseOfferParams {
            buyer_price,
            escrow_payment_bump,
            token_size,
            trade_state_bump,
        },
    }
    .data();

    Instruction {
        program_id: id(),
        accounts,
        data,
    }
}

// pub fn redeem_rewards() -> Instruction {
//     let accounts = accounts::RedeemRewards {
//         auction_house_program: mpl_auction_house::id(),
//         listing,
//         reward_center,
//         rewardable_collection,
//         wallet,
//         token_program: spl_token::id(),
//         system_program: system_program::id(),
//         rent: sysvar::rent::id(),
//     }
//     .to_account_metas(None);

//     let data = instruction::RedeemRewards {}.data();

//     Instruction {
//         program_id: id(),
//         accounts,
//         data,
//     }
// }