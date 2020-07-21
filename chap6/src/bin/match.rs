#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}


#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                None => {
                    println!("Quarter with no state");
                },
                Some(x) => {
                    println!("State quarter from {:?}", x);
                },
            }
            25
        },
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(None)));
    println!("{}", value_in_cents(Coin::Quarter(Some(UsState::Alabama))))
}
