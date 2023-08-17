//this is how you call a crate's methods
use rand::Rng;

mod user_choice;
use user_choice::{Choice, UserChoice, GoLeft, GoRight, get_user_choice};

fn execute_choice<T: UserChoice>(user_choice: T, roll: u8, lives: i8) -> i8 {
    return user_choice.execute(roll, lives);
}

fn main() {
    println!("#### Welcome to the dungeon! ####\n#### This is a simple text based game ####\n#### follow the prompts to play! ####");
    
    //instantiate a rand thread_rng instance
    let mut rng = rand::thread_rng();

    //track lives using u8
    let mut lives :i8 = 1;

    //this loop repeats every time the player inputs something
    loop {
        let choice = get_user_choice();
        let roll: u8 = rng.gen_range(1..10);

        // This is where the enum is evaluted and acted on.
        match choice {
            Choice::Left => {
                lives = execute_choice(GoLeft{}, roll, lives);
                if lives <= 0 {
                    print!("You are out of lives. Good bye!");
                    break;
                }
            }
            Choice::Right => {
                lives = execute_choice(GoRight{}, roll, lives);
                if lives <= 0 {
                    print!("You are out of lives. Good bye!");
                    break;
                }
            }
            Choice::Invalid => {
                println!("Invalid choice! Please enter L, R or Q.");
                continue;
            }

            //This could also be _ => as it represents all other possible choices
            Choice::Quit => {
                println!("You've chosen to quit the game. Goodbye!");
                break;
            }
        }
        println!("Lives remaining: {lives}");
    }
}
