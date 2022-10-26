#[cfg(feature = "sql_types")]
use crate::dao::generated::sea_orm_active_enums::{EndSettingType, WhitelistMintMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyMachineCreator {
    pub address: String,
    pub share: i32,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigLineSettings {
    pub prefix_name: String,
    pub name_length: i32,
    pub prefix_uri: String,
    pub uri_length: i32,
    pub is_sequential: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HiddenSettings {
    pub name: String,
    pub uri: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum EndSettingModel {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "amount")]
    Amount,
}

impl From<String> for EndSettingModel {
    fn from(s: String) -> Self {
        match &*s {
            "date" => EndSettingModel::Date,
            "amount" => EndSettingModel::Amount,
            _ => EndSettingModel::Date,
        }
    }
}

#[cfg(feature = "sql_types")]
impl From<EndSettingType> for EndSettingModel {
    fn from(s: EndSettingType) -> Self {
        match s {
            EndSettingType::Date => EndSettingModel::Date,
            EndSettingType::Amount => EndSettingModel::Amount,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EndSettings {
    pub end_setting_type: EndSettingType,
    pub number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FreezeInfo {
    pub allow_thaw: bool,
    pub frozen_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_start: Option<i64>,
    pub freeze_time: i64,
    pub freeze_fee: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Gatekeeper {
    pub gatekeeper_network: String,
    pub expire_on_use: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum WhitelistMintModel {
    #[serde(rename = "burn_every_time")]
    BurnEveryTime,
    #[serde(rename = "never_burn")]
    NeverBurn,
}

impl From<String> for WhitelistMintModel {
    fn from(s: String) -> Self {
        match &*s {
            "burn_every_time" => WhitelistMintModel::BurnEveryTime,
            "never_burn" => WhitelistMintModel::NeverBurn,
            _ => WhitelistMintModel::BurnEveryTime,
        }
    }
}

#[cfg(feature = "sql_types")]
impl From<WhitelistMintMode> for WhitelistMintModel {
    fn from(s: WhitelistMintMode) -> Self {
        match s {
            WhitelistMintMode::BurnEveryTime => WhitelistMintModel::BurnEveryTime,
            WhitelistMintMode::NeverBurn => WhitelistMintModel::NeverBurn,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WhitelistMintSettings {
    pub mode: WhitelistMintMode,
    pub mint: String,
    pub presale: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_price: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BotTax {
    pub lamports: u64,
    pub last_instruction: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolPayment {
    pub lamports: u64,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartDate {
    pub date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EndDate {
    pub date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ThirdPartySigner {
    pub signer_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllowList {
    pub merkle_root: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MintLimit {
    pub id: u8,
    pub limit: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NftPayment {
    pub destination: String,
    pub required_collection: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AddressGate {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RedeemedAmount {
    pub maximum: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FreezeSolPayment {
    pub lamports: u64,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenGate {
    pub amount: u64,
    pub mint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NftGate {
    pub required_collection: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenBurn {
    pub amount: u64,
    pub mint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NftBurn {
    pub required_collection: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenPayment {
    pub amount: u64,
    pub mint: String,
    pub destination_ata: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FreezeTokenPayment {
    pub amount: u64,
    pub mint: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GuardSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_tax: Option<BotTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_signer: Option<ThirdPartySigner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatekeeper: Option<Gatekeeper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<AllowList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_limit: Option<MintLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_payment: Option<NftPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sol_payment: Option<SolPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<StartDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<EndDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_gate: Option<AddressGate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_amount: Option<RedeemedAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_sol_payment: Option<FreezeSolPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_gate: Option<TokenGate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_gate: Option<NftGate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_burn: Option<TokenBurn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_burn: Option<NftBurn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_payment: Option<TokenPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_token_payment: Option<FreezeTokenPayment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyGuardGroup {
    pub label: String,
    pub guards: GuardSet,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyGuardData {
    pub default: GuardSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<CandyGuardGroup>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyGuard {
    pub id: String,
    pub base: String,
    pub bump: u8,
    pub authority: String,
    pub candy_guard_data: CandyGuardData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyMachineData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    pub symbol: String,
    pub seller_fee_basis_points: i16,
    pub max_supply: i64,
    pub is_mutable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_authority: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_live_date: Option<i64>,
    pub items_available: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_line_settings: Option<ConfigLineSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_settings: Option<HiddenSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_settings: Option<EndSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatekeeper: Option<Gatekeeper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_mint_settings: Option<WhitelistMintSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<CandyMachineCreator>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CandyMachine {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_info: Option<FreezeInfo>,
    pub data: CandyMachineData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_authority: Option<String>,
    pub authority: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_mint: Option<String>,
    pub items_redeemed: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candy_guard: Option<CandyGuard>,
}
