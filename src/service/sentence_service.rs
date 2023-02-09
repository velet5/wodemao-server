use std::{fmt::Display, sync::Arc};

use jieba_rs::Jieba;
use serde::Serialize;

use crate::repo::word_repo::{WordRepo, WordRepoError};

pub struct SentenceService {
    word_repo: Arc<WordRepo>,
    jieba: Arc<Jieba>,
}

impl SentenceService {
    pub fn new(word_repo: Arc<WordRepo>) -> Self
    where
        Self: Sized,
    {
        let jieba = Arc::new(Jieba::new());
        SentenceService { word_repo, jieba }
    }

    pub fn cut(&self, sentence: &str, hmm: bool) -> Vec<Vec<String>> {
        let words = self
            .jieba
            .cut(sentence, hmm)
            .iter()
            .map(|s| s.to_string())
            .collect();

        vec![words]
    }

    pub async fn process_sentence(
        &self,
        sentence: &str,
    ) -> Result<Vec<Vec<WordInfo>>, SentenceServiceError> {
        let sentences = self.cut(sentence, false);
        let mut result = vec![];

        for sentence in sentences {
            let mut sentence_result = vec![];
            for word in sentence {
                let word_info_opt = self.process_word(&word).await?;

                if let Some(word_info) = word_info_opt {
                    sentence_result.push(word_info);
                } else {
                    sentence_result.push(WordInfo {
                        word,
                        pinyin: "".to_string(),
                        translations: vec![],
                    });
                }
            }
            if sentence_result.len() > 0 {
                result.push(sentence_result);
            }
        }

        Ok(result)
    }

    async fn process_word(&self, word: &String) -> Result<Option<WordInfo>, SentenceServiceError> {
        let word_opt = self.word_repo.find(word).await?;

        let word_info_opt = word_opt.map(|word| WordInfo {
            word: word.word,
            pinyin: word.pinyin,
            translations: word.translations,
        });

        Ok(word_info_opt)
    }
}

pub enum SentenceServiceError {
    RepoError(WordRepoError),
}

impl Display for SentenceServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SentenceServiceError::RepoError(e) => write!(f, "RepoError: {}", e),
        }
    }
}

impl From<WordRepoError> for SentenceServiceError {
    fn from(e: WordRepoError) -> Self {
        SentenceServiceError::RepoError(e)
    }
}

#[derive(Serialize)]
pub struct WordInfo {
    pub word: String,
    pub pinyin: String,
    pub translations: Vec<String>,
}
