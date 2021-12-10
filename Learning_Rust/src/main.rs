use std::io;
use std::cmp;
use rand::Rng;

fn main() {
    println!("Enter Your guess");
    let sec_num = rand::thread_rng().gen_range(1..10);
    println!("{}", sec_num);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Enter Num and ot string");

    let guess : i32 = guess.trim().parse() {
        Ok(num) -> num,
        Err(_) -> continue,
    };
}
