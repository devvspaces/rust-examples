use example::aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let news = NewsArticle::new("Meteorite Shower", "London UK", "Eli Kun", "Wonderful stones fall from the sky");
    let post = SocialPost::new("shower.1000", "Wonderful stones fall from the sky");

    println!("News summary: {}", news.summary());
    println!("Post summary: {}", post.summary());
}