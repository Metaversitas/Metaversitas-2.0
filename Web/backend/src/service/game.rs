use crate::backend::AppState;
use redis::{AsyncCommands, Value};
use std::sync::Arc;
use thiserror::Error;

const REDIS_GAME_VERSION_KEY: &str = "game_version:";
pub struct GameService {
    app_state: Arc<AppState>,
}

impl GameService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        GameService { app_state }
    }

    pub async fn verify_game_version(&self, version: &str) -> Result<(), GameServiceError> {
        let mut redis_con = self
            .app_state
            .redis
            .get_async_connection()
            .await
            .map_err(|_| GameServiceError::RedisError)?;
        let redis_game_version = redis_con
            .get::<String, Value>(format!("{}{}", REDIS_GAME_VERSION_KEY, version))
            .await
            .map_err(|_| GameServiceError::RedisError)?;

        if redis_game_version == Value::Nil {
            let query = sqlx::query!(
                r#"
                select *
                    from public.game
                where version::text = $1;"#,
                version
            )
            .fetch_optional(&self.app_state.database)
            .await
            .map_err(|_| GameServiceError::DatabaseError)?
            .ok_or(GameServiceError::InvalidGameVersion)?;

            if !query.is_live {
                return Err(GameServiceError::OutdatedGameVersion);
            }

            redis_con
                .set::<String, bool, ()>(
                    format!("{}{}", REDIS_GAME_VERSION_KEY, query.version),
                    true,
                )
                .await
                .map_err(|_| GameServiceError::RedisError)?;
            redis_con
                .expire::<String, ()>(
                    format!("{}{}", REDIS_GAME_VERSION_KEY, query.version),
                    time::Duration::hours(1).whole_seconds() as usize,
                )
                .await
                .map_err(|_| GameServiceError::RedisError)?;
            Ok(())
        } else if let Value::Data(_) = redis_game_version {
            Ok(())
        } else {
            Err(GameServiceError::InvalidGameVersion)
        }
    }
}

#[derive(Error, Debug)]
pub enum GameServiceError {
    #[error("Invalid game version.")]
    InvalidGameVersion,
    #[error("Current game version is outdated.")]
    OutdatedGameVersion,
    #[error("Unknown error on Database")]
    DatabaseError,
    #[error("Unknown error on Redis")]
    RedisError,
}
