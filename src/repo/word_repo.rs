use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
trait WordRepo {
    async fn find(&self, word: &str) -> Result<Option<Word>, WordRepoError>;
}

enum WordRepoError {
    Sqlx(sqlx::error::Error),
}

impl From<sqlx::error::Error> for WordRepoError {
    fn from(e: sqlx::error::Error) -> Self {
        WordRepoError::Sqlx(e)
    }
}

struct WordRepoImpl<'a> {
    pool: &'a Pool<Postgres>,
}

#[async_trait]
impl WordRepo for WordRepoImpl<'_> {
    async fn find(&self, word: &str) -> Result<Option<Word>, WordRepoError> {
        let word_opt = sqlx::query_as::<_, Word>(
            r#"
            SELECT id, word, source, pinyin, translations
            FROM words
            WHERE word = $1
            "#,
        )
        .bind(word)
        .fetch_optional(self.pool)
        .await?;

        Ok(word_opt)
    }
}

#[derive(sqlx::FromRow)]
struct Word {
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
