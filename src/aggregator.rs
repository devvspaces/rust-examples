pub trait Summary {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl NewsArticle {
    pub fn new(headline: &str, location: &str, author: &str, content: &str) -> Self {
        Self {
            headline: headline.to_string(),
            location: location.to_string(),
            author: author.to_string(),
            content: content.to_string()
        }
    }
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl SocialPost {
    pub fn new(username: &str, content: &str) -> Self {
        Self {
            username: username.to_string(),
            content: content.to_string(),
            reply: false,
            repost: false,
        }
    }
}

impl Summary for SocialPost {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}