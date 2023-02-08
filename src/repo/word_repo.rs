use std::sync::Arc;

use sqlx::{Pool, Postgres};

pub struct WordRepo {
    pool: Arc<Pool<Postgres>>,
}

impl WordRepo {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self
    where
        Self: Sized,
    {
        WordRepo { pool }
    }

    async fn find(&self, word: &str) -> Result<Option<Word>, WordRepoError> {
        let word_opt = sqlx::query_as::<_, Word>(
            r#"
            SELECT id, word, source, pinyin, translations
            FROM words
            WHERE word = $1
            "#,
        )
        .bind(word)
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(word_opt)
    }
}

#[derive(sqlx::FromRow)]
pub struct Word {
    id: uuid::Uuid,
    word: String,
    source: WordSource,
    pinyin: String,
    translations: Vec<String>,
}

#[derive(sqlx::Type)]
enum WordSource {
    HSK,
}

enum WordRepoError {
    Sqlx(sqlx::error::Error),
}

impl From<sqlx::error::Error> for WordRepoError {
    fn from(e: sqlx::error::Error) -> Self {
        WordRepoError::Sqlx(e)
    }
}
