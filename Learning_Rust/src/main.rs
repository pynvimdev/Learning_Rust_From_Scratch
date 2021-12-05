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
fn Learninf_Rust_BU_Examples(){
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
    struct UnPrintable (i32);
    #[derive(Debug)]
    struct Printable(i32);
    struct Structure {
        hari : i32
    }
}

fn main() {
    mem();
    Learninf_Rust_BU_Examples();
}
