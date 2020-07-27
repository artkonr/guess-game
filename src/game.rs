use std::ops::AddAssign;
use std::fmt::{Display, Formatter};
use std::cmp::{Ordering};
use rand::Rng;
use crate::cmd::Console;

/// Trivial struct which holds data about the running game:
/// basically, has 3 fields:
/// * `total` :: number of games played
/// * `won` :: number of games won
/// * `lost` :: number of games lost
pub struct Stats {
    total: u32,
    won: u32,
    lost: u32
}

impl Stats {

    /// Factory method. Creates a new `Stats` object with
    ///  all fields initialized to `0`.
    pub const fn new() -> Stats {
        Stats { total: 0, won: 0, lost: 0 }
    }

    /// Accepts a `Guess` instance to derive the result
    ///  of the guess and adjust the `Stats` under the
    ///  pointer accordingly.
    pub fn consume_guess(&mut self, guess: Guess) {
        self.compare_and_augment(guess.get_val(), guess.get_expectation())
    }

	/// Resets all fields in a `Stats` instance to 0.
    pub fn reset(&mut self) {
    	self.total = 0;
    	self.won = 0;
    	self.lost = 0;
    }

    /// Compares 2 integer values to alter the game statistics
    ///     depending on the result of the comparison drawn.
    ///
    /// Uses `Ordering` to draw the comparison. Also notifies
    ///     the player, whether he/she guessed or not.
    fn compare_and_augment(&mut self, this: &u32, that: &u32) {
        match this.cmp(that) {
            Ordering::Less => {
                println!(">>> A bit too low: yours {}, expected {}", this, that);
                self.track_lose();
            },
            Ordering::Equal => {
                println!(">>> Whoa! Who's the lucky mack?!");
                self.track_win();
            },
            Ordering::Greater => {
                println!(">>> Now this went too far: yours {}, expected {}", this, that);
                self.track_lose();
            }
        }
    }

    /// Mutates the `Stats` struct belonging to the pointer
    ///     to illustrate that a game has been played and
    ///     resulted in **success**.
    fn track_win(&mut self) {
        self.total.add_assign(1);
        self.won.add_assign(1);
    }

    /// Mutates the `Stats` struct belonging to the pointer
    ///     to illustrate that a game has been played and
    ///     resulted in **failure**.
    fn track_lose(&mut self) {
        self.total.add_assign(1);
        self.lost.add_assign(1);
    }

    /// Calculates winning ratio (i.e. how often has the player
    ///     won in relation to the total number of games) as **float32**.
    ///
    /// If no game has yet been played when the function is invoked,
    ///     returns `0`.
    ///
    /// Always returns the relation multiplied by `100` to then be printed
    ///     as human-readable percentage value.
    ///
    /// Say, a `Stats` element with `won` set to `4` and `total` set to `10`. Then,
    ///     the resulting value will be:
    /// ```
    /// won / total * 100 = 4 / 10 * 100 = 40
    /// ```
    fn calc_win_ratio(&self) -> f32 {
        if self.total == 0 { return 0f32 }
        self.won as f32 / self.total as f32 * 100f32
    }

    /// Calculates win-lose ratio (i.e. how big big is the amount of successful
    ///     games in relation to the unsuccessful ones) as **float32**. Null-div safe.
    ///
    /// Always returns the relation multiplied by `100` to then be printed
    ///     as human-readable percentage value.
    ///
    /// Say, a `Stats` element with `won` set to `4` and `lost` set to `10`. Then,
    ///     the resulting value will be:
    /// ```
    /// won / lost * 100 = 4 / 10 * 100 = 40
    /// ```
    fn calc_win_lose_ratio(&self) -> f32 {
        if self.lost == 0 { return 100f32 }
        self.won as f32 / self.lost as f32 * 100f32
    }

}

impl Display for Stats {
    /// Prints a table-like view of a `Stats` instance. Output looks like:
    ///
    ///```
    /// --> games played   0
    /// --> games won      0
    /// --> win ratio      0%
    /// --> win-lose ratio 100%
    ///```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arrow = "--> ";
        let winrat = self.calc_win_ratio();
        let winloserat = self.calc_win_lose_ratio();

        let statsls = vec![
            StatsItem::new("games played", self.total.to_string()),
            StatsItem::new("games won", self.won.to_string()),
            StatsItem::new("win ratio", format!("{}{}", &winrat, '%')),
            StatsItem::new("win-lose ratio", format!("{}{}", &winloserat, '%'))
        ];

        let mut maxlen= 0;
        for item in statsls.iter() {
            maxlen = if maxlen < item.namelen { item.namelen } else { maxlen }
        }

        let mut str = String::new();
        for item in statsls.iter() {
            str.push_str(arrow);
            str.push_str(item.name);
            str.push_str(" ".repeat(maxlen - item.namelen + 1).as_str());
            str.push_str(item.val.as_str());
            str.push_str("\n")
        }

        write!(f, "{}", str)
    }
}

/// Local structure suited to contain some aggregated statistics
///  value with its description. Also contains aux data to render
///  `Stats` view in a beautiful table.
struct StatsItem {
    name: &'static str,
    val: String,
    namelen: usize
}

impl StatsItem {

    /// Factory method. Creates a `StatsItem` instance with
    ///  the name of the statistics value as a `str` slice.
    fn new(name: &'static str, val: String) -> StatsItem {
        StatsItem { name, val, namelen: name.len() }
    }

}

/// A structure which represents a result of a random number guess.
///  Normally contains guessed value and expectation. Both are **strictly** `u32`.
pub struct Guess {
    val: u32,
    exp: u32
}

impl Guess {

    /// Factory method. Creates a new `Guess` instance.
    ///  Consumes guessed value from the argument. Takes
    ///  an upper `bound` `u32` value to generate random figure
    ///  to be set as expected value, i.e. resulting rand lies
    ///  in [1; `bound`).
    pub fn new(val: u32, bound: &u32) -> Guess {
        Guess { val, exp: rand::thread_rng().gen_range(1u32, bound) }
    }

    /// Getter of the guessed value.
    pub fn get_val(&self) -> &u32 {
        &self.val
    }

    /// Getter of the expected value.
    pub fn get_expectation(&self) -> &u32 {
        &self.exp
    }

}

/// A convenience function which loops while taking user
///  input from `Console`. The loop breaks with the result
///  when the user provides a non-negative `u32` value.
pub fn get_numeric_input(console: &Console) -> u32 {
    let mut input = console.take_num_input();
    loop {
        match &input {
            Some(unwrap) => {
                if !unwrap.is_pos() {
                    println!("Only numeric values > 1 are permitted, yours is {}", unwrap.get_val());
                    input = console.take_num_input();
                    continue;
                }
                break *unwrap.get_val() as u32;
            }
            None => {
                println!("Your input cannot be resolved to numeric value");
                input = console.take_num_input();
                continue;
            }
        }
    }
}
