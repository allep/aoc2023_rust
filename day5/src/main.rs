use day5::Config;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, this is day5!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    println!("Using {config:?}");

    day5::run(config)
}
