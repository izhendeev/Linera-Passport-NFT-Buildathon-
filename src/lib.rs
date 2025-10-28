pub mod state;
pub use state::PassportState;

use async_graphql::{InputObject, Request, Response, SimpleObject};
use linera_base::data_types::Timestamp;
use linera_base::identifiers::{AccountOwner, ChainId};
use linera_sdk::abi::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

/// Типы ABI, разделяемые контрактом и сервисом.
pub struct PassportNftAbi;

impl ContractAbi for PassportNftAbi {
    type Operation = PassportOperation;
    type Response = ();
}

impl ServiceAbi for PassportNftAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Идентификатор токена
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, SimpleObject, InputObject,
)]
#[graphql(input_name = "TokenIdInput")]
pub struct TokenId {
    pub id: Vec<u8>,
}

/// Основная структура паспорта
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Passport {
    pub token_id: TokenId,
    pub owner: AccountOwner,
    pub created_at: Timestamp,
    /// Цифровой след цепочки, в которой был выпущен паспорт
    pub owner_chain: ChainId,
    /// URI с off-chain метаданными
    pub metadata_uri: String,
    /// URI для обложки/изображения паспорта
    pub image_uri: String,
    /// Контрольная сумма off-chain контента (например, SHA-256 hex)
    pub content_hash: String,
    pub achievements: Vec<String>,
    pub score: u64,
}

/// Mint аргументы
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct MintArgs {
    pub token_id: TokenId,
    /// URI с off-chain метаданными
    pub metadata_uri: String,
    /// URI для обложки/изображения паспорта
    pub image_uri: String,
    /// Контрольная сумма off-chain контента (например, SHA-256 hex)
    pub content_hash: String,
}

/// AddAchievement аргументы
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct AddAchievementArgs {
    pub token_id: TokenId,
    pub achievement: String,
}

/// IncreaseScore аргументы
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct IncreaseScoreArgs {
    pub token_id: TokenId,
    pub amount: u64,
}

/// UpdateAchievements аргументы - используется оракулом для batch обновления
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UpdateArgs {
    pub token_id: TokenId,
    pub new_achievements: Vec<String>,
    pub score_increase: u64,
}

/// AddOracle аргументы - добавить авторизованный оракул
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct AddOracleArgs {
    pub oracle: AccountOwner,
}

/// RemoveOracle аргументы - удалить авторизованный оракул
#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct RemoveOracleArgs {
    pub oracle: AccountOwner,
}

/// Все возможные операции контракта
#[derive(Debug, Serialize, Deserialize)]
pub enum PassportOperation {
    Mint(MintArgs),
    AddAchievement(AddAchievementArgs),
    IncreaseScore(IncreaseScoreArgs),
    UpdateAchievements(UpdateArgs),
    AddOracle(AddOracleArgs),
    RemoveOracle(RemoveOracleArgs),
}
