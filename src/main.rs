use std::io;

enum Command {
    Shorten,
    Delete,
}

fn main() {
    println!("Welcome to the J.A.L.S. CLI!");

    // Start action loop
    loop {
        println!("Enter command (shorten <url> or delete <url>, or type 'exit' to quit): ");

        // Declare input variable, inferred string
        let mut input = String::new();

        // Read standard input and mutate into input variable
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Remove trailing whitespace
        let input = input.trim();

        // If input equals 'exit' break action loop
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        // User input is not fixed size,
        // so vectors are preferred since they use heap instead of the stack
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() < 2 {
            // Input parts contains less than 2 elements,
            // so either command or URL is missing, skip loop
            println!("Error: You need to specify both a command and a target URL!");
            continue;
        }

        // Match command from input, to lower case, as string to Command enumeration
        let command = match parts[0].to_lowercase().as_str() {
            "shorten" => Command::Shorten,
            "delete" => Command::Delete,
            _ => {
                // Unsupported command, skip loop
                println!("Error: Unknown command '{}'!", parts[0]);
                continue;
            }
        };

        let url = parts[1];

        // Handle commands
        match command {
            Command::Shorten => {
                println!("Shortening URL: {}", url);

                // Shorten business logic
            },
            Command::Delete => {
                println!("Deleting URL: {}", url);

                // Delete business logic
            }
        }
    }
}

