use std::env;
use std::process;
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, App};

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use colored::*;

// function to set the value to clipboard
fn copy_to_clipboard(value: String, clear: Option<u64>) {
    // initiate OS clipboard context
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    // set the contents of the clipboard as the value
    clipboard.set_contents(value.to_owned()).unwrap();

    // verify the clipboard copy
    if clipboard.get_contents().unwrap() == value {
        // successfully exit after setting the value
        println!(
            "{}{}{} {}",
            "[".bold().white(),
            "✓".bold().green(),
            "]".bold().white(),
            "value has been secretly copied to clipboard.".green()
        );

        // handle clearing the clipboard
        if clear != None {
            let seconds = clear.unwrap();

            println!(
                "{}{}{} {}",
                "[".bold().white(),
                "i".bold().yellow(),
                "]".bold().white(),
                format!("value will be cleared from clipboard after {} seconds.", seconds).cyan()
            );
            
            // wait for given `seconds`
            thread::sleep(time::Duration::from_secs(seconds));

            // clear the clipboard after the timeout
            clipboard.set_contents("".to_owned()).unwrap();

            // verify clearance
            if clipboard.get_contents().unwrap() != "" {
                println!(
                    "{}{}{} {}",
                    "[".bold().white(),
                    "!".bold().red(),
                    "]".bold().white(),
                    "failed to clear clipboard.".red()
                );
            }
        }

        process::exit(0)
    } else {
        // gracefully exit with `code 1` if the clipboard copy failed
        println!(
            "{}{}{} {}",
            "[".bold().white(),
            "!".bold().red(),
            "]".bold().white(),
            "failed to set value.".red()
        );

        process::exit(1)
    }
}

fn main() -> std::io::Result<()> {
    // CLI app interface and parsed arguments
    let cli_matches = App::new("seclip")
                        .version("1.0.0")
                        .author("Mufeed VH <contact@mufeedvh.com>")
                        .about("A CLI utility to secretly copy secrets to clipboard.")
                        .arg(Arg::with_name("clear")
                            .short("c")
                            .long("clear")
                            .value_name("SECONDS")
                            .help("Sets a timeout (in seconds) to clear the value from clipboard and exit. (default: 20)")
                            .takes_value(true))
                        .arg(Arg::with_name("KEY")
                            .help("Sets the input key (file or environment variable).")
                            .required(true)
                            .index(1))
                        .get_matches();

    // value file/variable key from the user
    let key: &str = cli_matches.value_of("KEY").unwrap();

    // get the clear timeout seconds
    let clear_opt: Option<u64>;
    if cli_matches.is_present("clear") {
        // get the timeout seconds from the user
        let clear_value = cli_matches.value_of("clear").unwrap();

        // default to 20 seconds if the input is invalid
        let timeout_seconds = clear_value.parse::<u64>().unwrap_or(20);
        clear_opt = Some(timeout_seconds)
    } else {
        // set to `None` if the clear option is not specified
        clear_opt = None
    }

    // search for file
    if Path::new(key).exists() {
        // read the file contents
        let file = File::open(key)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        // remove leading and trailing whitespace from file contents
        contents = contents.trim().to_string();

        // pass the contents of the file to save to clipboard
        copy_to_clipboard(contents, clear_opt)
    }

    // get environment variable
    match env::var(key) {
        Ok(value) => copy_to_clipboard(value, clear_opt),
        Err(_e) => {
            println!(
                "{}{}{} {}",
                "[".bold().white(),
                "!".bold().red(),
                "]".bold().white(),
                "value does not exist.".red()
            );

            process::exit(1)
        },
    }

    Ok(())
}
