use std::io;
use std::io::Write; 

/// This enum represents all valid inputs for game
pub enum Choice {
    Left,
    Right,
    Invalid,
    Quit,
}

pub trait UserChoice {
    fn execute(&self, roll: u8, lives: i8) -> i8;
}

pub struct GoLeft;
impl UserChoice for GoLeft {
    fn execute(&self, roll: u8, lives: i8) -> i8 {
        if roll < 3 {
            println!("You go left and find a treasure chest and gain one life!");
            return lives + 1;
        }
        if roll <= 7 {
            println!("You go left and find a dead end.");
            return lives;
        }

        println!("You go left and meet a terrible monster and lose a life!");
        return lives - 1;
    }
}

pub struct GoRight;
impl UserChoice for GoRight {
    fn execute(&self, roll: u8, lives: i8) -> i8 {
        if roll > 7 {
            println!("You go right and find a treasure chest and gain one life!");
            return lives + 1;
        }
        if roll >= 3 {
            println!("You go right and find a dead end.");
            return lives;
        }

        println!("You go right and meet a terrible monster and lose a life!");
        return lives - 1;
    }
}

/// This function evaluates the user input and returns the correct choice enum
pub fn get_user_choice() -> Choice {
        println!("~~Do you go left or right? (L/R/Q)");
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the `>` appears before the program waits for user input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        //This matches the choice after formatting and taking a slice
        match choice.trim().to_lowercase().as_str() {
            "l" => Choice::Left,
            "r" => Choice::Right,
            "q" => Choice::Quit,
            _ => Choice::Invalid,
        }
}