mod cmd;
mod game;

extern crate strum;
extern crate strum_macros;
use crate::cmd::{Console, Command};
use crate::game::{Stats, get_numeric_input};

/// Static mutable game stats. Gets unsafely modified in the program.
static mut STATS: Stats = Stats::new();
/// Shared `Console` instance.
static CONSOLE: Console = Console;

fn main() {
    println!("Hi! Let's play a game!\nDefine the max number you'd like to guess:");

    // define upper bound to guess against
    let bound = get_numeric_input(&CONSOLE);
    println!("Game will produce random numbers belonging to [1; {})", &bound);

    // loop until the user terminates the program or program panics
    loop {
        println!("--------\nAwaiting input...");
        // take user input and try to resolve it to a numeric value
        let inp = CONSOLE.take_input();
        match inp.get_numeric() {
            // if the input can be resolved to numeric value,
            // make sure it is strictly positive and modify statistics
            Some(numinp) => {
                if !numinp.is_pos() {
                    println!("Guess should be a positive number, yours is {}", numinp.get_val());
                    continue;
                }

                unsafe {
                    STATS.consume_guess(numinp.derive_guess(&bound))
                }

            },
            // if the input is not a number, try to resolve a command
            None => {
                if let Err(_) = Command::handle_command(inp.get_val().as_str()) {
                    continue
                }
            }
        }
    }

}