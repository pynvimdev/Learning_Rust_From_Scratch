/*
- Stack Memory in rust
- Memory is allaocated and removed quickly
- But in heap Memory its slow but can be changed or modifeid

Stack Memory function(allaocateStackMemory)
/*----------------------------------------------------------------------------*/
| Address | Name | Value
|   0   |  x  | 10
|   1   |  y  | 20

Heap Memory function(allaocateHeapMemory)
/*----------------------------------------------------------------------------*/
| Address | Name | Value
|   0   |  x  | 10
|   1   |  y  | 20
*/

fn allaocate_stack_memory() {
    let _z = 10;
    let _y = 20;
}

fn allaocate_heap_memory() {
    let _x = Box::new(10);
    let _y = Box::new(20);
}
fn mem() {
    allaocate_heap_memory();
    allaocate_stack_memory();
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn Learning_Rust_By_Example() {
    // this alos works
    println!("{} num is stringified", 31);
    // Also needs a positional argument
    println!("{0}", "Haryy is here");
    // RustLang is the best
    println!("{sub}", sub = "This is RustLang");
    // Special formatting
    println!("{} of {:b} blah formatting , <-- is can use function", 1, 2);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 3);
    // Padding by 0
    println!("{number:0>width$}", number = 1, width = 3);
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 13;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}

fn main() {
    mem();
    Learning_Rust_By_Example();
}
