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
fn test_notify_with_reference() {
    let news = News {
        headline: String::from("Test Headline"),
        author: String::from("Test Author"),
        content: String::from("Test Content"),
    };

    // This should work without moving
    super::notify(&news);

    // Verify we can still access news fields
    assert_eq!(news.author, "Test Author");
}

#[test]
fn test_multiple_notifications() {
    let news = News {
        headline: String::from("Multiple Headline"),
        author: String::from("Multiple Author"),
        content: String::from("Multiple Content"),
    };

    // Should be able to notify multiple times with reference
    super::notify(&news);
    super::notify(&news);
}

