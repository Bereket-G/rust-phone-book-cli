extern crate core;
use getopts::{Matches, Options};
use std::{env};
use crate::models::models::PhoneBook;
mod models;

const DB_FILE_NAME: &'static str = "phonebook.json";

fn print_usage(opts: Options) {
    let brief = format!("Usage: FILE [options]");
    print!("{}", opts.usage(&brief));
}


fn initialize_cli_options() -> (Matches, Options) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("a", "add", "add new contact");
    opts.optflag("r", "read", "get contact");
    opts.optopt("n", "name", "contract name", "Name");
    opts.optopt("p", "phone", "phone number", "Phone Number");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    (matches, opts)
}

fn main() {
    let (cli_command_matches, opts) = initialize_cli_options();

    let mut phone_book = PhoneBook::new(DB_FILE_NAME);

    if cli_command_matches.opt_present("a") {
        let name = cli_command_matches.opt_str("n").unwrap();
        let phone = cli_command_matches.opt_str("p").unwrap();
        phone_book.add_contact(name, phone);
        return;
    }

    if cli_command_matches.opt_present("r") {
        let name = cli_command_matches.opt_str("n").unwrap();
        let found_contact = phone_book.get_contact(name);
        println!("Found {:?}", found_contact);
    }


    if cli_command_matches.free.is_empty() {
        print_usage(opts);
        return;
    };
}
