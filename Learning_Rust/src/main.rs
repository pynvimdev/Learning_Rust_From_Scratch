use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let sec_num = rand::thread_rng().gen_range(1..10);

    let mut guess = String::new();

    loop {
        print!("Enter Your guess: ");
            
        io::stdin()
            .read_line(&mut guess)
            .expect("Enter Num and ot string");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}", guess);

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too  low"),
            Ordering::Greater => println!("Too hight"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
