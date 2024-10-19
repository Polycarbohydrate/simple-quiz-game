use std::{io, process::exit};
use crate::{main, play_again, q_one, q_three, q_two};

pub fn check_ansr1(ansr: String) {
    let ansr: &str = ansr.as_str();
    match ansr  {
        "B" =>    {
            println!("==================================================");
            println!("Correct! Press 'enter' to go to the next question.");
            println!("==================================================");
            let mut user_in = String::new();
            io::stdin().read_line(&mut user_in).expect("Could not read line.");
            q_two();
            
        }
        _ =>    {
            println!("=======================================");
            println!("Wrong answer, press enter to try again.");
            println!("=======================================");
            let mut user_in = String::new();
            io::stdin().read_line(&mut user_in).expect("Could not read line.");
            q_one();
        }
    }
}

pub fn check_ansr2(ansr: String)  {
    let ansr = ansr.as_str();
    match ansr  {
        "D" =>  {
            println!("==================================================");
            println!("Correct! Press 'enter' to go to the next question.");
            println!("==================================================");
            let mut user_in = String::new();
            io::stdin().read_line(&mut user_in).expect("Could not read line.");
            q_three();
        }
        _ =>    {
            println!("======================================");
            println!("Wrong answer, press enter to continue.");
            println!("======================================");
            let mut user_in = String::new();
            io::stdin().read_line(&mut user_in).expect("Could not read line.");
            q_two();
        }
    }
}

pub fn check_ansr3(ansr: String) {
    let ansr: &str = ansr.as_str();
    match ansr  {
        "C" =>    {
            println!("==========================================");
            println!("Correct! Would you like to play again? Y/n");
            println!("==========================================");
            play_again();
            
        }
        _ =>    {
            println!("=======================================");
            println!("Wrong answer, press enter to try again.");
            println!("=======================================");
            let mut user_in = String::new();
            io::stdin().read_line(&mut user_in).expect("Could not read line.");
            q_three();
        }
    }
}

pub fn again(ansr: String) {
    let ansr= ansr.as_str();
    match ansr  {
        "Y" =>  {
            main();
        }
        "N" =>  {
            exit(0);
        }
        _ =>    {
            println!("=================================================================================");
            println!("Type either 'y' or 'n' to exit. Or press 'ctrl' + 'c' to force close the program.");
            println!("=================================================================================");
            play_again();
        }
    }
}