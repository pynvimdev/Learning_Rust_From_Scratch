use rand::Rng;

fn main() {
    println!("Hello World");
    let sec_num = rand::thread_rng().gen_range(1 .. 10);
    println!("{}",sec_num);
}
