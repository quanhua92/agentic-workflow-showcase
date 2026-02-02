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

#[test]
fn test_summarize_multiple_calls() {
    let news = News {
        headline: String::from("Multiple Calls Test"),
        author: String::from("Test Author"),
        content: String::from("Test content..."),
    };

    // Verify we can call summarize multiple times on the same instance
    let summary1 = news.summarize();
    let summary2 = news.summarize();
    let summary3 = news.summarize();

    assert_eq!(summary1, "Multiple Calls Test");
    assert_eq!(summary2, "Multiple Calls Test");
    assert_eq!(summary3, "Multiple Calls Test");
}

#[test]
fn test_original_value_persists_after_notify() {
    fn notify<T: Summary>(item: &T) {
        let _ = item.summarize(); // Consume the summary but not the item
    }

    let news = News {
        headline: String::from("Ownership Test"),
        author: String::from("Ownership Author"),
        content: String::from("Ownership content..."),
    };

    // Call notify with a reference
    notify(&news);

    // Verify we can still access the original value
    assert_eq!(news.headline, "Ownership Test");
    assert_eq!(news.author, "Ownership Author");
    assert_eq!(news.content, "Ownership content...");

    // Verify we can still call summarize
    let summary = news.summarize();
    assert_eq!(summary, "Ownership Test");
}

#[test]
fn test_empty_headline() {
    let news = News {
        headline: String::from(""),
        author: String::from("Empty Headline Author"),
        content: String::from("Content for empty headline..."),
    };

    let summary = news.summarize();
    assert_eq!(summary, "");
    assert!(summary.is_empty());
}

#[test]
fn test_long_headline() {
    let long_headline = "A".repeat(1000);
    let news = News {
        headline: long_headline.clone(),
        author: String::from("Long Headline Author"),
        content: String::from("Long headline content..."),
    };

    let summary = news.summarize();
    assert_eq!(summary.len(), 1000);
    assert_eq!(summary, long_headline);
}

