extern crate clap;
use clap::{App, Arg};

use service;

use std::env;

fn main() -> std::io::Result<()> {
    let matches = App::new("My Program")
        .version("1.0")
        .author("Flynn <ling7334@gmail.com>")
        .about("Deply my program")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("addr")
                .index(1)
                .help("Specify the address listen to")
                .takes_value(false),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap_or("127.0.0.1:8080");

    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    match env::var("RUST_LOG") {
        Ok(val) => println!("RUST_LOG: {:?}", val),
        Err(_) => env::set_var("RUST_LOG", "actix_web=debug,actix_server=info"),
    }
    env_logger::init();

    service::run(address.to_string())
}
