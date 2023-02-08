use std::sync::Arc;

use jieba_rs::Jieba;

use crate::repo::word_repo::WordRepo;

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

    pub fn cut<'a>(&self, sentence: &'a str, hmm: bool) -> Vec<&'a str> {
        self.jieba.cut(sentence, hmm)
    }
}
