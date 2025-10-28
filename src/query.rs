use std::sync::Arc;

use async_graphql::{Error, Object, Result};

use passport_nft::{Passport, PassportState, TokenId};

pub struct QueryRoot {
    pub state: Arc<PassportState>,
}

#[Object]
impl QueryRoot {
    async fn total_supply(&self) -> Result<u64> {
        Ok(*self.state.total_supply.get())
    }

    async fn passport(&self, token_id: TokenId) -> Result<Option<Passport>> {
        self.state
            .passports
            .get(&token_id)
            .await
            .map_err(|e| Error::new(format!("failed to read passport: {e}")))
    }

    async fn all_passports(&self) -> Result<Vec<Passport>> {
        let mut passports = Vec::new();
        self.state
            .passports
            .for_each_index_value(|_, passport| {
                passports.push(passport.into_owned());
                Ok(())
            })
            .await
            .map_err(|e| Error::new(format!("failed to iterate passports: {e}")))?;
        Ok(passports)
    }
}
