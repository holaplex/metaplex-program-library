mod utils;

#[cfg(test)]
mod create_store {
    use crate::utils::helpers::airdrop;

    use anchor_client::solana_sdk::{signature::Keypair, signer::Signer, system_program};
    use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
    use mpl_membership_token::{
        accounts as mpl_membership_token_accounts, instruction as mpl_membership_token_instruction,
        Store,
    };

    use solana_program::instruction::Instruction;
    use solana_program_test::*;
    use solana_sdk::{transaction::Transaction, transport::TransportError};

    pub fn membership_token_program_test() -> ProgramTest {
        ProgramTest::new("mpl_membership_token", mpl_membership_token::id(), None)
    }

    #[tokio::test]
    async fn success() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

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

        let store_acc = context
            .banks_client
            .get_account(store_keypair.pubkey())
            .await
            .expect("account not found")
            .expect("account empty");

        let store_data = Store::try_deserialize(&mut store_acc.data.as_ref()).unwrap();

        assert_eq!(admin_wallet.pubkey(), store_data.admin);
        assert_eq!(name, store_data.name);
        assert_eq!(description, store_data.description);
    }

    #[tokio::test]
    async fn failure_name_is_long() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

        airdrop(&mut context, &admin_wallet.pubkey(), 10_000_000_000).await;

        let name = String::from("123456789_123456789_12345");
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

        let tx_result = context.banks_client.process_transaction(tx).await;

        match tx_result.unwrap_err() {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn failure_description_is_long() {
        let mut context = membership_token_program_test().start_with_context().await;

        let admin_wallet = Keypair::new();
        let store_keypair = Keypair::new();

        airdrop(&mut context, &admin_wallet.pubkey(), 10_000_000_000).await;

        let name = String::from("123456789_123456789_");
        let description = String::from("123456789_123456789_12345");

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

        let tx_result = context.banks_client.process_transaction(tx).await;

        match tx_result.unwrap_err() {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
