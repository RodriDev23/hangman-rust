use rand::Rng;
use std::io;

fn inital_state() {
    let stick_man = "
     O
    /|\\
    / \\
    ";

    println!(
        "Welcome to the hangman try to guess the word! {}",
        stick_man
    );
}

fn random_words() -> &'static str {
    let words = [
        "apple",
        "banana",
        "chocolate",
        "diamond",
        "elephant",
        "fireworks",
        "guitar",
        "hamburger",
        "iceberg",
        "jigsaw",
        "kangaroo",
        "lemonade",
        "magnolia",
        "notebook",
        "ostrich",
        "piano",
        "quartz",
        "raccoon",
        "sapphire",
        "tiger",
        "umbrella",
        "violin",
        "watermelon",
        "xylophone",
        "yacht",
        "zeppelin",
    ];

    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..words.len());

    words[random_index]
}

fn show_status(lifes: u8) {
    match lifes {
        1 => println!(" +---+\n |   |\n     |\n     |\n     |\n     |\n=========\n"),
        2 => println!(" +---+\n |   |\n O   |\n     |\n     |\n     |\n=========\n"),
        3 => println!(" +---+\n |   |\n O   |\n/|   |\n     |\n     |\n=========\n"),
        4 => println!(" +---+\n |   |\n O   |\n/|\\ |\n     |\n     |\n=========\n"),
        5 => println!(" +---+\n |   |\n O   |\n/|\\ |\nx    |\n     |\n=========\n"),
        _ => println!("No character founded"),
    }
}

// ... your show_status function ...

fn game_winner() {
    println!("Congrats you win the game!");
}

fn loose_game() {
    println!("You kill the man!");
}

fn main() {
    let mut lifes = 7; // Initialize with 7 lives
    let selected_word = random_words();
    let mut hidden_word: String = "_".repeat(selected_word.len());

    inital_state();
    println!("Guess the word: {}", hidden_word);

    while lifes > 0 {
        let mut user_input = String::new();
        println!("Enter a letter to guess: ");
        io::stdin().read_line(&mut user_input).expect("error 404");

        let guessed_char = user_input.trim().chars().next(); // Get the first character

        if let Some(c) = guessed_char {
            let mut found_match = false;
            for (i, char_in_word) in selected_word.chars().enumerate() {
                if c == char_in_word {
                    hidden_word.replace_range(i..i + 1, &c.to_string()); // Replace the underscore
                    found_match = true;
                }
            }

            if !found_match {
                lifes -= 1;
            }

            println!("Guess the word: {}", hidden_word);

            if hidden_word == selected_word {
                game_winner();
                break;
            }

            show_status(lifes);

            if lifes == 0 {
                loose_game();
            }
        }
    }
}

