use std::io;

fn main() {
    let (s1, s2) = ("some", "string");
    let s = [s1, s2].concat();
    println!("{}", s);

    let mut age = String::new();

    println!("Enter your age");
    io::stdin().read_line(&mut age);

    let mut age: i32 = match age.trim().parse() {
        Ok(age) => age,
        Err(_) => -1,
    };

    if age < 18 {
        println!("Hello, child");
    } else if age > 45 && age < 81 {
        println!("Hello Bommer");
    } else {
        println!("Bye");
    }
}
