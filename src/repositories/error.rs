use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbRepoError {
    #[error("[DbRepoError::SerdeError] {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("[DbRepoError::SqlxError] {0}")]
    SqlxError(#[from] sqlx::Error),

    #[cfg(test)]
    #[error("Dummy error for testing")]
    DummyTestError,
}
