use std::env;
use std::io::{Result, Write};

use bcrypt;
use clap::Parser;

/// Program to generate a base64 encoded htpasswd file from the environment variables
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Pattern to match the beginning of env variable names.
    /// Uses remaining part of env key as username.
    #[clap(short, long, default_value = "user_")]
    pattern: String,

    /// Do not encode htpasswd with Base64
    #[clap(short, long)]
    no_base64: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pattern = &args.pattern;
    let pattern_length = args.pattern.len();

    let mut account_entries = Vec::new();
    let mut result;

    for (key, value) in env::vars() {
        let lowered_key = key.to_lowercase();

        if lowered_key.starts_with(pattern) {
            let username = &key.get(pattern_length..).unwrap();
            let encrypted_password = bcrypt::hash(value.clone(), 5).unwrap();

            writeln!(&mut account_entries, "{}:{}", username, encrypted_password)?;
        }
    }

    if args.no_base64 == false && account_entries.len() > 0 {
        result = base64::encode(&account_entries);
    } else {
        result = String::from_utf8(account_entries).unwrap();
    }

    println!("{}", result);
    Ok(())
}
