/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js';
import * as beet from '@metaplex-foundation/beet';
import * as beetSolana from '@metaplex-foundation/beet-solana';

/**
 * Arguments used to create {@link Listing}
 * @category Accounts
 * @category generated
 */
export type ListingArgs = {
  isInitialized: boolean;
  rewardCenter: web3.PublicKey;
  seller: web3.PublicKey;
  metadata: web3.PublicKey;
  price: beet.bignum;
  tokenSize: beet.bignum;
  bump: number;
  createdAt: beet.bignum;
  canceledAt: beet.COption<beet.bignum>;
  purchasedAt: beet.COption<beet.bignum>;
  rewardRedeemedAt: beet.COption<beet.bignum>;
};

const listingDiscriminator = [218, 32, 50, 73, 43, 134, 26, 58];
/**
 * Holds the data for the {@link Listing} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Listing implements ListingArgs {
  private constructor(
    readonly isInitialized: boolean,
    readonly rewardCenter: web3.PublicKey,
    readonly seller: web3.PublicKey,
    readonly metadata: web3.PublicKey,
    readonly price: beet.bignum,
    readonly tokenSize: beet.bignum,
    readonly bump: number,
    readonly createdAt: beet.bignum,
    readonly canceledAt: beet.COption<beet.bignum>,
    readonly purchasedAt: beet.COption<beet.bignum>,
    readonly rewardRedeemedAt: beet.COption<beet.bignum>,
  ) {}

  /**
   * Creates a {@link Listing} instance from the provided args.
   */
  static fromArgs(args: ListingArgs) {
    return new Listing(
      args.isInitialized,
      args.rewardCenter,
      args.seller,
      args.metadata,
      args.price,
      args.tokenSize,
      args.bump,
      args.createdAt,
      args.canceledAt,
      args.purchasedAt,
      args.rewardRedeemedAt,
    );
  }

  /**
   * Deserializes the {@link Listing} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(accountInfo: web3.AccountInfo<Buffer>, offset = 0): [Listing, number] {
    return Listing.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Listing} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
  ): Promise<Listing> {
    const accountInfo = await connection.getAccountInfo(address);
    if (accountInfo == null) {
      throw new Error(`Unable to find Listing account at ${address}`);
    }
    return Listing.fromAccountInfo(accountInfo, 0)[0];
  }

  /**
   * Deserializes the {@link Listing} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Listing, number] {
    return listingBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link Listing} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return listingBeet.serialize({
      accountDiscriminator: listingDiscriminator,
      ...this,
    });
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Listing} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: ListingArgs) {
    const instance = Listing.fromArgs(args);
    return listingBeet.toFixedFromValue({
      accountDiscriminator: listingDiscriminator,
      ...instance,
    }).byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Listing} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: ListingArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(Listing.byteSize(args), commitment);
  }

  /**
   * Returns a readable version of {@link Listing} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      isInitialized: this.isInitialized,
      rewardCenter: this.rewardCenter.toBase58(),
      seller: this.seller.toBase58(),
      metadata: this.metadata.toBase58(),
      price: (() => {
        const x = <{ toNumber: () => number }>this.price;
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      tokenSize: (() => {
        const x = <{ toNumber: () => number }>this.tokenSize;
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      bump: this.bump,
      createdAt: (() => {
        const x = <{ toNumber: () => number }>this.createdAt;
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      canceledAt: this.canceledAt,
      purchasedAt: this.purchasedAt,
      rewardRedeemedAt: this.rewardRedeemedAt,
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const listingBeet = new beet.FixableBeetStruct<
  Listing,
  ListingArgs & {
    accountDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['isInitialized', beet.bool],
    ['rewardCenter', beetSolana.publicKey],
    ['seller', beetSolana.publicKey],
    ['metadata', beetSolana.publicKey],
    ['price', beet.u64],
    ['tokenSize', beet.u64],
    ['bump', beet.u8],
    ['createdAt', beet.i64],
    ['canceledAt', beet.coption(beet.i64)],
    ['purchasedAt', beet.coption(beet.i64)],
    ['rewardRedeemedAt', beet.coption(beet.i64)],
  ],
  Listing.fromArgs,
  'Listing',
);
