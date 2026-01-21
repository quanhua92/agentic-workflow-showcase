mod traits;
mod logic;

#[cfg(test)]
mod tests;

fn notify<T: traits::Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

fn main() {
    let article = logic::News {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Sports Center"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("Attempting to notify about article...");
    notify(&article);

    println!("Article author: {}", article.author);
}
