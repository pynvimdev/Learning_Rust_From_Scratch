use std::io;

fn main() {
    println!("Enter your Number of mebers in teams: ");
    let team_size_next;
    let mut team_size = String::new();

    io::stdin().read_line(&mut team_size).expect("Err");

    let team_size: i32 = match team_size.trim().parse() {
        Ok(num) => num,
        Err(_) => return (),
    };
    println!("Number of members are {}", team_size);

    if team_size < 5 {
        team_size_next = "Small";
    } else if team_size < 10 {
        team_size_next = "Meduim";
    } else {
        team_size_next = "Big";
    }

    println!("Current team size is: {}", team_size_next);
}


fn vec(){
    let mut a = Vec::new();
}
