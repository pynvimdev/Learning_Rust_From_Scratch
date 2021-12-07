fn convert_str_into_num(){
    let mut me = "me";
    let me : i32 = match me.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    loop {
        
    }
}


fn main() {
    convert_str_into_num();
}
