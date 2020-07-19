fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let  a = [ 10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element)
    }

    for index in (1.. a.len()).step_by(2) {
        println!("The value is: {}", a[index])

    }
}
