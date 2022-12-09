use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("@tweet"),
        content: String::from("Cool content!"),
        reply: false,
        retweet: false,
    };
    let news_article = NewsArticle {
        headline: String::from("A cool update!"),
        location: String::from("Somewhere"),
        author: String::from("John Appleseed"),
        content: String::from("Something very interesting."),
    };
    println!("{}", summarize_content(&tweet));
    println!("{}", summarize_content(&news_article));
}

fn summarize_content(source: &impl Summary) -> String {
    source.summarize()
}
