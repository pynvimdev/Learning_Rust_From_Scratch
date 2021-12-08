use rand::Rng;
use std::io;

fn main() {
    let sec_num = rand::thread_rng().gen_range(1..10);
    println!("{}", sec_num);

    // Create main loop {

    loop {
        println!("Please enter your num: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Expected number");

        // Convert to num

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guesses {}", guess);
    }
}
