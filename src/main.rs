use std::{env, fs::File, io::{self, Write}};

use dotenv::dotenv;
use onion_link::Storage;
use text_io::read;
use yansi::{Color, Paint};

mod onion_link;
mod error;
mod cli;

#[cfg(test)]
mod tests;

fn main() -> Result<(), crate::error::Error> {
    dotenv().ok();
    let path = env::var("STORAGE_FILE_PATH").unwrap();
    let mut file = File::create(path).unwrap();
    let storage = match Storage::try_from(&mut file) {
        Ok(st) => st,
        Err(_) => Storage::default(),
    };
    loop {
        print!("{} ", ">".bold().fg(Color::Blue));
        io::stdout().flush().unwrap();

        let input: String = read!();

        match input.to_lowercase().as_str() {
            "search" => {
                let mut pattern = String::new();
                let mut ch;
                loop {
                    ch = read!();
                    if ch == '\n' {
                        break;
                    }
                    pattern.push(ch);

                    let search_results = storage.search_entry(&pattern);
                    for entry in search_results {
                        println!("{}", entry);
                    }

                }
            }
            "exit" | "quit" | "q" => break,
            _ => println!("Invalid command"),
        }
    }
    Ok(())
}
