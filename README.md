# Guessing Game in Rust

Welcome to the **Guessing Game**, a simple command-line project written in Rust! In this game, the computer generates a random number, and the player has to guess the correct number by entering their guesses. The game will guide the player by indicating whether the guess is too low, too high, or correct. It’s a fun and interactive way to practice Rust programming concepts such as variables, input/output handling, loops, and error handling. The project is great for beginners who are looking to explore the basics of Rust and enjoy coding games!

## Features
- Generates a random number for each game session
- Provides feedback for each guess (too high, too low, or correct)
- Repeats until the player guesses the correct number
- Simple and interactive command-line interface

## What I Learned
Working on this project has helped me gain hands-on experience with the following Rust concepts:
- **Standard Library**: Used modules like `std::io` for input/output operations and `std::cmp` for comparing values.
- **Crates**: Learned how to include external crates by adding them to `Cargo.toml`. For example, used the `rand` crate to generate random numbers.
- **Variables**: Declared variables using `let` and made them mutable with `mut` to allow value changes.
- **Shadowing**: Practiced shadowing variables to change their type or value later in the program.
- **Type Conversion**: Converted user input from string to numbers (e.g., `u32`) using `.parse()` and handled extra input like `\n` with `.trim()`.
- **References**: Used references (`&` and `&mut`) to pass variables between functions without ownership issues.
- **Comparisons**: Applied the `cmp` method to compare two variables, and used the `Ordering` enum to handle conditions like `Less`, `Greater`, and `Equal`.
- **Loops**: Repeatedly prompted for input using a loop until the correct guess was made, and exited the loop using `break` when the guess was correct.
- **Error Handling**: Managed potential errors with `.expect()`, and explored using `Result` types (`Ok` and `Err`) for error checking and handling.

## Getting Started

To run the guessing game, make sure you have Rust installed. Clone this repository, navigate to the project folder, and execute:

```bash
cargo run
