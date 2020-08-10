use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
   
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(&config) {
        Err(e) => {
            println!("Application error: {} (file: {})", e, config.filename);
            process::exit(1);
        },
        Ok(contents) => {
            println!("{}", contents)
        }
    }
       
   
}
