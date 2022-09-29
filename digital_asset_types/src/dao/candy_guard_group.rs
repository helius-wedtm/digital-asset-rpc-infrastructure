//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::{EndSettingType, WhitelistMintMode};
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
    pub gatekeeper_network: Option<Vec<u8>>,
    pub expire_on_use: Option<bool>,
    pub number: Option<u64>,
    pub end_setting_type: Option<EndSettingType>,
    pub merkle_root: Option<[u8; 32]>,
    pub amount: Option<u64>,
    pub destination: Option<Vec<u8>>,
    pub signer_key: Option<Vec<u8>>,
    pub mint_limit_id: Option<u8>,
    pub mint_limit_limit: Option<u16>,
    pub nft_payment_burn: Option<bool>,
    pub nft_payment_required_collection: Option<Vec<u8>>,
    pub lamports: Option<u64>,
    pub last_instruction: Option<bool>,
    pub live_date: Option<i64>,
    pub spl_token_amount: Option<u64>,
    pub token_mint: Option<Vec<u8>>,
    pub destination_ata: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Label,
    CandyGuardId,
    Mode,
    CollectionMint,
    Presale,
    DiscountPrice,
    GatekeeperNetwork,
    ExpireOnUse,
    Number,
    EndSettingType,
    MerkleRoot,
    Amount,
    Destination,
    SignerKey,
    MintLimitId,
    MintLimitLimit,
    NftPaymentBurn,
    NftPaymentRequiredCollection,
    Lamports,
    LastInstruction,
    LiveDate,
    SplTokenAmount,
    TokenMint,
    DestinationAta,
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
    CandyGuard,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Label => ColumnType::String.def().null(),
            Self::CandyGuardId => ColumnType::Binary.def(),
            Self::Mode => WhitelistMintMode::db_type().null(),
            Self::CollectionMint => ColumnType::Binary.def().null(),
            Self::Presale => ColumnType::Boolean.def().null(),
            Self::DiscountPrice => ColumnType::Integer.def().null(),
            Self::GatekeeperNetwork => ColumnType::Binary.def().null(),
            Self::ExpireOnUse => ColumnType::Boolean.def().null(),
            Self::Number => ColumnType::Integer.def().null(),
            Self::EndSettingType => EndSettingType::db_type().null(),
            Self::Mode => WhitelistMintMode::db_type().null(),
            Self::CollectionMint => ColumnType::Binary.def().null(),
            Self::Presale => ColumnType::Boolean.def().null(),
            Self::DiscountPrice => ColumnType::Integer.def().null(),
            Self::MerkleRoot => ColumnType::Binary.def().null(),
            Self::Amount => ColumnType::Integer.def().null(),
            Self::Destination => ColumnType::Binary.def().null(),
            Self::SignerKey => ColumnType::Binary.def().null(),
            Self::MintLimitId => ColumnType::Integer.def().null(),
            Self::MintLimitLimit => ColumnType::Integer.def().null(),
            Self::NftPaymentBurn => ColumnType::Boolean.def().null(),
            Self::NftPaymentRequiredCollection => ColumnType::Binary.def().null(),
            Self::Lamports => ColumnType::Integer.def().null(),
            Self::LastInstruction => ColumnType::Boolean.def().null(),
            Self::LiveDate => ColumnType::Integer.def().null(),
            Self::SplTokenAmount => ColumnType::Integer.def().null(),
            Self::TokenMint => ColumnType::Binary.def().null(),
            Self::DestinationAta => ColumnType::Binary.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::CandyGuard => Entity::belongs_to(super::candy_guard::Entity)
                .from(Column::CandyGuardId)
                .to(super::candy_guard::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
