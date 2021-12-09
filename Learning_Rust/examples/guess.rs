use std::io;
use rand::Rng;

fn main() {
    let num : u32 = rand::thread_rng().gen_range(1 .. 3);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Enter Number not string");

        let guess : u32 = guess.trim().parse()
    }
}
