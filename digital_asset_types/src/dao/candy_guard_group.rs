//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::WhitelistMintMode;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "candy_guard_group"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub label: Option<String>,
    pub candy_guard_id: Vec<u8>,
    pub mode: Option<WhitelistMintMode>,
    pub whitelist_mint: Option<Vec<u8>>,
    pub presale: Option<bool>,
    pub discount_price: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Label,
    CandyGuardId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BotTax,
    Lamports,
    SplToken,
    LiveDate,
    ThirdPartySigner,
    Whitelist,
    Gatekeeper,
    EndSettings,
    AllowList,
    MintLimit,
    NftPayment,
    Mode,
    CollectionMint,
    Presale,
    DiscountPrice,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Label => ColumnType::String.def().null(),
            Self::CandyGuardId => ColumnType::Binary.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyMachine => Entity::belongs_to(super::candy_machine::Entity)
                .from(Column::CandyMachineId)
                .to(super::candy_machine::Column::MintAuthority)
                .into(),
            Self::BotTax => Entity::has_one(super::candy_guard_bot_tax::Entity).into(),
            Self::Lamports => Entity::has_one(super::candy_guard_lamports::Entity).into(),
            Self::SplToken => Entity::has_one(super::candy_guard_spl_token::Entity).into(),
            Self::LiveDate => Entity::has_one(super::candy_guard_live_date::Entity).into(),
            Self::ThirdPartySigner => {
                Entity::has_one(super::candy_guard_third_party_signer::Entity).into()
            }
            Self::Gatekeeper => Entity::has_one(super::candy_machine_gatekeeper::Entity).into(),
            Self::EndSettings => Entity::has_one(super::candy_machine_end_settings::Entity).into(),
            Self::AllowList => Entity::has_one(super::candy_guard_allow_list::Entity).into(),
            Self::MintLimit => Entity::has_one(super::candy_guard_mint_limit::Entity).into(),
            Self::NftPayment => Entity::has_one(super::candy_guard_nft_payment::Entity).into(),
            Self::Mode => WhitelistMintMode::db_type().null(),
            Self::CollectionMint => ColumnType::Binary.def().null(),
            Self::Presale => ColumnType::Boolean.def().null(),
            Self::DiscountPrice => ColumnType::Integer.def().null(),
        }
    }
}

impl Related<super::candy_guard_bot_tax::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotTax.def()
    }
}

impl Related<super::candy_guard_lamports::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lamports.def()
    }
}

impl Related<super::candy_guard_spl_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SplToken.def()
    }
}

impl Related<super::candy_guard_live_date::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LiveDate.def()
    }
}

impl Related<super::candy_guard_third_party_signer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThirdPartySigner.def()
    }
}

impl Related<super::candy_machine_creators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Creators.def()
    }
}

impl Related<super::candy_machine_gatekeeper::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gatekeeper.def()
    }
}

impl Related<super::candy_machine_end_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EndSettings.def()
    }
}

impl Related<super::candy_guard_allow_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AllowList.def()
    }
}

impl Related<super::candy_guard_mint_limit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MintLimit.def()
    }
}

impl Related<super::candy_guard_nft_payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NftPayment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
