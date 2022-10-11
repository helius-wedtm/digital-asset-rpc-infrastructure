mod candy_machine_constants;
mod helpers;
mod initialize;

use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction, system_program, sysvar,
};
use anchor_lang::{InstructionData, ToAccountMetas};
use candy_machine_constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE};
use initialize::make_a_candy_machine;
use mpl_candy_machine::CandyMachineData;
use solana_client::rpc_request::RpcError::RpcRequestError;
use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_program::{instruction::Instruction, native_token::LAMPORTS_PER_SOL};
use solana_sdk::{signature::keypair_from_seed, transaction::Transaction};
use std::{env, sync::Arc, time::Duration};
use tokio::{sync::Semaphore, time::sleep};

#[tokio::main]
async fn main() {
    let sow_thy_seed = env::var("KEYPAIR_SEED").unwrap_or_else(|_| {
        "Cast your bread upon the waters, for you will find it after many days.".to_string()
    });
    let le_blockchain_url =
        env::var("RPC_URL").unwrap_or_else(|_| "http://solana:8899".to_string());
    let network = env::var("NETWORK").unwrap_or_else(|_| "local".to_string());
    let carnage = env::var("AMOUNT_OF_CHAOS").map(|chaos_str| chaos_str.parse::<usize>().expect("How can you mess that up? Okay okay, your AMOUNT OF CHAOS variable is super messed up.")).unwrap_or_else(|_| 64);
    let le_blockchain = Arc::new(RpcClient::new_with_timeout_and_commitment(
        le_blockchain_url.clone(),
        Duration::from_secs(45),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    ));
    let kp = Arc::new(
        keypair_from_seed(sow_thy_seed.as_ref())
            .expect("Thy Keypair is not available, I humbly suggest you look for it."),
    );
    let semaphore = Arc::new(Semaphore::new(carnage));

    check_balance(le_blockchain.clone(), kp.clone(), network != "mainnet").await;
    loop {
        let mut tasks = vec![];
        for _ in 0..carnage {
            let kp = kp.clone();
            let le_clone = le_blockchain.clone();
            let semaphore = semaphore.clone();

            // Start tasks
            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                sleep(Duration::from_millis(1000)).await;
                let res = make_a_candy_machine(le_clone, kp).await;
                // TODO put the ids in a vec and then call update on them
                res
            }));
        }

        for task in tasks {
            match task.await.unwrap() {
                Ok(e) => {
                    println!("Candy machine created with an id of: {:?}", e);
                    let candy_machine_account =
                        le_blockchain.clone().get_account(&e).await.unwrap();

                    println!("candy {:?}", candy_machine_account);
                    continue;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }
        }

        check_balance(le_blockchain.clone(), kp.clone(), network != "mainnet").await;
    }
}

pub async fn check_balance(
    solana_client: Arc<RpcClient>,
    payer: Arc<Keypair>,
    airdrop: bool,
) -> Result<(), ClientError> {
    let sol = solana_client.get_balance(&payer.pubkey()).await?;
    if sol / LAMPORTS_PER_SOL < 1 {
        if airdrop {
            solana_client
                .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 100)
                .await?;
        } else {
            return Err(ClientError::from(RpcRequestError(
                "Not Enough Sol".to_string(),
            )));
        }
    }
    Ok(())
}

// pub async fn add_config_lines(
//     candy_machine: &Pubkey,
//     authority: &Keypair,
//     index: u32,
//     config_lines: Vec<ConfigLine>,
//     solana_client: Arc<RpcClient>,
// ) -> Result<(), ClientError> {
//     let accounts = mpl_candy_machine::accounts::AddConfigLines {
//         candy_machine: *candy_machine,
//         authority: authority.pubkey(),
//     }
//     .to_account_metas(None);

//     let data = mpl_candy_machine::instruction::AddConfigLines {
//         index,
//         config_lines,
//     }
//     .data();

//     let add_config_line_ix = Instruction {
//         program_id: mpl_candy_machine::id(),
//         data,
//         accounts,
//     };

//     let tx = Transaction::new_signed_with_payer(
//         &[add_config_line_ix],
//         Some(&authority.pubkey()),
//         &[authority],
//         solana_client.get_latest_blockhash().await?,
//     );

//     solana_client.send_and_confirm_transaction(&tx).await?;

//     Ok(())
// }

pub async fn initialize_candy_machine(
    candy_account: &Keypair,
    payer: &Arc<Keypair>,
    wallet: &Pubkey,
    candy_data: CandyMachineData,
    // token_info: TokenInfo,
    solana_client: Arc<RpcClient>,
) -> Result<(), ClientError> {
    let items_available = candy_data.items_available;
    let candy_account_size = if candy_data.hidden_settings.is_some() {
        CONFIG_ARRAY_START
    } else {
        CONFIG_ARRAY_START
            + 4
            + items_available as usize * CONFIG_LINE_SIZE
            + 8
            + 2 * (items_available as usize / 8 + 1)
    };

    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &candy_account.pubkey(),
        solana_client
            .get_minimum_balance_for_rent_exemption(candy_account_size)
            .await?,
        candy_account_size as u64,
        &mpl_candy_machine::id(),
    );

    let mut accounts = mpl_candy_machine::accounts::InitializeCandyMachine {
        candy_machine: candy_account.pubkey(),
        wallet: payer.pubkey(),
        authority: payer.pubkey(),
        payer: payer.pubkey(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
    }
    .to_account_metas(None);

    // if token_info.set {
    //     accounts.push(AccountMeta::new_readonly(token_info.mint, false));
    // }

    let data = mpl_candy_machine::instruction::InitializeCandyMachine { data: candy_data }.data();

    let init_ix = Instruction {
        program_id: mpl_candy_machine::id(),
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[create_ix, init_ix],
        Some(&payer.pubkey()),
        &[payer, candy_account],
        solana_client.get_latest_blockhash().await?,
    );

    solana_client.send_and_confirm_transaction(&tx).await?;

    Ok(())
}
