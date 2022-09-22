//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "candy_machine_data"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub uuid: Option<String>,
    pub price: Option<u64>,
    pub symbol: String,
    pub seller_fee_basis_points: u16,
    pub max_suppy: u64,
    pub is_mutable: bool,
    pub retain_authority: Option<bool>,
    pub go_live_date: Option<i64>,
    pub items_available: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Uuid,
    Price,
    Symbol,
    SellerFeeBasisPoints,
    MaxSupply,
    IsMutable,
    RetainAuthority,
    GoLiveDate,
    ItemsAvailable,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Vec<u8>;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CandyMachineHiddenSettings,
    CandyMachineEndSettings,
    CandyMachineGatekeeper,
    CandyMachineWhitelistMintSettings,
    CandyMachineCreators,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Uuid => ColumnType::BigInteger.def().null(),
            Self::Price => ColumnType::Binary.def().null(),
            Self::Symbol => ColumnType::Binary.def(),
            Self::SellerFeeBasisPoints => ColumnType::Binary.def(),
            Self::MaxSupply => ColumnType::Integer.def(),
            Self::IsMutable => ColumnType::Boolean.def().null(),
            Self::RelationAuthority => ColumnType::Boolean.def(),
            Self::GoLiveDate => ColumnType::Integer.def().null(),
            Self::ItemsAvailable => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyMachineHiddenSettings => {
                Entity::has_one(super::candy_machine_hidden_settings::Entity).into()
            }
            Self::CandyMachineEndSettings => {
                Entity::has_one(super::candy_machine_end_settings::Entity).into()
            }
            Self::CandyMachineGatekeeper => {
                Entity::has_one(super::candy_machine_gatekeeper::Entity).into()
            }
            Self::CandyMachineWhitelistMintSettings => {
                Entity::has_one(super::candy_machine_whitelist_mint_settings::Entity).into()
            }
            Self::CandyMachineCreators => {
                Entity::has_many(super::candy_machine_creators::Entity).into()
            }
            Self::ConfigLineSettings => {
                Entity::has_one(super::candy_machine_config_line_settings::Entity).into()
            }
        }
    }
}

impl Related<super::candy_machine_whitelist_mint_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineWhitelistMintSettings.def()
    }
}

impl Related<super::candy_machine_gatekeeper::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineGatekeeper.def()
    }
}

impl Related<super::candy_machine_end_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineEndSettings.def()
    }
}

impl Related<super::candy_machine_hidden_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineHiddenSettings.def()
    }
}

impl Related<super::candy_machine_creators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineCreators.def()
    }
}

impl Related<super::candy_machine_config_line_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CandyMachineConfigLineSettings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
