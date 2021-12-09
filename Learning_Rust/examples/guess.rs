fn main(){
    sum(1 , 2);
    hello();
    sec_sum(2);
}

pub fn hello() -> String {
    ("Hello World").to_string()
}

fn sum(a : i8 , b : i8){
    println!("Sum is {}" ,a + b);
}

fn sec_sum(a: i32){
   a + 1; 
}
