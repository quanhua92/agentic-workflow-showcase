use crate::traits::Summary;

pub struct News {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        self.headline.clone()
    }
}
