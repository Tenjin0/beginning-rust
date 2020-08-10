
pub struct NewsArticle {

    pub healine: String,
    pub location: String,
    pub author: Person,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

pub struct Person {
    firstname: String,
    lastname: String,
    age: u8
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more ...")
    }
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.healine, self.author.full_name(), self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
