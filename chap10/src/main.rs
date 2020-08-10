use crate::trait_samples::Summary;

mod trait_samples;
mod lifetime;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut found = list[0];
    for &item in list {
        if item > found {
            found = item;
        }
    }
    return found;
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Point<T, U> {
    x: T,
    y: U
}

#[allow(dead_code)]
impl <T, U>Point<T, U> {

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, p2: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: p2.y}
    }

}

#[allow(dead_code)]
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    println!("Hello, world!");

    let p1 = Point { x: 4, y: 10};
    let p2 = Point { x: 4.5, y: 10};
    let p3 = p1.mixup(p2);

    println!("{:?}, {:?}, {:?}", p1, p2, p3);

    let v = vec![34, 50, 25, 100, 65];

    println!("{}", largest(&v));

    let tweet = trait_samples::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());

    let str1 = String::from("abcd");
    {
        let str2 = "uvwxyz";
        let mut result = lifetime::longest(str1.as_str(), str2);
        println!("{}", result);
        result = lifetime::longest2(str1.as_str(), str2);
        println!("{}", result);

    }

}  
