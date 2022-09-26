use crate::{
    program_transformers::{candy_machine::state::CandyMachine, common::save_changelog_event},
    IngesterError,
};
use blockbuster::{
    instruction::InstructionBundle,
    programs::bubblegum::{BubblegumInstruction, LeafSchema, Payload},
};
use candy_machine::state::CandyMachine;
use digital_asset_types::{
    adapter::{TokenStandard, UseMethod, Uses},
    dao::{
        candy_machine, candy_machine_data, candy_machine_end_settings, candy_machine_freeze,
        candy_machine_gatekeeper, candy_machine_whitelist_mint_settings,
        sea_orm_active_enums::{ChainMutability, Mutability, OwnerType, RoyaltyTargetType},
    },
    json::ChainDataV1,
};
use num_traits::FromPrimitive;
use plerkle_serialization::{
    account_info_generated::account_info::AccountInfo,
    transaction_info_generated::transaction_info::{self},
};
use sea_orm::{
    entity::*, query::*, sea_query::OnConflict, ConnectionTrait, DatabaseTransaction, DbBackend,
    EntityTrait, JsonValue,
};

use super::state::FreezePDA;

pub async fn freeze<'c>(
    freeze: &FreezePDA,
    acct: &AccountInfo<'c>,
    txn: &'c DatabaseTransaction,
) -> Result<(), IngesterError> {
    let candy_machine_freeze = candy_machine::ActiveModel {
        id: Unchanged(freeze.candy_machine.to_bytes().to_vec()),
        allow_thaw: Set(Some(freeze.allow_thaw)),
        frozen_count: Set(Some(freeze.frozen_count)),
        mint_start: Set(freeze.mint_start),
        freeze_time: Set(Some(freeze.freeze_time)),
        freeze_fee: Set(Some(freeze.freeze_fee)),
        ..Default::default()
    };

    let query = candy_machine::Entity::update(candy_machine_freeze)
        .filter(candy_machine::Column::Id.eq(freeze.candy_machine.to_bytes().to_vec()))
        .build(DbBackend::Postgres);
    txn.execute(query).await.map(|_| ()).map_err(Into::into);

    Ok(())
}
