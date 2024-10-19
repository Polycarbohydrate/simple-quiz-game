use std::io;
use engine::storage::QuizFunctions;
use crate::engine::checks;
pub mod engine;
fn main()   {
    println!("===========================================================");
    println!("===== Welcome to this quiz! Press 'enter' to continue. ====");
    println!("===========================================================");
    let mut user_in = String::new();
    io::stdin().read_line(&mut user_in).expect("Could not read line.");
    q_one();
}

fn q_one() {
    println!("============================================================");
    let question_1 = QuizFunctions {question_text: String::from("Which of the following is the 3rd tallest building on Earth?"), f_answer1: String::from("Empire State Building (US, NY)"), f_answer2: String::from("Taipei 101 (TW, TW-TPE)"), f_answer3: String::from("Eiffel Tower (FR, PAR)"), f_answer4: String::from("Shanghai Tower (CN, SH)")};
    println!("{}", question_1.question_text);
    println!("A: {}", question_1.f_answer1);
    println!("B: {}", question_1.f_answer4);
    println!("C: {}", question_1.f_answer3);
    println!("D: {}", question_1.f_answer2);
    println!("============================================================");

    let mut answer_1 = String::new();
    io::stdin().read_line(&mut answer_1).expect("Could not read line.");
    let answer_1 = answer_1.trim().to_uppercase();
    checks::check_ansr1(answer_1);
}

fn q_two()  {
    println!("====================================");
    let question_2 = QuizFunctions  {question_text: String::from("How many centimeters are in a meter?"), f_answer4: String::from("100 cm"), f_answer2: String::from("1,000 cm"), f_answer3: String::from("10,000 cm"), f_answer1: String::from("10 cm")};
    println!("{}", question_2.question_text);
    println!("A: {}", question_2.f_answer1);
    println!("B: {}", question_2.f_answer2);
    println!("C: {}", question_2.f_answer3);
    println!("D: {}", question_2.f_answer4);
    println!("====================================");

    let mut answer_2 = String::new();
    io::stdin().read_line(&mut answer_2).expect("Could not readline.");
    let answer_2 = answer_2.trim().to_uppercase();
    checks::check_ansr2(answer_2);
}

fn q_three()    {
    println!("=================================");
    let question_3 = QuizFunctions  {question_text: String::from("What is the world's fastest bird?"), f_answer1: String::from("Bald Eagle (Haliaeetus leucocephalus)"), f_answer2: String::from("Hummingbird (Trochilidae)"), f_answer3: String::from("Parrot (Psittaciformes)"), f_answer4: String::from("Peregrine Falcon (Falco peregrinus)")};
    println!("{}", question_3.question_text);
    println!("A: {}", question_3.f_answer1);
    println!("B: {}", question_3.f_answer2);
    println!("C: {}", question_3.f_answer4);
    println!("D: {}", question_3.f_answer3);
    println!("=================================");

    let mut answer_3 = String::new();
    io::stdin().read_line(&mut answer_3).expect("Could not readline.");
    let answer_3 = answer_3.trim().to_uppercase();
    checks::check_ansr3(answer_3);
}

fn play_again() {
    let mut user_in = String::new();
    io::stdin().read_line(&mut user_in).expect("Could not read line.");
    let user_in = user_in.trim().to_uppercase();
    checks::again(user_in);
}