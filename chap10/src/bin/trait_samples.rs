use chap10::trait_samples::{Summary, Tweet};

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());
}
