use std::io;
use std::collections::HashMap;
use sha2::{Digest, Sha256};

enum Command {
    Shorten,
    Get,
    Delete,
    List,
}

fn handle_input() -> String {
    // Declare input variable, inferred string
    let mut input = String::new();

    // Read standard input and mutate into input variable
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Return String from input with trailing whitespace trimmed
    String::from(input.trim())
}

fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new(); // New hasher
    hasher.update(url); // Update with URL to hash
    let hash_result = hasher.finalize(); // Create hash

    // Take first 8 characters and convert to hex string, return result
    hash_result.iter()
        .take(4) // Take the first 4 bytes (8 hex characters)
        .map(|b| format!("{:02x}", b)) // Convert bytes to hex format
        .collect::<String>()
}

fn main() {
    // Set up the hash map for URL storage in memory
    let mut url_mapping: HashMap<String, String> = HashMap::new();

    println!("Welcome to the J.A.L.S. CLI!");
    println!();
    println!("Available commands:");
    println!("  - shorten <url> - shortens a URL");
    println!("  - get <short_url> - get original URL by short URL");
    println!("  - delete <short_url> - deletes an entry by short URL");
    println!("  - list - lists all URL entries");
    println!("  - exit - quits the program");
    println!();
    // Start action loop
    loop {

        println!("Enter command: ");

        let input = handle_input();

        // If command equals 'exit' break action loop
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        // User input is not fixed size,
        // so vectors are preferred since they use heap instead of the stack
        let parts: Vec<&str> = input.split_whitespace().collect();

        let input_command = &parts[0].to_lowercase(); // Separate command
        let input_args = &parts[1..]; // Separate arguments after command

        // Match command from input, to lower case, as string to Command enumeration
        let command = match input_command.as_str() {
            "shorten" => Command::Shorten, // If command is 'shorten', assign enum Shorten, etc...
            "get" => Command::Get,
            "delete" => Command::Delete,
            "list" => Command::List,
            _ => {
                // Unsupported command, skip loop
                println!("Error: Unknown command '{}'!", input_command);
                continue;
            }
        };

        // Handle commands
        match command {
            Command::Shorten => {
                if let Some(url) = input_args.get(0) {
                    if !url.is_empty() {
                        println!("Shortening URL: {}", url);

                        // Logic
                        let short_url = hash_url(url); // Generate short URL
                        url_mapping.insert(short_url.clone(), url.to_string()); // Insert into memory

                        println!("Generated short url: {}", short_url);

                        continue;
                    } else {
                        println!("Error: You need to specify a target URL argument!");
                        continue;
                    }
                } else {
                    println!("Error: No argument specified! You need to specify target URL!");
                    continue;
                }
            },
            Command::Get => {
                if let Some(short_url) = input_args.get(0) {
                    if !short_url.is_empty() {
                        println!("Getting short URL: {}", short_url);

                        if let Some(url) = url_mapping.get(*short_url) {
                            println!("Full URL: {}", url);
                            continue;
                        } else {
                            println!("Entry for `{}` not found!", short_url);
                            continue;
                        }
                    } else {
                        println!("Error: You need to specify a target short URL argument!");
                        continue;
                    }
                } else {
                    println!("Error: No argument specified! You need to specify target short URL!");
                    continue;
                }
            },
            Command::Delete => {
                if let Some(url) = input_args.get(0) {
                    if !url.is_empty() {
                        println!("Deleting URL: {}", url);

                        // Logic

                        continue;
                    } else {
                        println!("Error: You need to specify a target URL argument!");
                        continue;
                    }
                } else {
                    println!("Error: No argument specified! You need to specify target URL!");
                    continue;
                }
            },
            Command::List => {
                println!("Listing all shortened URLs...");

                // Logic

                continue;
            }
        }
    }
}

