use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::ops::AddAssign;

fn main() {
    println!("Hi! Let's play a game!");

    let mut games_total: u32 = 0;
    let mut games_won: u32 = 0;
    let mut games_lost: u32 = 0;

    loop {
        println!("Awaiting input...");
        let expectation = rand::thread_rng()
            .gen_range(1, 10);

        let mut input_buffer = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to obtain user input from stdin");

        if input_buffer.trim().eq_ignore_ascii_case("quit") {
            let win_ratio: f32 = (games_won as f32) / (games_total as f32) * 100f32;
            let win_lose_ratio: f32 = if games_lost == 0 { 100f32 }
                else { (games_won as f32) / (games_lost as f32) * 100f32 };
            println!("Quitting...");
            println!("Your stats are:\n\
                -> games played: {}\n\
                -> games won: {}\n\
                -> win ratio: {}%\n\
                -> win-lose-ratio: {}%", games_total, games_won, win_ratio, win_lose_ratio);
            break;
        }

        let converted: u32 = match input_buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input {} is not an int, try again", input_buffer.trim());
                continue;
            }
        };

        match converted.cmp(&expectation) {
            Ordering::Less => {
                println!("A bit too low: yours {}, expected {}", converted, expectation);
                games_total.add_assign(1);
                games_lost.add_assign(1);
            },
            Ordering::Equal => {
                println!("Whoa! Who's the lucky mack?!");
                games_total.add_assign(1);
                games_won.add_assign(1);
            },
            Ordering::Greater => {
                println!("Now this went too far: yours {}, expected {}", converted, expectation);
                games_total.add_assign(1);
                games_lost.add_assign(1);
            }
        }

    }

}