pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = numbers[0];
    println!("largest: {}", largest);

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("largest: {}", largest);
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
