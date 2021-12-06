use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    let mut gues = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // We can also do this in oneline
    // io::stdin().read_line(&mut gues).expect("Expected Input");
    io::stdin()
        .read_line(&mut gues)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
    println!("You guessed: {}", gues);
}
