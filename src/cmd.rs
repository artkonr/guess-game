use std::io;
use crate::game::{Guess, STATS_PATH_STRING};
use std::process::exit;
use strum::{VariantNames, ParseError};
use strum_macros::EnumVariantNames;
use crate::{STATS};
use std::str::FromStr;
use crate::files::Exporter;

/// Convenience structure which takes wraps `stdin` calls in
///  a friendly manner and returns wrapper structures.
pub struct Console;

impl Console {

    /// Takes user input from `stdin` and does not assume that
    ///  the provided input is anything different from `String`.
    ///
    /// Returns the value wrapped in the `StringInput`.
    pub fn take_input(&self) -> StringInput {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)
            .expect("Failed to obtain input");
        StringInput::new(String::from(buf.trim()))
    }

}

/// Wrapper around numeric input.
pub struct NumInput {
    val: i32
}

impl NumInput {

    /// Factory method. Creates a new instance of `NumInput` from
    ///  a numeric `i32` value.
    pub fn new(n: i32) -> NumInput {
        NumInput { val: n }
    }

    /// Determines whether the input is in (`0`; `i32`).
    ///  If the numeric input is a positive value, returns
    ///  `true`; else - `false`.
    pub fn is_pos(&self) -> bool {
        self.val > 0
    }

    /// Creates a `Guess` instance from this input with
    ///  the desired upper `bound` provided.
    pub fn derive_guess(&self, bound: &u32) -> Guess {
        Guess::new(self.val as u32, bound)
    }

    /// Getter of the pointer to the value supplied as input.
    pub fn get_val(&self) -> &i32 {
        &self.val
    }

}

/// Wrapper around simple `String` input.
pub struct StringInput {
    val: String
}

impl StringInput {

    /// Factory mehtod. Creates a new instance of `StringInput`
    ///  with a `String` values provided as argument.
    pub fn new(s: String) -> StringInput {
        StringInput { val: s }
    }

    /// Getter of the pointer to the value supplied as input.
    pub fn get_val(&self) -> &String {
        &self.val
    }

    /// Tries to resolve the input into `NumInput`. Returns
    ///  `Option` which signifies, whether the input could
    ///  be resolved or not.
    pub fn get_numeric(&self) -> Option<NumInput> {
        match self.get_val().parse() {
            Ok(num) => Option::Some(NumInput::new(num)),
            Err(_) => Option::None
        }
    }

}

/// An enum which contains all the commands the game
///  accepts in the console as straightforward user input.
#[derive(EnumVariantNames)]
pub enum Command {
    Quit,
    Show,
    Hi,
    Author,
    Version,
    Kill,
    Help,
    Restart,
    Cheat,
    Save,
    Json,
}

impl FromStr for Command {
    type Err = ParseError;

    /// Recognizes a `Command` from a string slice.
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "quit" => Ok(Command::Quit),
            "kill" => Ok(Command::Kill),
            "show" => Ok(Command::Show),
            "hi" => Ok(Command::Hi),
            "author" => Ok(Command::Author),
            "version" => Ok(Command::Version),
            "help" => Ok(Command::Help),
            "restart" => Ok(Command::Restart),
            "cheat" => Ok(Command::Cheat),
            "save" => Ok(Command::Save),
            "json" => Ok(Command::Json),
            _ => Err(ParseError::VariantNotFound)
        }
    }

}

impl Command {

    /// Static resolver method. Takes a string slice, attempts
    ///  to find a `Command` associated with this slice value
    ///  and tries to execute it.
    ///
    /// If a `Command` can be resolved from the string slice,
    ///  also returns its name.
    ///
    /// If a `Command` cannot be resolved, returns error and
    ///  notifies the user in the console.
    pub fn handle_command(str: &str) -> Result<&str, ParseError> {
        match find(str) {
            Some(cmd) => {
                handle(&cmd);
                Ok(str)
            }
            None => {
                println!("Command {} not recognized. Available commands: {}",
                         if str.is_empty() { "<empty>" } else { str },
                         Command::VARIANTS.join(","));
                Err(ParseError::VariantNotFound)
            }
        }
    }

}

/// Attempts to find a `Command` associated with
///  the name equal to the supplied string slice.
///  Wraps the result in `Option`.
fn find(str: &str) -> Option<Command> {
    match Command::from_str(str) {
        Ok(cmd) => Some(cmd),
        Err(_) => None
    }
}

/// Exhaustively assigns a functional action to every `Command`.
fn handle(cmd: &Command) {
    match cmd {
        Command::Quit => {
            println!("Quitting...");
            println!("Your final stats are:\n{}", unsafe { &STATS });
            exit(0)
        },
        Command::Show => {
            println!("Your intermediate stats are:\n{}", unsafe { &STATS })
        }
        Command::Hi => {
            println!("Hi, {}!", whoami::realname())
        },
        Command::Author => {
            println!("Author artkonr (https://github.com/artkonr) says hi!")
        },
        Command::Version => {
            println!("Current version: {}", env!("CARGO_PKG_VERSION"))
        },
        Command::Kill => {
            println!("Quitting...");
            exit(0);
        },
        Command::Help => println!("Available commands:\n{}", get_command_list()),
        Command::Restart => {
            handle(&Command::Show);
        	println!("Restarting game...");
        	unsafe { &STATS.reset() };
        },
        Command::Cheat => {
        	println!("Cheat! Cheat! Cheat!");
        	unsafe { &STATS.add_wins(10) };
        },
        Command::Save => {
            println!("Saving statistics to file...");
            match unsafe { &STATS.serialize() } {
                Ok(ser) => {
                    match Exporter::new(STATS_PATH_STRING).export(ser) {
                        Ok(_) => println!("Statistics were saved to {}", STATS_PATH_STRING),
                        Err(err) => println!("Failed to save statistics: {}", err)
                    }
                }
                Err(err) => println!("Failed to serialize stats data: {}", err)
            }
        },
        Command::Json =>
            println!("Your intermediate stats are:\n{}",
                     unsafe { &STATS.serialize().unwrap() })
    }
}

/// Returns a nicely formatted list of available commands.
fn get_command_list() -> String {
	let mut ls = String::new();
	for item in Command::VARIANTS.iter() {
		ls.push_str(format!(" -> {}\n", item.to_ascii_lowercase()).as_str())
	}
	ls
}
