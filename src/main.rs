use clap::{App, Arg};
use colored::*;
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    let matches = App::new("SHA-256 Hash Cracker")
        .version("1.0.0")
        .author("akash2061")
        .about("Cracks SHA-256 hashes and converts strings to hashes")
        .arg(
            Arg::with_name("xhash")
                .short("x")
                .long("hash")
                .value_name("HASH")
                .help("The SHA-256 hash to add to the wordlist")
                .takes_value(true)
                .global(true), // Make this argument global
        )
        .arg(
            Arg::with_name("string")
                .short("s")
                .long("string")
                .value_name("STRING")
                .help("The string to convert to a SHA-256 hash and add to pass_list.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("passfile")
                .short("p")
                .long("passfile")
                .value_name("PASSFILE")
                .help("The path to the password list file")
                .takes_value(true),
        )
        .get_matches();

    let pass_file_path = matches.value_of("passfile").unwrap_or("pass_list.txt");

    if !Path::new(pass_file_path).exists() {
        eprintln!(
            "The specified password list file '{}' does not exist.",
            pass_file_path
        );
        exit(1);
    }
    if let Some(hash) = matches.value_of("xhash") {
        crack_hash(hash, pass_file_path);
    } else if let Some(string) = matches.value_of("string") {
        convert_to_hash(string, pass_file_path);
    } else {
        eprintln!("Please provide either a hash or a string to convert.");
        exit(1);
    }
}

fn crack_hash(hash: &str, pass_file_path: &str) {
    let pass_file = pass_file_path;
    let pass_list = File::open(pass_file).unwrap();
    let reader = BufReader::new(pass_list);

    for (mut line_number, line) in reader.lines().enumerate() {
        let pass = line.unwrap().trim().to_owned();
        let pass_hash = format!("{:x}", Sha256::digest(pass.as_bytes()));
        line_number += 1;
        print!("[{}] ", line_number.to_string().bright_yellow());
        println!("{}: {}", pass.bold().bright_magenta(), pass_hash);

        if pass_hash == hash {
            print!("{}", "Password hash found! Password: ".green());
            println!("{}", pass.bold().bright_blue());
            return;
        }
    }

    println!("{}", "Password hash not found in the password list.".red());
}

fn convert_to_hash(string: &str, pass_file_path: &str) {
    let hash = format!("{:x}", Sha256::digest(string.as_bytes()));

    println!("String: {}", string.bold().bright_green());
    println!("SHA-256 Hash: {}", hash.bold().bright_purple());

    let wordlist_path = pass_file_path;

    if string_exists_in_wordlist(string, wordlist_path) {
        println!(
            "{}",
            "String already exists in the wordlist. Skipping addition.".bright_yellow()
        );
        return;
    }

    // Update pass_list.txt with the new string
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(wordlist_path)
        .expect("Failed to open pass_list.txt");

    writeln!(file, "{}", string).expect("Failed to write to pass_list.txt");

    println!(
        "{}",
        "Updated pass_list.txt with the new string and hash.".bright_yellow()
    );
}

fn string_exists_in_wordlist(string: &str, wordlist_path: &str) -> bool {
    if let Ok(file) = File::open(wordlist_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.trim() == string {
                    return true;
                }
            }
        }
    }
    false
}
