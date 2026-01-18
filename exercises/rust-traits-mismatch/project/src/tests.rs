use crate::logic::News;
use crate::traits::Summary;

#[test]
fn test_summarize_returns_headline() {
    let news = News {
        headline: String::from("Breaking: Rust becomes #1 language"),
        author: String::from("Tech Daily"),
        content: String::from("Rust has overtaken all other languages..."),
    };

    let summary = news.summarize();
    assert_eq!(summary, "Breaking: Rust becomes #1 language");
}

