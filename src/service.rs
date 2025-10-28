#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;

use async_graphql::{Context, EmptySubscription, Object, Request, Response, Schema};
use linera_base::identifiers::AccountOwner;
use linera_sdk::{
    linera_base_types::WithServiceAbi, service, views::View, Service, ServiceRuntime,
};

mod query;

use passport_nft::{
    AddAchievementArgs, AddOracleArgs, IncreaseScoreArgs, MintArgs, PassportNftAbi,
    PassportOperation, PassportState, RemoveOracleArgs, TokenId, UpdateArgs,
};

pub struct PassportService {
    state: Arc<PassportState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

service!(PassportService);

impl WithServiceAbi for PassportService {
    type Abi = PassportNftAbi;
}

impl Service for PassportService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = match PassportState::load(runtime.root_view_storage_context()).await {
            Ok(state) => state,
            Err(error) => {
                log::error!("Failed to load state in service: {error:#}");
                // For service initialization, we must panic as there's no way to return an error
                // This is acceptable behavior per Linera SDK design
                panic!("Critical: Unable to initialize service state: {error:#}");
            }
        };
        PassportService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            query::QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot,
            EmptySubscription,
        )
        .data(self.runtime.clone())
        .finish();
        schema.execute(request).await
    }
}

struct MutationRoot;

/// Helper function to safely extract runtime from GraphQL context
fn get_runtime(ctx: &Context<'_>) -> Option<Arc<ServiceRuntime<PassportService>>> {
    match ctx.data::<Arc<ServiceRuntime<PassportService>>>() {
        Ok(runtime) => Some(runtime.clone()),
        Err(e) => {
            log::error!("Failed to get runtime from context: {e:?}");
            None
        }
    }
}

#[Object]
impl MutationRoot {
    async fn mint(
        &self,
        ctx: &Context<'_>,
        token_id: TokenId,
        metadata_uri: String,
        image_uri: String,
        content_hash: String,
    ) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::Mint(MintArgs {
                token_id,
                metadata_uri,
                image_uri,
                content_hash,
            });
            runtime.schedule_operation(&operation);
        }
        []
    }

    async fn add_achievement(
        &self,
        ctx: &Context<'_>,
        token_id: TokenId,
        achievement: String,
    ) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::AddAchievement(AddAchievementArgs {
                token_id,
                achievement,
            });
            runtime.schedule_operation(&operation);
        }
        []
    }

    async fn increase_score(&self, ctx: &Context<'_>, token_id: TokenId, amount: u64) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::IncreaseScore(IncreaseScoreArgs { token_id, amount });
            runtime.schedule_operation(&operation);
        }
        []
    }

    async fn update_achievements(
        &self,
        ctx: &Context<'_>,
        token_id: TokenId,
        new_achievements: Vec<String>,
        score_increase: u64,
    ) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::UpdateAchievements(UpdateArgs {
                token_id,
                new_achievements,
                score_increase,
            });
            runtime.schedule_operation(&operation);
        }
        []
    }

    async fn add_oracle(&self, ctx: &Context<'_>, oracle: AccountOwner) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::AddOracle(AddOracleArgs { oracle });
            runtime.schedule_operation(&operation);
        }
        []
    }

    async fn remove_oracle(&self, ctx: &Context<'_>, oracle: AccountOwner) -> [u8; 0] {
        if let Some(runtime) = get_runtime(ctx) {
            let operation = PassportOperation::RemoveOracle(RemoveOracleArgs { oracle });
            runtime.schedule_operation(&operation);
        }
        []
    }
}
