mod utils;

#[cfg(test)]
mod init_selling_resource {
    use crate::utils::helpers::{
        airdrop, create_master_edition, create_mint, create_token_account, create_token_metadata,
        mint_to,
    };
    use anchor_client::solana_sdk::{signature::Keypair, signer::Signer, system_program};
    use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
    use mpl_membership_token::{
        accounts as mpl_membership_token_accounts, instruction as mpl_membership_token_instruction,
        SellingResource, SellingResourceState,
    };
    use solana_program::{instruction::Instruction, sysvar};
    use solana_program_test::*;
    use solana_sdk::{transaction::Transaction, transport::TransportError};

    pub fn membership_token_program_test() -> ProgramTest {
        let mut program_test =
            ProgramTest::new("mpl_membership_token", mpl_membership_token::id(), None);
        program_test.add_program("mpl_token_metadata", mpl_token_metadata::id(), None);

        program_test
    }

    #[tokio::test]
    async fn success() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

        // Create `Store`
        airdrop(&mut context, &admin_wallet.pubkey(), 10_000_000_000).await;

        let name = String::from("123456789_123456789_");
        let description = String::from("123456789_123456789_");

        let accounts = mpl_membership_token_accounts::CreateStore {
            admin: admin_wallet.pubkey(),
            store: store_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateStore {
            name: name.to_owned(),
            description: description.to_owned(),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &store_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // Create `SellingResource`
        let resource_mint = Keypair::new();
        create_mint(&mut context, &resource_mint, &admin_wallet.pubkey(), 0).await;

        let resource_token = Keypair::new();
        create_token_account(
            &mut context,
            &resource_token,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let vault = Keypair::new();
        create_token_account(
            &mut context,
            &vault,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let (vault_owner, vault_owner_bump) = mpl_membership_token::find_vault_owner_address(
            &resource_mint.pubkey(),
            &store_keypair.pubkey(),
        );

        mint_to(
            &mut context,
            &resource_mint.pubkey(),
            &resource_token.pubkey(),
            &admin_wallet,
            1,
        )
        .await;

        // Create metadata
        let metadata = create_token_metadata(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            String::from("TEST"),
            String::from("TST"),
            String::from("https://github.com/"),
            100,
            false,
            false,
        )
        .await;

        // Create MasterEdition
        let (master_edition, master_edition_bump) = create_master_edition(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            &metadata,
            Some(3),
        )
        .await;

        let selling_resource = Keypair::new();

        let accounts = mpl_membership_token_accounts::InitSellingResource {
            store: store_keypair.pubkey(),
            admin: admin_wallet.pubkey(),
            selling_resource: selling_resource.pubkey(),
            selling_resource_owner: admin_wallet.pubkey(),
            resource_mint: resource_mint.pubkey(),
            master_edition,
            vault: vault.pubkey(),
            vault_owner,
            resource_token: resource_token.pubkey(),
            rent: sysvar::rent::id(),
            token_program: spl_token::id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::InitSellingResource {
            _master_edition_bump: master_edition_bump,
            _vault_owner_bump: vault_owner_bump,
            max_supply: Some(1),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &selling_resource, &vault],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        let selling_resource_acc = context
            .banks_client
            .get_account(selling_resource.pubkey())
            .await
            .expect("account not found")
            .expect("account empty");

        let selling_resource =
            SellingResource::try_deserialize(&mut selling_resource_acc.data.as_ref()).unwrap();

        assert_eq!(store_keypair.pubkey(), selling_resource.store);
        assert_eq!(admin_wallet.pubkey(), selling_resource.owner);
        assert_eq!(resource_mint.pubkey(), selling_resource.resource);
        assert_eq!(vault.pubkey(), selling_resource.vault);
        assert_eq!(vault_owner, selling_resource.vault_owner);
        assert_eq!(0, selling_resource.supply);
        assert_eq!(Some(1), selling_resource.max_supply);
        assert_eq!(SellingResourceState::Created, selling_resource.state);
    }

    #[tokio::test]
    async fn fail_supply_is_gt_than_available() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

        // Create `Store`
        airdrop(&mut context, &admin_wallet.pubkey(), 10_000_000_000).await;

        let name = String::from("123456789_123456789_");
        let description = String::from("123456789_123456789_");

        let accounts = mpl_membership_token_accounts::CreateStore {
            admin: admin_wallet.pubkey(),
            store: store_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateStore {
            name: name.to_owned(),
            description: description.to_owned(),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &store_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // Create `SellingResource`
        let resource_mint = Keypair::new();
        create_mint(&mut context, &resource_mint, &admin_wallet.pubkey(), 0).await;

        let resource_token = Keypair::new();
        create_token_account(
            &mut context,
            &resource_token,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let vault = Keypair::new();
        create_token_account(
            &mut context,
            &vault,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let (vault_owner, vault_owner_bump) = mpl_membership_token::find_vault_owner_address(
            &resource_mint.pubkey(),
            &store_keypair.pubkey(),
        );

        mint_to(
            &mut context,
            &resource_mint.pubkey(),
            &resource_token.pubkey(),
            &admin_wallet,
            1,
        )
        .await;

        // Create metadata
        let metadata = create_token_metadata(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            String::from("TEST"),
            String::from("TST"),
            String::from("https://github.com/"),
            100,
            false,
            false,
        )
        .await;

        // Create MasterEdition
        let (master_edition, master_edition_bump) = create_master_edition(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            &metadata,
            Some(3),
        )
        .await;

        let selling_resource = Keypair::new();

        let accounts = mpl_membership_token_accounts::InitSellingResource {
            store: store_keypair.pubkey(),
            admin: admin_wallet.pubkey(),
            selling_resource: selling_resource.pubkey(),
            selling_resource_owner: admin_wallet.pubkey(),
            resource_mint: resource_mint.pubkey(),
            master_edition,
            vault: vault.pubkey(),
            vault_owner,
            resource_token: resource_token.pubkey(),
            rent: sysvar::rent::id(),
            token_program: spl_token::id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::InitSellingResource {
            _master_edition_bump: master_edition_bump,
            _vault_owner_bump: vault_owner_bump,
            max_supply: Some(1337),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &selling_resource, &vault],
            context.last_blockhash,
        );

        let err = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();
        match err {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn fail_supply_is_not_provided() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

        // Create `Store`
        airdrop(&mut context, &admin_wallet.pubkey(), 10_000_000_000).await;

        let name = String::from("123456789_123456789_");
        let description = String::from("123456789_123456789_");

        let accounts = mpl_membership_token_accounts::CreateStore {
            admin: admin_wallet.pubkey(),
            store: store_keypair.pubkey(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateStore {
            name: name.to_owned(),
            description: description.to_owned(),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &store_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // Create `SellingResource`
        let resource_mint = Keypair::new();
        create_mint(&mut context, &resource_mint, &admin_wallet.pubkey(), 0).await;

        let resource_token = Keypair::new();
        create_token_account(
            &mut context,
            &resource_token,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let vault = Keypair::new();
        create_token_account(
            &mut context,
            &vault,
            &resource_mint.pubkey(),
            &admin_wallet.pubkey(),
        )
        .await;

        let (vault_owner, vault_owner_bump) = mpl_membership_token::find_vault_owner_address(
            &resource_mint.pubkey(),
            &store_keypair.pubkey(),
        );

        mint_to(
            &mut context,
            &resource_mint.pubkey(),
            &resource_token.pubkey(),
            &admin_wallet,
            1,
        )
        .await;

        // Create metadata
        let metadata = create_token_metadata(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            String::from("TEST"),
            String::from("TST"),
            String::from("https://github.com/"),
            100,
            false,
            false,
        )
        .await;

        // Create MasterEdition
        let (master_edition, master_edition_bump) = create_master_edition(
            &mut context,
            &resource_mint.pubkey(),
            &admin_wallet,
            &metadata,
            Some(3),
        )
        .await;

        let selling_resource = Keypair::new();

        let accounts = mpl_membership_token_accounts::InitSellingResource {
            store: store_keypair.pubkey(),
            admin: admin_wallet.pubkey(),
            selling_resource: selling_resource.pubkey(),
            selling_resource_owner: admin_wallet.pubkey(),
            resource_mint: resource_mint.pubkey(),
            master_edition,
            vault: vault.pubkey(),
            vault_owner,
            resource_token: resource_token.pubkey(),
            rent: sysvar::rent::id(),
            token_program: spl_token::id(),
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::InitSellingResource {
            _master_edition_bump: master_edition_bump,
            _vault_owner_bump: vault_owner_bump,
            max_supply: None,
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &admin_wallet, &selling_resource, &vault],
            context.last_blockhash,
        );

        let err = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();
        match err {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
