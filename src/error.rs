use async_graphql::{Error, ErrorExtensions, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PassportError {
    #[error("passport already exists")]
    PassportAlreadyExists,
    #[error("owner already has a passport")]
    OwnerHasPassport,
    #[error("mint requires an authenticated owner")]
    MissingSigner,
    #[error("passport anchored on different chain")]
    WrongChain,
    #[error("only owner may mutate passport")]
    Unauthorized,
    #[error("achievement text exceeds 256 chars")]
    AchievementTooLong,
    #[error("score increment must be positive")]
    ScoreNotPositive,
    #[error("score overflow")]
    ScoreOverflow,
    #[error("URIs and content hash must be non-empty")]
    MissingUris,
    #[error("metadata_uri must be at most 256 characters")]
    MetadataTooLong,
    #[error("image_uri must be at most 256 characters")]
    ImageTooLong,
    #[error("content_hash must be at most 256 characters")]
    ContentHashTooLong,
    #[error("passport not found")]
    PassportNotFound,
}

impl ErrorExtensions for PassportError {
    fn extend(&self) -> Result<Error> {
        let mut error = Error::new(self.to_string());
        let error = error.extend_with(|_, e| e.set("code", self.code()));
        Ok(error)
    }
}

impl PassportError {
    pub fn code(&self) -> &'static str {
        match self {
            PassportError::PassportAlreadyExists => "PASSPORT_EXISTS",
            PassportError::OwnerHasPassport => "OWNER_HAS_PASSPORT",
            PassportError::MissingSigner => "MISSING_SIGNER",
            PassportError::WrongChain => "WRONG_CHAIN",
            PassportError::Unauthorized => "UNAUTHORIZED",
            PassportError::AchievementTooLong => "ACHIEVEMENT_TOO_LONG",
            PassportError::ScoreNotPositive => "SCORE_NOT_POSITIVE",
            PassportError::ScoreOverflow => "SCORE_OVERFLOW",
            PassportError::MissingUris => "MISSING_URIS",
            PassportError::MetadataTooLong => "METADATA_TOO_LONG",
            PassportError::ImageTooLong => "IMAGE_TOO_LONG",
            PassportError::ContentHashTooLong => "CONTENT_HASH_TOO_LONG",
            PassportError::PassportNotFound => "PASSPORT_NOT_FOUND",
        }
    }
}
