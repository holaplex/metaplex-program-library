import { config, Program } from '@metaplex-foundation/mpl-core';
import { PublicKey } from '@solana/web3.js';
import * as errors from './generated/errors';
import * as instructions from './generated/instructions';
import * as accounts from './generated/accounts';
import * as beet from '@metaplex-foundation/beet';
import * as anchor from '@project-serum/anchor';
import BN from 'bn.js';
export class AuctionHouseProgram extends Program {
  static readonly PREFIX = 'auction_house';
  static readonly FEE_PAYER = 'fee_payer';
  static readonly TREASURY = 'treasury';
  static readonly SIGNER = 'signer';

  static readonly PUBKEY = new PublicKey(config.programs.auctionHouse);
  static readonly instructions = instructions;
  static readonly errors = errors;
  static readonly accounts = accounts;

  static readonly TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
  static readonly SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  );
  static readonly AUCTION_HOUSE_PROGRAM_ID = new PublicKey(
    'hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk',
  );
  static readonly TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
  );

  // https://github.com/metaplex-foundation/metaplex/blob/master/js/packages/cli/src/helpers/accounts.ts#L504
  static async getAuctionHouseTradeState (
    auctionHouse: PublicKey,
    wallet: PublicKey,
    tokenAccount: PublicKey,
    treasuryMint: PublicKey,
    tokenMint: PublicKey,
    tokenSize: number,
    buyPrice: number,
  ): Promise<[PublicKey, number]> {

    return await PublicKey.findProgramAddress(
      [
        Buffer.from(AuctionHouseProgram.PREFIX),
        wallet.toBuffer(),
        auctionHouse.toBuffer(),
        tokenAccount.toBuffer(),
        treasuryMint.toBuffer(),
        tokenMint.toBuffer(),
        Buffer.from(buyPrice.toString()),
        Buffer.from(tokenSize.toString())
      ],
      AuctionHouseProgram.AUCTION_HOUSE_PROGRAM_ID,
    );
  }

  // https://github.com/metaplex-foundation/metaplex/blob/master/js/packages/cli/src/helpers/accounts.ts#L459
  // ERROR IS "TypeError: Cannot read properties of undefined (reading 'toString')"
  static async getAuctionHouseProgramAsSigner (): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [
        Buffer.from(AuctionHouseProgram.PREFIX,),
        Buffer.from(AuctionHouseProgram.SIGNER,),
      ],
      AuctionHouseProgram.AUCTION_HOUSE_PROGRAM_ID,
    );
  }


  static async getAuctionHouseTreasuryAcct(
    auctionHouse: PublicKey,
  ): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [
        Buffer.from(AuctionHouseProgram.PREFIX),
        auctionHouse.toBuffer(),
        Buffer.from(AuctionHouseProgram.TREASURY),
      ],
      AuctionHouseProgram.AUCTION_HOUSE_PROGRAM_ID,
    );
  };

  static async getMetadata (mint: PublicKey): Promise<PublicKey> {
    return (
      await PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          AuctionHouseProgram.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        AuctionHouseProgram.TOKEN_METADATA_PROGRAM_ID,
      )
    )[0];
  }

  static async getAtaForMint (mint: PublicKey, buyer: PublicKey): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [buyer.toBuffer(), AuctionHouseProgram.TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
      AuctionHouseProgram.SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
    );
  }

  static async findEscrowPaymentAccount (
    auctionHouse: PublicKey,
    wallet: PublicKey,
  ): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from(AuctionHouseProgram.PREFIX, 'utf8'), auctionHouse.toBuffer(), wallet.toBuffer()],
      AuctionHouseProgram.PUBKEY,
    );
  }

  static async findTradeStateAccount (
    wallet: PublicKey,
    auctionHouse: PublicKey,
    tokenAccount: PublicKey,
    treasuryMint: PublicKey,
    tokenMint: PublicKey,
    price: string,
    tokenSize: string,
  ): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [
        Buffer.from(AuctionHouseProgram.PREFIX, 'utf8'),
        wallet.toBuffer(),
        auctionHouse.toBuffer(),
        tokenAccount.toBuffer(),
        treasuryMint.toBuffer(),
        tokenMint.toBuffer(),
        Buffer.from(price, 'utf8'),
        Buffer.from(tokenSize, 'utf8'),
      ],
      AuctionHouseProgram.PUBKEY,
    );
  }
}
