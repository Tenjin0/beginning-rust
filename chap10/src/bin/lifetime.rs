use chap10::lifetime;

fn main() {

    let str1 = String::from("abcd");
    {
        let str2 = "uvwxyz";
        let mut result = lifetime::longest(str1.as_str(), str2);
        println!("{}", result);
        result = lifetime::longest2(str1.as_str(), str2);
        println!("{}", result);

    }

}
