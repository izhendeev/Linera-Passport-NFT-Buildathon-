use linera_base::identifiers::AccountOwner;
use linera_sdk::views::{linera_views, MapView, RegisterView, SetView, RootView, ViewStorageContext};

use crate::{Passport, TokenId};

/// Основное состояние приложения Passport NFT
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct PassportState {
    /// Словарь всех паспортов: ключ — `token_id`, значение — `Passport`
    pub passports: MapView<TokenId, Passport>,
    /// Общее количество выпущенных паспортов
    pub total_supply: RegisterView<u64>,
    /// Сопоставление владельцев их паспорту (для ограничения 1 паспорт на владельца)
    pub owner_index: MapView<AccountOwner, TokenId>,
    /// Список авторизованных оракулов (могут обновлять паспорта)
    pub authorized_oracles: SetView<AccountOwner>,
    /// SECURITY FIX: Administrator of the application (can manage oracles)
    /// Set during instantiation to the first signer
    pub admin: RegisterView<Option<AccountOwner>>,
}
