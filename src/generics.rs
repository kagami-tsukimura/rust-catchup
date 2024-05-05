pub fn run() {
    println!("generics.rs is start!");
}

fn print_generics(s: &String) {
    println!("\nprint_generics: {}------", s);
    println!("{}", s);
    println!("Stack address of s is: {:p}", &s);
    println!("Heap address of s is: {:?}", &s.as_ptr());
    println!("Length of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("------print_generics: {}\n", s);
}
