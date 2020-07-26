mod open_file;
mod others;

fn main() {
    // open_file::main();
    let s = open_file::read_username_from_file().unwrap();
    println!("{}", s);
    others::main();

}
