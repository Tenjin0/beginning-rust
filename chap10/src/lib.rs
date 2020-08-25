pub mod lifetime {
    use std::fmt::Display;

    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    pub fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }


    pub fn longest3(x: &str) -> &str {
        x
    }

    pub fn longest4<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }


    pub fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display, 
    {
        println!("Annoucement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // pub fn longest5<'a>() -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

}

pub mod trait_samples {
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
    
}
