use std::io; // This imports Rust's standard input/output module so we can read keyboard input.

pub fn read_number(prompt: &str) -> f64 { // This declares a public function that receives a string slice prompt and returns an f64 number.
    loop { // This starts an infinite loop so the program can keep asking until the user enters a valid number.
        println!("{}", prompt); // This prints the prompt text given to the function.

        let mut input = String::new(); // This creates a mutable empty String to store what the user types.

        io::stdin() // This accesses the terminal keyboard input.
            .read_line(&mut input) // This gives read_line a mutable reference so it can write the user's text into input.
            .expect("Failed to read input"); // This stops the program if Rust cannot read from the terminal.

        match input.trim().parse::<f64>() { // This trims the text and tries to convert it into an f64 number.
            Ok(number) => return number, // This returns the number if parsing worked.
            Err(_) => println!("Please enter a valid number."), // This prints an error message if parsing failed.
        }
    }
}

pub fn read_choice(prompt: &str) -> u32 { // This declares a public function that receives a prompt and returns a u32 whole number.
    loop { // This starts an infinite loop so the user can retry after invalid input.
        println!("{}", prompt); // This prints the menu prompt.

        let mut input = String::new(); // This creates a mutable String to store the user's menu choice.

        io::stdin() // This accesses keyboard input from the terminal.
            .read_line(&mut input) // This passes a mutable reference so Rust can write the user's input into the String.
            .expect("Failed to read input"); // This stops the program if reading input fails.

        match input.trim().parse::<u32>() { // This trims the input and tries to parse it as an unsigned whole number.
            Ok(choice) => return choice, // This returns the choice if parsing worked.
            Err(_) => println!("Please enter a valid menu number."), // This prints an error if the input was not a whole number.
        }
    }
}

