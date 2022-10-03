mod v1_asset;
mod master_edition;

use crate::{program_transformers::token_metadata::v1_asset::save_v1_asset, BgTask, IngesterError};
use blockbuster::programs::token_metadata::{TokenMetadataAccountData, TokenMetadataAccountState};
use plerkle_serialization::AccountInfo;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tokio::sync::mpsc::UnboundedSender;
use crate::program_transformers::token_metadata::master_edition::{save_v1_master_edition, save_v2_master_edition};

pub async fn handle_token_metadata_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b TokenMetadataAccountState,
    db: &'c DatabaseConnection,
    task_manager: &UnboundedSender<Box<dyn BgTask>>,
) -> Result<(), IngesterError> {
    let txn = db.begin().await?;
    let key = account_update.pubkey().unwrap().clone();
    match &parsing_result.data {
        // TokenMetadataAccountData::EditionV1(e) => {}
        TokenMetadataAccountData::MasterEditionV1(m) => {
            save_v1_master_edition(key, account_update.slot(), &m, &txn).await?;
            Ok(())
        }
        TokenMetadataAccountData::MetadataV1(m) => {
            let task = save_v1_asset(key, account_update.slot(), &m, &txn).await?;
            task_manager.send(Box::new(task))?;
            Ok(())
        }
        TokenMetadataAccountData::MasterEditionV2(m) => {
            save_v2_master_edition(key, account_update.slot(), m, &txn).await?;
            Ok(())
        }
        // TokenMetadataAccountData::EditionMarker(_) => {}
        // TokenMetadataAccountData::UseAuthorityRecord(_) => {}
        // TokenMetadataAccountData::CollectionAuthorityRecord(_) => {}
        _ => Err(IngesterError::NotImplemented),
    }?;
    txn.commit().await?;
    Ok(())
}
